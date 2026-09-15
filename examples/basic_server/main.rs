mod handler;
use handler::home;
use http_mambo::types::{Client, Router};
use http_mambo::{listener, stream};
use std::sync::Arc;

const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    router.post("/hello", home);
    router.get("/", home);

    let arg = std::env::args().nth(1);

    let custom_addr: &str = match arg.as_deref() {
        Some(addr) => addr,
        None => DEFAULT_SERVER_ADDRESS,
    };

    let listener = listener::bind_addresses(custom_addr).await.unwrap();

    let router = Arc::new(router);

    println!("Server listening on: {:?}", listener.local_addr().unwrap());
    loop {
        let (stream, peer_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let (reader, writer) = stream.into_split();
        let client = Client { peer_addr };

        let router = Arc::clone(&router);

        tokio::spawn(async move {
            stream::handle_stream(reader, writer, client, router).await;
        });
    }
}
