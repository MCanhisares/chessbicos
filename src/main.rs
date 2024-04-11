use chess::game::Game;
use sea_orm::{DatabaseConnection, DbErr};
use tonic::{transport::Server, Request, Response, Status};
pub mod db {
    pub mod connector;
}
pub mod chess {
    pub mod board;
    pub mod chess_move;
    pub mod game;
    pub mod pieces;
    pub mod square;
    tonic::include_proto!("chess_server"); // The string specified here must match the proto package name
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("chess_server_descriptor");
}
use chess::match_server::{Match, MatchServer};
use chess::pieces::Color;
use chess::{MoveRequest, MoveResponse};
use db::connector;
use std::env;
static GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Default)]
pub struct MatchService {
    db_connection: DatabaseConnection,
}

#[tonic::async_trait]
impl Match for MatchService {
    async fn move_piece(
        &self,
        request: Request<MoveRequest>,
    ) -> Result<Response<MoveResponse>, Status> {
        println!("Got a request: {:?}", request);
        let r = request.into_inner();
        let color = Color::from_str(r.player_color.as_str());
        if color.is_none() {
            return Err(Status::invalid_argument("Invalid color"));
        }
        let game_parse = Game::from_fen(GAME_FEN);
        if game_parse.is_err() {
            return Err(Status::invalid_argument("Invalid FEN"));
        }
        let mut game = game_parse.unwrap();
        let success = game.play_move(&color.unwrap(), &r.pgn_move);
        let board_state = game.to_fen();

        let reply = chess::MoveResponse {
            match_id: r.match_id,
            success,
            board_state,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("[::0]:{}", port).parse()?;
    let db = connector::db_connector().await?;
    let match_service = MatchService { db_connection: db };
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(chess::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(service)
        .add_service(MatchServer::new(match_service))
        .serve(addr)
        .await?;

    Ok(())
}
