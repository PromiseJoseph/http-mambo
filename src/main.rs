mod listener;
mod stream;
mod types;
use types::Client;
mod http;

const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:8000";
#[tokio::main]
async fn main() {
    println!("Starting ChatMambo server ");
    let arg = std::env::args().nth(1);
    let custom_addr = arg.as_deref();
    let listener = listener::bind_addresses(custom_addr).await.unwrap();

    println!("Server listening on: {:?}", listener.local_addr().unwrap());
    loop {
        let (stream, peer_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let (reader, writer) = stream.into_split();
        let client = Client { peer_addr };

        tokio::spawn(async move {
            stream::handle_stream(reader, writer, client).await;
        });
    }
}
