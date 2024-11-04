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

- new routes: `/<segments..>`, `/ignore/<_>`, `/ignore/segments/<_..>`
- i18n & l10n: `/{i18n}/<segments..>`, `req.url.lang`
- testing: `assert_get(route, expected_res)`
- caching: `POST`, `PUT` & `PATCH` request data caching
- file validation (size and type)
- HTTPS support
- pulsar-cli for watching, initializing and adding features

optional:

- OAuth, JWT, MFA, RBAC mechanisms
- ORM support
- easier DB integration