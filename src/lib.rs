pub mod http;
pub mod routes;
pub mod env;
pub mod session;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use coloredpp::Colorize;
use crate::http::{parse_cookies, Req, Res};
use crate::routes::{match_dynamic, parse_params, Route};
pub use async_std::task;
use crate::session::SessionStorage;

/// wraps `server` function in the asynchronous `main` function.
///
/// ### Example
///
/// ```text
///async fn server(mut server: Pulse) {...}
///pulsar!(server);
/// ```
#[macro_export]
macro_rules! pulsar {
    ($server:ident) => {
        fn main() {
            let mut pulsar = Pulse::new();
            $crate::task::block_on($server(pulsar));
        }
    };
}

/// closure argument type for http method
type Closure = Box<dyn Fn(&Req, &mut Res)>;

/// type of `req` in the http method closure argument
pub struct Request {
    pub route: Route,
    pub method: Closure,
    pub http: String,
}

pub struct Pulse {
    pub port: u16,
    pub method: String,
    pub path: String,
    pub url: String,
    pub user_agent: String,
    pub content_type: String,
    pub content_length: usize,
    pub secrets: HashMap<String, String>,
    pub is_https: bool,
    routes: Vec<Route>,
    requests: Vec<Box<Request>>,
}

impl Pulse {
    pub fn new() -> Pulse {
        Pulse {
            // default port: 3000
            port: 3000,
            routes: Vec::new(),
            requests: Vec::new(),
            method: String::new(),
            path: String::new(),
            url: String::new(),
            user_agent: String::new(),
            content_type: String::new(),
            secrets: HashMap::new(),
            is_https: false,
            content_length: 0,
        }
    }

    /// launch the server on the localhost argument port
    pub async fn launch(&mut self, port: u16) {
        println!("{} {}{}/", "server launched on:".yellow(), "http://127.0.0.1:".green(), port.green());
        self.port = port;
        self.launch_http().await;
    }

    async fn launch_http(&mut self) {
        let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("failed to bind to address");

        for _ in listener.incoming() {
            match listener.accept() {
                Ok((stream, _)) => self.client(stream, None).await,
                Err(_) => panic!("{}", "failed to accept".red()),
            }
        }
    }

    async fn client(&mut self, mut stream: TcpStream, buffer: Option<&[u8; 1024]>) {
        let mut buffer = if buffer.is_some() { buffer.unwrap().clone() } else { [0; 1024] };

        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]).to_string();
                let mut body = String::new();
                let mut content_length = 0;
                let mut headers = HashMap::new();

                // parse data from the request
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

                // is unhandled route
                let mut is_404 = true;

                let path = self.path.clone();
                // match the handler routes and application route
                for req in self.requests.iter_mut() {
                    let r = req.route.route.clone();
                    if (r == path || match_dynamic(path.clone(), r.clone()))
                        && (req.http == self.method || req.http == "all") {
                        // set 404 to false when matching the route to avoid 404 error
                        is_404 = false;
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
                            req.route.slugs = slugs;
                        }

                        // passed `req` argument
                        let pass_req = Req {
                            method: self.method.clone(),
                            url: self.url.clone(),
                            body,
                            query: parse_params(path),
                            cookies: parse_cookies(headers.clone()),
                            headers,
                            route: req.route.clone(),
                        };
                        // passed `res` argument
                        let mut res = Res {
                            status: 200,
                            body: String::new(),
                            headers: HashMap::new(),
                            session: SessionStorage::new(),
                        };

                        // execute the handler body
                        (req.method)(&pass_req, &mut res);

                        // write the response body
                        let mut response = format!("HTTP/1.1 {} OK\r\n", res.status);

                        res.headers.iter().for_each(|(k, v)| response.push_str(&format!("{}: {}\r\n", k, v)));
                        response.push_str("\r\n");
                        response.push_str(&res.body);
                        stream.write_all(response.as_bytes()).unwrap();
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
