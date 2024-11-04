# Installation

To get started with Pulsar, initialize a new cargo project and install necessary crates:

```bash
cargo new my_project
cd ./my_project
cargo add pulsar_web
```

After the installation is complete, set up the necessary boilerplate in the `src/main.rs` and
launch the application:

```rust
use pulsar_web::{Pulse, pulsar};

async fn server(mut server: Pulse) {
    server.get("/", |_, res| {
        res.data("hello, pulsar!");
    }).await;

    server.launch(3000).await;
}

pulsar!(server);
```

```bash
cargo run
```

```bash
# output:
server launched on 127.0.0.1:3000
```

If everything was done right, you should see `hello, pulsar!` when visiting the
localhost at a port 3000.

#### todo:

- `pulsar-cli` for faster initialization, better DX and debugging
- syntax highlighting on docs page