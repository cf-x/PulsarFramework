mod http;
mod routes;

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use coloredpp::Colorize;
use crate::http::{Req, Res};
use crate::routes::Route;

type Closure = Box<dyn Fn(&Req, &mut Res) -> Res + Send + 'static>;
pub struct Request {
    pub route: Route,
    pub method: Closure,
}

pub struct Pulse {
    pub port: usize,
    pub current_method: String,
    pub current_path: String,
    routes: Vec<Route>,
    requests: Vec<Box<Request>>,
}
impl Pulse {
    pub fn new(port: usize) -> Pulse {
        println!("{} {}{}", "server launched on:".yellow(), "http://127.0.0.1:".green(), port.green());
        Pulse {
            port,
            routes: Vec::new(),
            requests: Vec::new(),
            current_method: String::new(),
            current_path: String::new(),
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
                let method = request.split_whitespace().next().unwrap();
                let path = request.split_whitespace().nth(1).unwrap_or("");
                let url = request.split_whitespace().nth(4).unwrap_or("");
                let _user_agent = request.split("\n")
                    .nth(2).unwrap_or("")
                    .split("User-Agent: ")
                    .nth(1).unwrap_or("").to_string();
                self.current_method = method.trim().to_string();
                self.current_path = path.trim().to_string();
                let mut is_404 = true;

                for req in self.requests.iter() {
                    if req.route.route == self.current_path {
                        is_404 = false;
                        let pass_req = Req {
                            method: method.to_string(),
                            url: url.to_string(),
                            body: String::new(),
                            query: HashMap::new(),
                            headers: HashMap::new(),
                            route: req.route.clone(),
                        };
                        let mut pass_res = Res {
                            status: 200,
                            body: String::from("hi"),
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
