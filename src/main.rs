/*
Security TODOs
after adding SQL TODO: XSS, SQL Injection, CSRF, prevention
TODO: OAuth, JWT, MFA, RBAC mechanisms
TODO: Cookies (HttpOnly, Secure, SameSite flags)
TODO: Session expiration and invalidation
TODO: HTTPS and data encryption
TODO: .env secret loading
TODO: file type and size validation
TODO: update Routing
TODO: middleware (requests, responses, logging, authentication, E handling...)
TODO: ORM support
TODO: RESTful and GraphQL serialization and versioning
TODO: Session storage and expiration management
TODO: environments: dev, prod, test
TODO: caching, async processing
TODO: test suite library
TODO: i18n and l10n
TODO: filesystem management
*/

use pulsar_web::Pulse;

fn main() {
    let mut server = Pulse::new(3000);
    server.get("/", |req, res| {
        res.body("hello world!");
        res.clone()
    });
    server.get("/user/<name>", |req, res| {
        res.body(format!("hello {}!", req.route.slugs.get("name").unwrap_or(&String::from("anon"))).as_str());
        res.clone()
    });
    server.get("/user", |req, res| {
        res.body("hello user");
        res.clone()
    });
    server.launch();
}
