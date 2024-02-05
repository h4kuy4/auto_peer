use std::error::Error;

use auto_peer::rpc::ping::{ping_client::PingClient, PingReq};
use dotenv::dotenv;
use tonic::transport::{Endpoint, Channel};

async fn ping(client: &mut PingClient<Channel>) -> Result<(), Box<dyn Error>> {
    loop {
        let req = PingReq {
            req: String::from("ping")
        };

        let resp = client.ping(req).await?;
        log::debug!("Recived resp {}", resp.get_ref().resp);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();
    let channel = Endpoint::from_static("http://[::1]:8080").connect().await?;

    let ping_client = PingClient::new(channel.clone());

    let ping_task = tokio::spawn(async move {
        let mut c = ping_client.clone();
        if let Err(e) = ping(&mut c).await {
            println!("voting error: {}", e);
        }
    });

    tokio::try_join!(ping_task)?;

    Ok(())
}
