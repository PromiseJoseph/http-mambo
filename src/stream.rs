use crate::types::Client;
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
                let message = String::from_utf8_lossy(&buffer[..bytes]);
                println!("Received test request from {}: {}", peer_addr, message);
            }
            Err(e) => {
                eprintln!("Failed to read from {}: {}", peer_addr, e);
                break;
            }
        }
    }
}
