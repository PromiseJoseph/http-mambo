use crate::types::HttpResponse;

/**
 *  Convert the HttpResponse to a string
 */
impl HttpResponse {
    pub fn new(self) -> String {
        let mut response = String::new();

        // Add the status line
        response.push_str(&format!("HTTP/1.1 {} OK\r\n", self.status_code));
        // Add the headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str(&format!(
            "Content-Length: {}\r\n",
            self.body.as_bytes().len()
        ));

        response.push_str("\r\n");
        response.push_str(&self.body);

        response
    }
}
