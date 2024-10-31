use pulsar_web::{Pulse, pulsar};

async fn server(mut server: Pulse) {
    server.get("/", |req, res| {
        res.body("hello world!");
    }).await;

    server.launch(3000).await;
}

pulsar!(server);