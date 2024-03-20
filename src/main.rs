use std::io::{stdin, stdout, Write};

use tonic::{transport::Server, Request, Response, Status};
pub mod chess {
    pub mod board;
    pub mod game;
    pub mod pieces;
    tonic::include_proto!("chess_server"); // The string specified here must match the proto package name
}
use chess::match_server::{Match, MatchServer};
use chess::{MoveRequest, MoveResponse};

use crate::chess::pieces::Color;

#[derive(Debug, Default)]
pub struct MatchService {}

// #[tonic::async_trait]
// impl Match for MatchService {
//     async fn move_piece(
//         &self,
//         request: Request<MoveRequest>,
//     ) -> Result<Response<MoveResponse>, Status> {
//         println!("Got a request: {:?}", request);
//         let reply = chess::MoveResponse {
//             success: true,
//             board_state: "board_state".into(),
//         };

//         Ok(Response::new(reply))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let match_service = MatchService {};

//     Server::builder()
//         .add_service(MatchServer::new(match_service))
//         .serve(addr)
//         .await?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    let mut game = chess::game::Game::new();
    print!("Welcome to ChessBic! \n");
    while s != "exit" {
        let printed_game = game.print_board();
        print!("{}", printed_game);
        print!("Enter your move: ");
        let _ = stdout().flush();
        s.clear();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        game.play_move(&Color::White, s.as_str());
        println!("You typed: {}", s);
    }

    Ok(())
}
