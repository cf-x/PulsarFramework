# Changelog

### 0.1.3

- fixed: route not parsing url
- fixed: query and slug parsing

### 0.1.2

- added `req.cookie` for reading requested cookies
- added session and expiration management tools [in `res.session`]

### 0.1.1

- changed `res.body` to `res.data`
- added `res.format` for setting content-type
- updated security policy
- fixed headers not applying to the response

### 0.1.0

- added asynchronicity
- returning response in methods is no longer required

### 0.0.5

- added `all` for handling any http request method
- added `file` and `download` response methods
- updated cookie handling
- security: `XSS`, `MitM`, `HTTP`, `CSRF` prevention

### 0.0.4

- added `.env` support
- added html responses
- fixed `404 not found` error

### 0.0.3

- added dynamic routing
- fixed request `body` and `headers` not passing in `req`
- fixed request parsing
- fixed route parsing

### 0.0.2

- added `req` and `res` arguments for request methods
- added `POST`, `PUT`, `PATCH`, `DELETE`
- updated method syntax

### 0.0.1

- added `GET` method handler
- added basic routing
- fixed server not starting