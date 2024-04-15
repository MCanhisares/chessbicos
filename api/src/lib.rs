mod auth;
mod chess;
mod db {
    pub mod connector;
}
use chess::match_server::{Match, MatchServer};
use chess::{pieces::Color, MoveRequest, MoveResponse};
use db::connector::{self};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use std::env;
use tonic::{transport::Server, Request, Response, Status};
use auth::service::AuthService;
use auth::auth_server::AuthServer;

use crate::chess::game::Game;
static GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("chessbicos_descriptor");

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
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("[::0]:{}", port).parse()?;
    let db = connector::db_connector().await?;    
    Migrator::up(&db, None).await?;
    let match_service = MatchService { db_connection: db.clone() };
    let auth_service = AuthService { db_connection: db.clone() };
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(service)
        .add_service(MatchServer::new(match_service))
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
