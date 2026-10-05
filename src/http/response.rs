use crate::types::{HttpResponse, StatusCode};
use std::collections::HashMap;

/**
 *  Convert the HttpResponse to a string
 */
impl HttpResponse {
    // Default constructor for HttpResponse
    pub fn new() -> Self {
        Self {
            status_code: StatusCode::OK,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    // //getters for the response fields
    // pub fn status(&self) -> u16 {
    //     self.status_code.as_u16()
    // }
    // pub fn status_code(&self) -> StatusCode {
    //     self.status_code
    // }
    // pub fn headers(&self) -> HashMap<String, String> {
    //     self.headers.clone()
    // }

    // pub fn body(&self) -> &str {
    //     &self.body
    // }

    /*
     * Builder methods for the response fields
     * Configure the response with status code, and body
     */
    pub fn with_status_and_body(status_code: StatusCode, body: Vec<u8>) -> Self {
        Self {
            status_code,
            headers: HashMap::new(),
            body: body,
        }
    } //shorthand for creating a response with status code and body

    pub fn with_status(status_code: StatusCode) -> Self {
        Self {
            status_code,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = body.into_bytes();
        self
    }

    pub fn with_body_bytes(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn to_http_bytes(self) -> Vec<u8> {
        let mut response = Vec::new();

        let reason_phrase = self.status_code.reason_phrase();

        // Add the status line
        response.extend_from_slice(
            format!("HTTP/1.1 {} {}\r\n", self.status_code.0, reason_phrase).as_bytes(),
        );

        // Add the headers
        for (key, value) in &self.headers {
            // ignore the Content-Length header if it exists
            if key.eq_ignore_ascii_case("Content-Length") {
                continue;
            }
            response.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
        }

        response.extend_from_slice(format!("Content-Length: {}\r\n", self.body.len()).as_bytes());

        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(&self.body);

        response
    }
}
