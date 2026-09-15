use tokio::net::TcpListener;
pub async fn bind_addresses(custom_address: &str) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(custom_address).await
}
