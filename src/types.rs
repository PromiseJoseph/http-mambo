use std::{collections::HashMap, net::SocketAddr};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestLines {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub request_lines: RequestLines,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub peer_addr: SocketAddr,
}

pub type Handler = fn(&HttpRequest) -> HttpResponse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handler: Handler,
}
pub struct Router {
    pub routes: Vec<Route>,
}

#[derive(Debug, Clone)]
pub struct HttpStatus(u16, &'static str);
