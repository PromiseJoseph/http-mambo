use crate::types::Client;
use crate::types::HttpRequest;
use crate::types::Router;
use std::sync::Arc;

use std::io::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn handle_stream(
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
    client: Client,
    router: Arc<Router>,
) -> Result<(), Error> {
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

                let res = router.handle_request(&parsed_req);

                // Send the response back to the client
                writer.write_all(res.to_http_string().as_bytes()).await?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}
