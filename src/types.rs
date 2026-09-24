use std::future::Future;
use std::pin::Pin;
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
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub peer_addr: SocketAddr,
}

pub type BoxFuture = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;

pub type Handler = Box<dyn Fn(HttpRequest) -> BoxFuture + Send + Sync>;

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

pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handler: Handler,
}
pub struct Router {
    pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(pub u16);
