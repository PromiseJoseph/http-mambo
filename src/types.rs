use std::{collections::HashMap, net::SocketAddr};

#[derive(Debug, Clone)]
pub struct RequestLines {
    pub method: String,
    pub path: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub request_lines: RequestLines,
    pub headers: HashMap<String, String>,
    pub body: String,
}
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub peer_addr: SocketAddr,
}
