use PulsarFramework::{Pulse};

fn main() {
    let mut server = Pulse::new(3000);
    server.launch();
    server.get("/", |_| {
        ("hello, world".to_string(), 200)
    });
}
