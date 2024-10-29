use PulsarFramework::{Pulse, Result};

fn main() {
    Pulse::launch(3000);
    Pulse::get("/", |_| {
        (Result::String("hello, world"), 200)
    })
}
