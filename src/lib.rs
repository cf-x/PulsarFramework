use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use coloredpp::Colorize;

#[derive(Debug)]
pub enum ServerResult {
    String(&'static str),
}

type Closure = dyn FnOnce(&Route) -> (String, usize) + 'static;
pub struct Route {
    pub path: String,
    pub route: String,
    pub routes: Vec<String>,
    pub slugs: HashMap<String, String>,
}

impl Route {
    pub fn parse(path: &str, route: &str) -> Route {
        // get slugs and routes from `route`
        // `/users/34`, `/users/<id>`
        // slugs: HashMap::from(&["id", "34"])
        // routes: vec!["users", "34"]
        // route: "/users/<id>"
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
    current_method: String,
    current_path: String,
}
impl Pulse {
    pub fn new(port: usize) -> Pulse {
        Pulse {
            port,
            routes: Vec::new(),
            current_method: String::new(),
            current_path: String::new(),
        }
    }
    pub fn launch(&mut self) {
        let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("failed to bind to address");
        println!("{} {}{}", "server launched on:".yellow(), "http://127.0.0.1:".green(), self.port.green());

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
                for route in &self.routes {
                    if route.path == self.current_path {
                        is_404 = false;
                        let response = format!("{}: {}", route.path, "Hello, World!");
                        stream.write_all(response.as_bytes()).unwrap();
                        break;
                    }
                }

                if !is_404 {
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
        Methods
    */

    /// `get` method
    pub fn get<F>(&mut self, route: &'static str, _closure: F)
    where
        F: FnOnce(&Route) -> (String, usize) + 'static,
    {
        let route = Route::parse("", route);
        self.routes.push(route);
    }
    // pub fn post<F>(&self, route: &'static str, closure: F)
    // where
    //     F: Fn(Option<Route>) -> (ServerResult, String) + 'static,
    // {}
    // pub fn put<F>(&self, route: &'static str, closure: F)
    // where
    //     F: Fn(Option<Route>) -> (ServerResult, String) + 'static,
    // {}
    // pub fn delete<F>(&self, route: &'static str, closure: F)
    // where
    //     F: Fn(Option<Route>) -> (ServerResult, String) + 'static,
    // {}
    // pub fn patch<F>(&self, route: &'static str, closure: F)
    // where
    //     F: Fn(Option<Route>) -> (ServerResult, String) + 'static,
    // {}
}