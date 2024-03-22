use chess::game::Game;
use tonic::{transport::Server, Request, Response, Status};
pub mod chess {
    pub mod board;
    pub mod chess_move;
    pub mod game;
    pub mod pieces;
    pub mod square;
    tonic::include_proto!("chess_server"); // The string specified here must match the proto package name
}
use chess::match_server::{Match, MatchServer};
use chess::pieces::Color;
use chess::{MoveRequest, MoveResponse};
static GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Default)]
pub struct MatchService {}

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
    let addr = "[::1]:50051".parse()?;
    let match_service = MatchService {};

    Server::builder()
        .add_service(MatchServer::new(match_service))
        .serve(addr)
        .await?;
    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut s = String::new();
//     let mut game = chess::game::Game::new();
//     print!("Welcome to ChessBic! \n");
//     while s != "exit" {
//         let printed_game = game.print_board();
//         print!("{}", printed_game);
//         print!("Enter your move: ");
//         let _ = stdout().flush();
//         s.clear();
//         stdin()
//             .read_line(&mut s)
//             .expect("Did not enter a correct string");
//         if let Some('\n') = s.chars().next_back() {
//             s.pop();
//         }
//         if let Some('\r') = s.chars().next_back() {
//             s.pop();
//         }

//         game.play_move(&Color::White, s.as_str());
//         println!("You typed: {}", s);
//     }

//     Ok(())
// }
