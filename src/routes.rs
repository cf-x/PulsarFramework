use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Route {
    pub path: String,
    pub route: String,
    pub routes: Vec<String>,
    pub slugs: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

impl Route {
    pub fn parse(path: &str, route: &str) -> Route {
        // path: full path (127.0.0.1:3000)
        // route: url route (/)
        // slugs: extracted slugs from the path
        // (/user/32, /user/<id> -> ["id", "32"])
        // params: query params (/?q="hi" -> ["q", "hi"])
        // routes: ["user", "32"]
        Self {
            path: String::from(path),
            route: String::from(route),
            slugs: HashMap::new(),
            params: HashMap::new(),
            routes: Vec::new(),
        }
    }
    pub fn stringify(&self) -> String {
        // converts Route to the String
        String::from(&self.route)
    }
}
