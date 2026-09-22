use crate::types::StatusCode;

impl StatusCode {
    pub const OK: Self = Self(200);
    pub const NOT_FOUND: Self = Self(404);
    pub const METHOD_NOT_ALLOWED: Self = Self(405);

    // Get the reason phrase for the status code
    pub fn reason_phrase(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            404 => "Not Found",
            405 => "Method Not Allowed",
            _ => "Unknown Status",
        }
    }

    // Convert the status code to its u16 representation, useful for sending the status code as part of the HTTP response
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}
