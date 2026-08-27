use crate::types::Client;
use crate::types::HttpRequest;
use crate::types::RequestLines;
use std::collections::HashMap;
use std::println;
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

                let mut lines = request.lines();

                let request_line_: &str = lines.next().unwrap_or("");

                let mut headers = HashMap::new();

                // println!(
                //     "Request line: {}, line: {}",
                //     request_line_,
                //     lines.clone().collect::<Vec<&str>>().join(", ")
                // );

                for line in lines {
                    if let Some((key, value)) = line.split_once(": ") {
                        headers.insert(key.to_string(), value.to_string());
                    }
                }

                // println!(
                //     "Header host: {}, others: {:?}",
                //     headers.get("Host").unwrap_or(&"".to_string()),
                //     headers
                // );

                let request_item: Vec<&str> = request_line_.split_whitespace().collect();

                let request_lines = RequestLines {
                    method: request_item.get(0).unwrap_or(&"").to_string(),
                    path: request_item.get(1).unwrap_or(&"").to_string(),
                    version: request_item.get(2).unwrap_or(&"").to_string(),
                };

                let body = request
                    .lines()
                    .skip_while(|line| !line.is_empty())
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join("\n");
                println!("Request body: {}", body);

                let parsed_req = HttpRequest {
                    request_lines: request_lines.clone(),
                    headers: headers.clone(),
                    body: body,
                };

                println!("Parsed request: {:#?}", parsed_req);
                // Process the request and send a response
                let res_body: String = format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title> HttpMambo</title>
                    </head>
                    <body>
                        <h1>Welcome to HttpMambo!</h1>
                        <p>This is a simple HTTP server test page response implemented in Rust using Tokio.</p>
                        <p> test raw request: <span>{:#?}</span></p>
                    </body>
                    </html>                
                "#,
                    parsed_req
                );
                let response: String = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html; charset=utf-8\r\n\
                    Content-Length: {}\r\n\
                    \r\n\
                    {}",
                    res_body.as_bytes().len(),
                    res_body
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
