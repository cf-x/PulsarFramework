use std::collections::HashMap;
use crate::{Pulse, Request, Route};
use crate::env::load_file;

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
    pub fn body<B: Into<String>>(&mut self, body: B) {
        self.body = body.into();
    }
    pub fn header(&mut self, key: &'static str, value: &'static str) {
        if key == "Set-Cookie" {
            let mut cookie_value = value.to_string();
            if !cookie_value.contains("Secure") {
                cookie_value.push_str("; Secure");
            }
            if !cookie_value.contains("HttpOnly") {
                cookie_value.push_str("; HttpOnly");
            }
            if !cookie_value.contains("SameSite") {
                cookie_value.push_str("; SameSite=Lax");
            }
            if !cookie_value.contains("Path") {
                cookie_value.push_str("; Path=/");
            }
            self.headers.insert(key.to_string(), cookie_value);
        } else {
            self.headers.insert(key.to_string(), value.to_string());
        }
    }

    pub fn json(&mut self, json: &'static str) {
        self.header("Content-Type", "application/json");
        self.body(json);
    }
    pub fn html(&mut self, html: &'static str) {
        self.header("Content-Type", "text/html");
        self.body(html);
    }
    pub fn html_load(&mut self, html: &'static str) {
        self.header("Content-Type", "text/html");
        let contents = load_file(html).unwrap();
        self.body(contents);
    }
    pub fn file(&mut self, url: &'static str) {
        let file = load_file(url).unwrap();
        self.header("Content-Disposition", "inline");
        self.body(file);
    }
    pub fn download(&mut self, url: &'static str) {
        let file = load_file(url).unwrap();
        self.header("Content-Disposition", "attachment; filename=\"downloaded_file\"");
        self.body(file);
    }
}

impl Pulse {
    pub fn get<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure, "GET");
    }

    pub fn post<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure, "POST");
    }

    pub fn put<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure, "PUT");
    }

    pub fn delete<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure, "DELETE");
    }

    pub fn patch<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure, "PATCH");
    }
    pub fn all<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) -> Res + Send + 'static,
    {
        Self::method(self, route, closure, "all");
    }

    pub fn method<F>(&mut self, route: &'static str, closure: F, method: &'static str)
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
            http: method.to_string(),
        }));
    }
}