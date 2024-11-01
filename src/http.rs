use std::collections::HashMap;
use coloredpp::Colorize;
use crate::{Pulse, Request, Route};
use crate::env::load_file;
use crate::session::{SessionStorage};

#[derive(Debug)]
pub struct Req {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: String,
    pub query: HashMap<String, String>,
    pub route: Route,
}

pub fn parse_cookies(headers: HashMap<String, String>) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    if let Some(cookie_string) = headers.get("Cookie") {
        for cookie in cookie_string.split(';') {
            let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                let value = parts[1].to_string();
                cookies.insert(key, value);
            }
        }
    }

    cookies
}

#[derive(Debug, Clone)]
pub struct Res {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub session: SessionStorage,
}

impl Res {
    pub fn status(&mut self, code: u16) {
        self.status = code;
    }
    pub fn data<B: Into<String>>(&mut self, body: B) {
        self.body = body.into();
    }
    pub fn header<K, V>(&mut self, key: K, value: V)
    where
        K: AsRef<str> + ToString,
        V: AsRef<str> + ToString,
    {
        if key.as_ref() == "Set-Cookie" {
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
            if !cookie_value.contains("Max-Age") {
                cookie_value.push_str("; Max-Age=86400");
            }
            if !cookie_value.contains("X-Content-Type-Options") {
                cookie_value.push_str("; nosniff");
            }
            self.headers.insert(key.to_string(), cookie_value.to_string());
        } else {
            self.headers.insert(key.to_string(), value.to_string());
        }
    }
    pub fn format(&mut self, format: &'static str) {
        match format {
            "json" |
            "javascript" |
            "xml" |
            "octet-stream" |
            "x-www-form-urlencoded" |
            "pdf" |
            "zip" |
            "vnd.api+json" |
            "vnd.ms-excel" |
            "vnd.openxmlformats-officedocument.spreadsheetml.sheet" |
            "vnd.ms-powerpoint" |
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
                let value = format!("application/{}", format);
                self.header("Content-Type", &value);
            }
            "html" | "css" | "text" | "plain" => {
                let value = format!("text/{}", format);
                self.header("Content-Type", &value);
            }
            "jpeg" | "png" | "gif" | "svg+xml" | "webp" => {
                let value = format!("image/{}", format);
                self.header("Content-Type", &value);
            }
            "mpeg" | "wav" => {
                let value = format!("audio/{}", format);
                self.header("Content-Type", &value);
            }
            "mp4" | "x-msvideo" | "x-matroska" | "ogg" => {
                let value = format!("video/{}", format);
                self.header("Content-Type", &value);
            }
            "form-data" | "alternative" | "mixed" => {
                let value = format!("multipart/{}", format);
                self.header("Content-Type", &value);
            }
            "woff" | "woff2" | "ttf" => {
                let value = format!("font/{}", format);
                self.header("Content-Type", &value);
            }
            "file" => {
                self.header("Content-Disposition", "inline");
                self.header("Content-Type", "application/x-www-form-urlencoded");
            }
            "download" => {
                self.header("Content-Disposition", "attachment; filename=\"downloaded_file\"");
                self.header("Content-Type", "application/x-www-form-urlencoded");
            }
            _ => {
                eprintln!("{} {}", "Unsupported format:".red(), format.yellow());
            }
        }
    }
    pub fn json(&mut self, json: &'static str) {
        self.header("Content-Type", "application/json");
        self.data(json);
    }
    pub fn html(&mut self, html: &'static str) {
        self.header("Content-Type", "text/html");
        self.data(html);
    }
    pub fn html_load(&mut self, html: &'static str) {
        self.header("Content-Type", "text/html");
        let contents = load_file(html).unwrap();
        self.data(contents);
    }
    pub fn file(&mut self, url: &'static str) {
        let file = load_file(url).unwrap();
        self.header("Content-Disposition", "inline");
        self.data(file);
    }
    pub fn download(&mut self, url: &'static str) {
        let file = load_file(url).unwrap();
        self.header("Content-Disposition", "attachment; filename=\"downloaded_file\"");
        self.data(file);
    }
}

impl Pulse {
    pub async fn get<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) + 'static,
    {
        Self::method::<F>(self, route, closure, "GET");
    }

    pub async fn post<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) + 'static,
    {
        Self::method::<F>(self, route, closure, "POST");
    }

    pub async fn put<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) + 'static,
    {
        Self::method::<F>(self, route, closure, "PUT");
    }

    pub async fn delete<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) + 'static,
    {
        Self::method::<F>(self, route, closure, "DELETE");
    }

    pub async fn patch<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) + 'static,
    {
        Self::method::<F>(self, route, closure, "PATCH");
    }

    pub async fn all<F>(&mut self, route: &'static str, closure: F)
    where
        F: Fn(&Req, &mut Res) + 'static,
    {
        Self::method::<F>(self, route, closure, "all");
    }

    pub fn method<F>(&mut self, route: &'static str, closure: F, method: &'static str)
    where
        F: Fn(&Req, &mut Res) + 'static,
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
