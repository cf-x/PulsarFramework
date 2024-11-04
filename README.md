# PulsarFramework

Fast and reliable web framework.

### Getting Started

First, create the new rust project and install the `pulsar_web` crate:

```bash
cargo new my_project
```

```bash
cargo add pulsar_web
```

Now, set up the boilerplate in the `src/main.rs` to get started:

```rust
use pulsar_web::{Pulse, pulsar};

async fn server(mut server: Pulse) {
    server.get("/", |_, res| {
        res.data("hello world!");   
    }).await;

    server.launch(3000).await;
}
pulsar!(server);
```

And now launch the server and visit the localhost on the port `3000`:

```bash
cargo run
# output: server launched on: http://127.0.0.1:3000/
```

### Routing

Route path is defined in the first argument of the http method:

```rs
server.get("/", |_, res| {
    res.data("hello, world!");
}).await;
```

> `"/"` is the route, `get` correspondences to http GET method and the second argument
> of the method is the method handler, with `req` and `res` parameters.

In the code above, GET method on `/` route is being handled with the second argument.
You can pass the route you want to listen to in the first argument (e.g `/user/settings`).
If a request is made to the unhandled route, 404 error message is responded.

Priority ranking for the routes comes from the first defined method to the last.

To implement dynamic routing, you can wrap the slug name with `<` and `>`:

```rs
server.get("/user/<username>", |req, res| {
    let message = format!("hello, {}!", req.route.slugs.get("username"));
    res.data(&message);
}).await;
```

You can easily get the query data as well:

```rs
server.get("/?key=value", |req, res|{
    let param = req.route.params.get("key");
}).await;
```

### Methods

|  http  | pulsar |
|:------:|:------:|
|  GET   |  get   |
|  POST  |  post  |
|  PUT   |  put   |
| PATCH  | patch  |
| DELETE | delete |
|  any   |  all   |

Every method handler has the same API. For example, to get the received data from the POST request,
you can use `req.body`, which is also accessible in GET handler but has a value of an empty string.

`req` parameter in the handler's closure argument, represents the requested data,
and `res` - responded. `req` isn't mutable, but the response can be changed.

### Other

Framework also includes cookie, session and expiration manager tools, built inside the
`req` and `res` arguments.

### Documentation

Project is in the early stage of the development and setting up the
documentation would be huge waste of time due to the drastic changes between
the versions. Please feel free to contribute or give us some tips!

### License

This software is released in the public domain, meaning that anyone can distribute, modify, use or
publish it. Read the full [LICENSE](LICENSE).
