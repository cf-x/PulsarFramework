# PulsarFramework

Fast and reliable web framework (under the heavy development)

```rust
use pulsar_web::Pulse;

fn main() {
    let mut server = Pulse::new(3000);
    server.launch();
    server.get("/", |_| {
        ("hello, world".to_string(), 200)
    });
}
```