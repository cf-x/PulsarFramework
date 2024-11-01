# Todo

- error handler

```rust 
async fn server(mut server: Pulse) {
    // handle any error
    server.catch(|error: String| {}).await;
    // handle route errors
    server.route(404, |req, res| {}).await;
}
```

- new req API

```rust 
async fn server(mut server: Pulse) {
    server.get("/", |req, res| {
        req.cookies.get("MyCookie");
        req.url;
        req.url.params;
        req.url.query;
        // req.url...

        res.data("hello world!");
    }).await;
}
```

- new routes: `/<segments..>`, `/ignore/<_>`, `/ignore/segments/<_..>`
- i18n & l10n: `/{i18n}/<segments..>`, `req.url.lang`
- testing: `assert_get(route, expected_res)`
- caching: `POST`, `PUT` & `PATCH` request data caching
- session storage and expiration manager
- file validation (size and type)
- HTTPS support

optional:
- OAuth, JWT, MFA, RBAC mechanisms
- ORM support
- easier DB integration