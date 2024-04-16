mod auth;
mod chess;
mod db {
    pub mod connector;
}

use auth::{auth_server::AuthServer, service::AuthService};
use chess::{chess_game_server::ChessGameServer, service::ChessGameService};
use db::connector::{self};
use migration::{Migrator, MigratorTrait};
use std::env;
use tonic::transport::Server;
const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("chessbicos_descriptor");

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("[::0]:{}", port).parse()?;
    let db = connector::db_connector().await?;
    Migrator::up(&db, None).await?;

    // Cloning is not a problem, https://github.com/SeaQL/sea-orm/discussions/2198#discussioncomment-9119481
    let auth_service = AuthService {
        db_connection: db.clone(),
    };
    let chess_game_service = ChessGameService {
        db_connection: db.clone(),
    };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(ChessGameServer::new(chess_game_service))
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
