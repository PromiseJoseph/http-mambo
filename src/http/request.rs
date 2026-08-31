use crate::types::HttpRequest;
use crate::types::RequestLines;
use std::collections::HashMap;

/**
 *  Parse a request string and return a HttpRequest struct.
 */
impl HttpRequest {
    pub fn parse_request(req: &str) -> Self {
        let mut lines = req.lines();

        // Get the request line (e.g., "GET / HTTP/1.1")
        let request_line_: &str = lines.next().unwrap_or("");

        let mut headers = HashMap::new();

        // println!(
        //     "Request line: {}, line: {}",
        //     request_line_,
        //     lines.clone().collect::<Vec<&str>>().join(", ")
        // );

        // Parse headers
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

        // Parse the request line
        let request_item: Vec<&str> = request_line_.split_whitespace().collect();

        let request_lines = RequestLines {
            method: request_item.get(0).unwrap_or(&"").to_string(),
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
