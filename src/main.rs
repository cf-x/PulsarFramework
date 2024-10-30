/*
Security TODOs
TODO: OAuth, JWT, MFA, RBAC mechanisms
TODO: Session expiration and invalidation
TODO: HTTPS and data encryption
TODO: file type and size validation
TODO: ORM support
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
    server.load_env(".env");

    server.get("/", |req, res| {
        res.body("hello world!");
        res.clone()
    });
    server.get("/user/<name>", |req, res| {
        let body = format!("hello {}!", req.route.slugs.get("name").unwrap().as_str());
        res.body(body);
        res.clone()
    });
    server.launch();
}
