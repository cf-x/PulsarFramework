# Routing

Every server needs routing functionality to handle requests on different routes. Pulsar's approach
to the routing is to have as robust and utilized as possible while keeping simplicity and modernity.

### Routes

Pulsar handler three components in the routing:

1. static routes (`/`, `/about`, ...)
2. dynamic routes (`/user/<id>`, `/items/<category>/<id>`, ...)
3. parameters (`/search?query=hello%20%world`)

Routes can be specified when constructing the method, and query parameters can be read/written
by the method handler:

```rust
server.get("/search", |req, res| {
    let query = req.query.get("query");
    if query.is_some(){
        res.data(format!("searching for: '{}'!", query.unwrap()));
        res.status(200);
    } else {
        res.data("expected search query");
        res.status(400);
    }
});
```

The code above is listening to the `/search` route for https `GET` method, and if the request
has `query` parameter it returns a normal response, and if not an error.

```rust
server.get("/user/<username>", |req, res| {
    let username = req.route.slugs.get("username").unwrap();
    res.data(format!("hello, {}!", username));
});
```

Requests to the routes`/user/michael`, `/user/john`, or `/user/whatever` will be handled
by the method above. You can access route parameters by calling `req.route.slugs.get("name")`.


<Note type="todo">

- `route` method for handling multiple methods together
- better API (`req.route.slugs.get()` -> `req.slugs.get()`, ...)
- `/...` and `/<named...>` for better route handling
- automatic route ranker 
- parser for params (`hello world` -> `hello%20%world`)

</Note>
