use tonic::{transport::Server, Request, Response, Status};
pub mod chess {
    tonic::include_proto!("chess"); // The string specified here must match the proto package name
}
use chess::match_server::{Match, MatchServer};
use chess::{MoveRequest, MoveResponse};

#[derive(Debug, Default)]
pub struct MatchService {}

#[tonic::async_trait]
impl Match for MatchService {
  async fn move_piece(&self, request: Request<MoveRequest>) -> Result<Response<MoveResponse>, Status> {
    println!("Got a request: {:?}", request);
    let reply = chess::MoveResponse {
      success: true,
      board_state: "board_state".into()
    };

    Ok(Response::new(reply))
  }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let match_service = MatchService {};
    
    Server::builder()
        .add_service(MatchServer::new(match_service))
        .serve(addr)
        .await?;
    Ok(())
}