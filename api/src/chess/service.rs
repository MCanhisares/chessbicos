use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};

use super::{chess_game_server::ChessGame, game::Game, pieces::Color, MoveRequest, MoveResponse};

static GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct ChessGameService {
    pub db_connection: DatabaseConnection,
}

#[tonic::async_trait]
impl ChessGame for ChessGameService {
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

        let reply = MoveResponse {
            match_id: r.match_id,
            success,
            board_state,
        };

        Ok(Response::new(reply))
    }
}
