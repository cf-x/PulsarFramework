# PulsarFramework

Fast and reliable web framework

```rust
use pulsar_web::{Pulse, pulsar};

async fn server(mut server: Pulse) {
    server.get("/", |_, res| {
        res.body("hello world!");
    }).await;

    server.launch(3000).await;
}
pulsar!(server);
```

## Installation

```bash
cargo add pulsar_web
```

## Features

- robust routing and http method handling
- built-in security mechanisms
- easy to write and read
- asynchronous by default


  incomplete/todo:


- caching & optimisations
- HTTPS and data encryption
- i18n
- testing