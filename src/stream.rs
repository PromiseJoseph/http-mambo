use crate::http::handler::home;
use crate::types::Client;
use crate::types::HttpRequest;
use crate::types::Router;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn handle_stream(mut reader: OwnedReadHalf, mut writer: OwnedWriteHalf, client: Client) {
    let mut buffer = [0; 1024];
    loop {
        let peer_addr = &client.peer_addr;

        match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", peer_addr);
                break;
            }
            Ok(bytes) => {
                let request = String::from_utf8_lossy(&buffer[..bytes]);
                println!("Received test request from {}: {}", peer_addr, request);

                let parsed_req = HttpRequest::parse_request(&request);

                println!("Parsed request: {:#?}", parsed_req);

                /*
                ==========================
                 Router test
                 ==========================
                 */

                let mut router = Router::new();
                router.get("/hello", home); //using get method to test the 405 method not allowed response using a browser meh!

                let res = router.handle_request(&parsed_req);

                // ==========================

                // Send the response back to the client
                writer
                    .write_all(res.new().as_bytes())
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to send response to {}: {}", peer_addr, e);
                    });
            }
            Err(e) => {
                eprintln!("Failed to read from {}: {}", peer_addr, e);
                break;
            }
        }
    }
}
