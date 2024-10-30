use std::collections::HashMap;
use crate::{Pulse, Request, Route};

#[derive(Debug)]
pub struct Req {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub query: HashMap<String, String>,
    pub route: Route,
}

#[derive(Debug, Clone)]
pub struct Res {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Res {
    pub fn status(&mut self, code: u16) {
        self.status = code;
    }
    pub fn body(&mut self, body: &str) {
        self.body = body.to_string();
    }
    pub fn header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
    pub fn json(&mut self, json: &str) {
        self.header("Content-Type", "application/json");
        self.body(json);
    }
}

impl Pulse {
    pub fn get<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure);
    }

    pub fn post<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure);
    }

    pub fn put<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure);
    }

    pub fn delete<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure);
    }

    pub fn patch<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure);
    }

    pub fn method<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        let route = Route {
            path: String::new(),
            route: String::from(route),
            slugs: HashMap::new(),
            params: HashMap::new(),
            routes: Vec::new(),
        };
        self.routes.push(route.clone());
        self.requests.push(Box::new(Request {
            route,
            method: Box::new(closure),
        }));
    }
}