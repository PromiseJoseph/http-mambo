use crate::DEFAULT_SERVER_ADDRESS;
use tokio::io::{Error, ErrorKind};
use tokio::net::TcpListener;
pub async fn bind_addresses(custom_address: Option<&str>) -> Result<TcpListener, std::io::Error> {
    if let Some(addr) = custom_address {
        if let Ok(listener) = TcpListener::bind(addr).await {
            return Ok(listener);
        }
    }

    match TcpListener::bind(DEFAULT_SERVER_ADDRESS).await {
        Ok(listener) => return Ok(listener),
        Err(e) => eprintln!("Failed to bind to {}: {}", DEFAULT_SERVER_ADDRESS, e),
    }
    Err(Error::new(
        ErrorKind::AddrNotAvailable,
        "Failed to bind to default or any provided addresses",
    ))
}
