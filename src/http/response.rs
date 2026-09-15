use crate::types::HttpResponse;

/**
 *  Convert the HttpResponse to a string
 */
impl HttpResponse {
    pub fn new(self) -> String {
        let mut response = String::new();

        //basic reason phrase mapping for common status codes, to be updated
        let reason_phrase = self.status_code.reason_phrase();

        // Add the status line
        response.push_str(&format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code.0, reason_phrase
        ));
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
