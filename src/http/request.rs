use crate::types::HttpMethod;
use crate::types::HttpRequest;
use crate::types::RequestLines;

use std::collections::HashMap;
/**
 *  Parsea request string and return a HttpRequest struct.
 */
impl HttpRequest {
    pub fn parse_request(req: &str) -> Self {
        let mut lines = req.lines();

        // Get the request line (e.g., "GET / HTTP/1.1")
        let request_line_: &str = lines.next().unwrap_or("");

        let mut headers = HashMap::new();

        println!("Request line: {}", request_line_); // for debugging purposes, to be removed later

        // Parse headers
        for line in lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // Parse the request line
        let request_item: Vec<&str> = request_line_.split_whitespace().collect();

        let method = match request_item.get(0).copied().unwrap_or("") {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            _ => {
                // Handle invalid method
                //default to GET for now,
                HttpMethod::GET
            }
        };

        let request_lines = RequestLines {
            method: method,
            path: request_item.get(1).unwrap_or(&"").to_string(),
            version: request_item.get(2).unwrap_or(&"").to_string(),
        };

        let body = req
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");
        println!("Request body: {}", body);

        // Creates the HttpRequest struct
        let parsed_req = HttpRequest {
            request_lines: request_lines.clone(),
            headers: headers.clone(),
            body: body,
        };
        return parsed_req;
    }
}
