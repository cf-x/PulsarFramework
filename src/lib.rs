mod http;
mod routes;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use coloredpp::Colorize;
use crate::http::{Req, Res};
use crate::routes::{match_dynamic, parse_params, Route};

type Closure = Box<dyn Fn(&Req, &mut Res) -> Res + Send + 'static>;
pub struct Request {
    pub route: Route,
    pub method: Closure,
    pub http: String,
}

pub struct Pulse {
    pub port: usize,
    pub method: String,
    pub path: String,
    pub url: String,
    pub user_agent: String,
    pub content_type: String,
    pub content_length: usize,
    routes: Vec<Route>,
    requests: Vec<Box<Request>>,
}
impl Pulse {
    pub fn new(port: usize) -> Pulse {
        println!("{} {}{}/", "server launched on:".yellow(), "http://127.0.0.1:".green(), port.green());
        Pulse {
            port,
            routes: Vec::new(),
            requests: Vec::new(),
            method: String::new(),
            path: String::new(),
            url: String::new(),
            user_agent: String::new(),
            content_type: String::new(),
            content_length: 0,
        }
    }
    pub fn launch(&mut self) {
        let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("failed to bind to address");
        for _ in listener.incoming() {
            match listener.accept() {
                Ok((stream, _)) => self.client(stream),
                Err(_) => panic!("{}", "failed to accept".red()),
            }
        }
    }

    fn client(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]).to_string();
                let mut body = String::new();
                let mut content_length = 0;
                let mut headers = HashMap::new();

                for line in request.split('\n') {
                    if line.starts_with("GET") || line.starts_with("POST") || line.starts_with("PUT") || line.starts_with("DELETE") || line.starts_with("PATCH") {
                        self.method = String::from(line.split_whitespace().nth(0).unwrap_or(""));
                        self.path = String::from(line.split_whitespace().nth(1).unwrap_or(""));
                    }
                    if line.starts_with("User-Agent:") {
                        self.user_agent = String::from(line.split("User-Agent:").nth(1).unwrap_or("").trim());
                        headers.insert("User-Agent".to_string(), self.user_agent.clone());
                    }
                    if line.starts_with("Content-Type:") {
                        self.content_type = String::from(line.split("Content-Type:").nth(1).unwrap_or("").trim());
                        headers.insert("Content-Type".to_string(), self.content_type.clone());
                    }
                    if line.starts_with("Content-Length:") {
                        content_length = usize::from_str_radix(line.split("Content-Length:").nth(1).unwrap_or("0").trim(), 10).unwrap_or(0);
                    }
                    if line.starts_with("HOST:") {
                        self.url = String::from(line.split_whitespace().nth(1).unwrap_or("").trim());
                        headers.insert("HOST".to_string(), self.url.clone());
                    }
                }

                if content_length > 0 {
                    let mut body_buffer = vec![0; content_length];
                    match stream.read_exact(&mut body_buffer) {
                        Ok(_) => {
                            body = String::from_utf8_lossy(&body_buffer).to_string();
                        }
                        Err(e) => {
                            eprintln!("Failed to read body: {}", e);
                        }
                    }
                }

                let mut is_404 = true;
                let path = self.path.clone();
                for req in self.requests.iter_mut() {
                    let r = req.route.route.clone();
                    if (r == path || match_dynamic(path.clone(), r.clone())) && req.http == self.method {
                        if r.contains("<") {
                            let route_segments = r.split("/").collect::<Vec<&str>>();
                            let path_segments = path.split("/").collect::<Vec<&str>>();
                            let mut slugs = HashMap::new();

                            for (j, segment) in route_segments.iter().enumerate() {
                                if segment.starts_with("<") && segment.ends_with(">") {
                                    let param_name = &segment[1..segment.len() - 1]; // Get the parameter name
                                    slugs.insert(param_name.to_string(), path_segments[j].to_string());
                                }
                            }
                            is_404 = false;
                            req.route.slugs = slugs;
                        }

                        let pass_req = Req {
                            method: self.method.clone(),
                            url: self.url.clone(),
                            body,
                            query: parse_params(path),
                            headers,
                            route: req.route.clone(),
                        };
                        let mut pass_res = Res {
                            status: 200,
                            body: String::new(),
                            headers: HashMap::new(),
                        };

                        let res = (req.method)(&pass_req, &mut pass_res);
                        stream.write_all(res.body.as_bytes()).unwrap();
                        break;
                    }
                }

                if is_404 {
                    let response = "404 Not Found";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                eprintln!("{} {}", "failed to read from connection:".red().bold(), e.red());
            }
        }
    }
}
