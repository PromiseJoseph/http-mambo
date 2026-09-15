use crate::types::StatusCode;

impl StatusCode {
    pub const OK: Self = Self(200);
    pub const NOT_FOUND: Self = Self(404);
    pub const METHOD_NOT_ALLOWED: Self = Self(405);

    pub fn reason_phrase(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            404 => "Not Found",
            405 => "Method Not Allowed",
            _ => "Unknown Status",
        }
    }
}
