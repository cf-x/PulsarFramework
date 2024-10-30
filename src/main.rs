use pulsar_web::{Pulse};

fn main() {
    let mut server = Pulse::new(3000);
    server.get("/", move |_| {
        ("hello, world!".to_string(), 200)
    });
    server.launch();
}
