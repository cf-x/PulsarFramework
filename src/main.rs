use pulsar_web::Pulse;

fn main() {
    let mut server = Pulse::new(3000);
    server.get("/", |req, res| {
        res.body("hello world!");
        res.clone()
    });
    server.launch();
}
