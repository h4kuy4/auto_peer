use std::{error::Error, net::SocketAddr, str::FromStr};

use auto_peer::rpc::ping::{PingService, ping_server::PingServer};
use dotenv::dotenv;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let addr = SocketAddr::from_str("[::1]:8080")?;
    let ping_service = PingServer::new(PingService::default());

    Server::builder()
        .add_service(ping_service)
        .serve(addr)
        .await?;

    Ok(())
}
