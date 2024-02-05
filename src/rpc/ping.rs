use tonic::{Request, Response, Status};

use self::ping_server::Ping;

tonic::include_proto!("ping");

#[derive(Debug, Default, Clone, Copy)]
pub struct PingService;

#[tonic::async_trait]
impl Ping for PingService {
    async fn ping(&self, request: Request<PingReq>) -> Result<Response<PingResp>, Status> {
        log::debug!("Recive ping request {}", request.get_ref().req);
        Ok(Response::new(
            PingResp { resp: String::from("pong!") }
        ))
    }
}
