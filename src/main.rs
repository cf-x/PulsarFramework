use pulsar_web::{Pulse, pulsar};

async fn server(mut server: Pulse) {
    server.get("/", |req, res| {

        res.data("hello world");
    }).await;

    server.launch(3000).await;
}

pulsar!(server);
