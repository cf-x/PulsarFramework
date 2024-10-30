use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use coloredpp::Colorize;

type Closure = Box<dyn Fn(&Route) -> (String, usize) + Send + 'static>;
pub struct Request {
    pub route: Route,
    pub method: Closure,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub path: String,
    pub route: String,
    pub routes: Vec<String>,
    pub slugs: HashMap<String, String>,
}

impl Route {
    pub fn parse(path: &str, route: &str) -> Route {
        Self {
            path: String::from(path),
            route: String::from(route),
            slugs: HashMap::new(),
            routes: Vec::new(),
        }
    }
    pub fn stringify(&self) -> String {
        // converts Route to the String
        String::from(&self.route)
    }
}

pub struct Pulse {
    port: usize,
    routes: Vec<Route>,
    requests: Vec<Box<Request>>,
    current_method: String,
    current_path: String,
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

    /*
        Server
    */
    fn client(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]).to_string();
                let method = request.split_whitespace().next().unwrap();
                let path = request.split_whitespace().nth(1).unwrap_or("");
                self.current_method = method.trim().to_string();
                self.current_path = path.trim().to_string();
                let mut is_404 = true;
                for req in self.requests.iter() {
                    if req.route.route == self.current_path {
                        is_404 = false;
                        let (res, _code) = (req.method)(&req.route);
                        stream.write_all(res.as_bytes()).unwrap();
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

    /*
        HTTP Methods
    */
    pub fn get<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Route) -> (String, usize) + Send + 'static,
    {
        let route = Route {
            path: String::new(),
            route: String::from(route),
            slugs: HashMap::new(),
            routes: Vec::new(),
        };
        self.routes.push(route.clone());
        self.requests.push(Box::new(Request {
            route,
            method: Box::new(closure),
        }));
    }
    // post, put, delete, patch
}