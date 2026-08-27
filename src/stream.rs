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
                let request = String::from_utf8_lossy(&buffer[..bytes]);
                println!("Received test request from {}: {}", peer_addr, request);
                // Process the request and send a response
                let body: String = format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title> HttpMambo</title>
                    </head>
                    <body>
                        <h1>Welcome to HttpMambo!</h1>
                        <p>This is a simple HTTP server test page response implemented in Rust using Tokio.</p>
                        <p> test raw request: <span>{request}</span></p>
                    </body>
                    </html>                
                "#
                );
                let response: String = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html; charset=utf-8\r\n\
                    Content-Length: {}\r\n\
                    \r\n\
                    {}",
                    body.as_bytes().len(),
                    body
                );
                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response to {}: {}", peer_addr, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from {}: {}", peer_addr, e);
                break;
            }
        }
    }
}
