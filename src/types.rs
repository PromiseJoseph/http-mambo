use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Client {
    pub peer_addr: SocketAddr,
}
