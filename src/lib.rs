use std::collections::HashMap;

#[derive(Debug)]
pub enum Result {
    String(&'static str),
}

pub struct Route {
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
        Self { route: String::from(route), slugs: HashMap::new(), routes: Vec::new() }
    }
    pub fn stringify(&self) -> String {
        // converts Route to the String
        String::from(&self.route)
    }
}

pub struct Pulse {}
impl Pulse {
    pub fn launch(port: usize) -> Pulse {
        // start listening to the localhost:port
        // initialize the system
        // log necessary info
        Pulse {}
    }
    /// `get` method
    pub fn get<F>(route: &'static str, closure: F)
    where
        F: Fn(Option<Route>) -> (Result, usize) + 'static,
    {
        println!("route: {}", route);

        let (result, status_code) = closure();

        println!("closure result: {:?}, status code: {}", result, status_code);
    }
    pub fn post<F>(route: &'static str, closure: F)
    where
        F: Fn(Option<Route>) -> (Result, String) + 'static,
    {}
    pub fn put<F>(route: &'static str, closure: F)
    where
        F: Fn(Option<Route>) -> (Result, String) + 'static,
    {}
    pub fn delete<F>(route: &'static str, closure: F)
    where
        F: Fn(Option<Route>) -> (Result, String) + 'static,
    {}
    pub fn patch<F>(route: &'static str, closure: F)
    where
        F: Fn(Option<Route>) -> (Result, String) + 'static,
    {}
}