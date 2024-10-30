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
    pub fn parse(path: &'static str, route: &'static str) -> Route {
        Self {
            path: String::from(path),
            route: String::from(route),
            slugs: parse_slug(path, route),
            params: parse_params(path.to_string()),
            routes: path.split("/").map(String::from).collect(),
        }
    }
}

pub fn parse_params(query: String) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    let mut queries = query.split("?");
    for query in queries.nth(1).unwrap_or("").split("&") {
        let key = query.split('=').nth(0).unwrap_or("").trim().to_string();
        let value = query.split('=').nth(1).unwrap_or("").trim().to_string();
        params.insert(key, value);
    }
    params
}

pub fn parse_slug(path: &'static str, pattern: &'static str) -> HashMap<String, String> {
    let mut slugs = HashMap::new();
    for (i, route) in pattern.split("/").enumerate() {
        if route.starts_with("<") && route.ends_with(">") {
            let slug = route.trim_start_matches("<").trim_end_matches(">");
            let value = path.split("/").nth(i).unwrap_or("").trim().to_string();
            slugs.insert(slug.to_string(), value);
        }
    }
    slugs
}

pub fn match_dynamic(path: String, route: String) -> bool {
    let path_routes = path.split("/").collect::<Vec<&str>>();
    let route_routes = route.split("/").collect::<Vec<&str>>();

    if path_routes.len() != route_routes.len() {
        return false;
    }

    for (i, route_route) in route_routes.iter().enumerate() {
        if route_route.starts_with("<") && route_route.ends_with(">") {
            continue;
        } else if *route_route != path_routes[i] {
            return false;
        }
    }

    true
}
