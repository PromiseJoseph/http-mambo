use crate::http::handler::handle_request;
use crate::types::Client;
use crate::types::HttpRequest;
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

                /*  Process the request and send a response
                   res is an instance of HttpResponse, which is created by the handle_request function based on the parsed request. The response is then converted to a string using the new() method and sent back to the client using the writer.
                */
                let res = handle_request(&parsed_req);

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
