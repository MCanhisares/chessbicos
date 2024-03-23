use super::{
    board::Board,
    chess_move::ChessMove,
    pieces::{Color, Kind, Piece},
    square::Square,
};
use std::fmt;

pub struct GameError;

// Implement std::fmt::Display for AppError
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
//Game state represented in FEN notation https://www.chessprogramming.org/Forsyth-Edwards_Notation
pub struct Game {
    board: Board,
    //b or w
    turn: Color,
    // K Q k q
    // If neither side can castle, the symbol '-' is used, otherwise each of four individual castling rights for king and queen castling for both sides are indicated by a sequence of one to four letters.
    castling: Option<(Option<Piece>, Option<Piece>, Option<Piece>, Option<Piece>)>,
    // e3
    //The en passant target square is specified after a double push of a pawn, no matter whether an en passant capture is really possible or not. Other moves than double pawn pushes imply the symbol '-' for this FEN field.
    en_passant: Option<Square>,
    // The halfmove clock specifies a decimal number of half moves with respect to the 50 move draw rule. It is reset to zero after a capture or a pawn move and incremented otherwise.
    half_move: u32,
    // The number of the full moves in a game. It starts at 1, and is incremented after each Black's move.
    full_move: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            turn: Color::White,
            castling: Some((
                Some(Piece::new(Color::White, Kind::King)),
                Some(Piece::new(Color::White, Kind::Queen)),
                Some(Piece::new(Color::Black, Kind::King)),
                Some(Piece::new(Color::Black, Kind::Queen)),
            )),
            en_passant: None,
            half_move: 0,
            full_move: 1,
        }
    }

    pub fn from_fen(fen_game: &str) -> Result<Game, GameError> {
        let mut fen_iter = fen_game.split(' ').into_iter();
        let board = match fen_iter.next() {
            None => return Err(GameError),
            Some(s) => Board::from_fen(s),
        };
        let color = match fen_iter.next() {
            None => return Err(GameError),
            Some(s) => Color::from_str(s),
        };
        if color.is_none() {
            return Err(GameError);
        }
        let castling = match fen_iter.next() {
            None => return Err(GameError),
            Some(s) => {
                let mut castling_rights = (None, None, None, None);
                for c in s.chars() {
                    match c {
                        'K' => castling_rights.0 = Some(Piece::new(Color::White, Kind::King)),
                        'Q' => castling_rights.1 = Some(Piece::new(Color::White, Kind::Queen)),
                        'k' => castling_rights.2 = Some(Piece::new(Color::Black, Kind::King)),
                        'q' => castling_rights.3 = Some(Piece::new(Color::Black, Kind::Queen)),
                        _ => return Err(GameError),
                    }
                }
                Some(castling_rights)
            }
        };

        let en_passant = match fen_iter.next() {
            None => return Err(GameError),
            Some(s) => {
                if s == "-" {
                    None
                } else {
                    Square::from_san_str(s)
                }
            }
        };

        let half_move = match fen_iter.next() {
            None => return Err(GameError),
            Some(s) => match s.parse::<u32>() {
                Ok(hm) => hm,
                Err(_) => return Err(GameError),
            },
        };

        let full_move = match fen_iter.next() {
            None => return Err(GameError),
            Some(s) => match s.parse::<u32>() {
                Ok(fm) => fm,
                Err(_) => return Err(GameError),
            },
        };

        let game = Game {
            board,
            turn: color.unwrap(),
            castling,
            en_passant,
            half_move,
            full_move,
        };
        Ok(game)
    }

    pub fn to_fen(&self) -> String {
        let mut fen = self.board.to_fen();
        fen.push(' ');
        fen.push_str(self.turn.as_str());
        fen.push(' ');
        let mut castling = String::new();
        if let Some((wk, wq, bk, bq)) = self.castling {
            if wk.is_some() {
                castling.push('K');
            }
            if wq.is_some() {
                castling.push('Q');
            }
            if bk.is_some() {
                castling.push('k');
            }
            if bq.is_some() {
                castling.push('q');
            }
        }
        if castling.is_empty() {
            fen.push_str("-");
        } else {
            fen.push_str(&castling);
        }
        fen.push(' ');
        if let Some(en_passant) = &self.en_passant {
            fen.push_str(&en_passant.to_san());
        } else {
            fen.push_str("-");
        }
        fen.push(' ');
        fen.push_str(&self.half_move.to_string());
        fen.push(' ');
        fen.push_str(&self.full_move.to_string());
        fen
    }

    pub fn print_board(&self) -> String {
        self.board.print_board()
    }

    fn can_castle(&self, color: &Color, kind: &Kind) -> bool {
        if let Some((wk, wq, bk, bq)) = self.castling {
            match color {
                Color::White => match kind {
                    Kind::King => wk.is_some(),
                    Kind::Queen => wq.is_some(),
                    _ => false,
                },
                Color::Black => match kind {
                    Kind::King => bk.is_some(),
                    Kind::Queen => bq.is_some(),
                    _ => false,
                },
            }
        } else {
            false
        }
    }

    pub fn play_move(&mut self, player: &Color, san_move: &str) -> bool {
        if self.turn != *player {
            return false;
        }
        let chess_move = ChessMove::from_san(player, san_move);

        match chess_move {
            None => return false,
            Some(chess_move) => {
                if chess_move.castling.is_some() {
                    let kind = chess_move.castling.unwrap();
                    if !self.can_castle(player, &kind) {
                        return false;
                    }
                }
                let success = self.board.play_move(&chess_move, player);
                if success {
                    self.turn = match self.turn {
                        Color::White => Color::Black,
                        Color::Black => Color::White,
                    };
                    self.full_move += 1;
                }
                success
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_fen() {
        let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(game.is_ok());
        let game = game.unwrap();
        assert_eq!(game.turn, Color::White);
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);
        assert_eq!(
            game.castling,
            Some((
                Some(Piece::new(Color::White, Kind::King)),
                Some(Piece::new(Color::White, Kind::Queen)),
                Some(Piece::new(Color::Black, Kind::King)),
                Some(Piece::new(Color::Black, Kind::Queen))
            ))
        );
        assert_eq!(game.en_passant, None);
    }

    #[test]
    fn test_game_to_fen() {
        let game = Game::new();
        let fen = game.to_fen();
        assert_eq!(
            fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_game_play_move() {
        let mut game = Game::new();
        let success = game.play_move(&Color::White, "e4");
        assert!(success);
        let success = game.play_move(&Color::Black, "e5");
        assert!(success);
        let success = game.play_move(&Color::White, "Nf3");
        assert!(success);
        let success = game.play_move(&Color::Black, "Nc6");
        assert!(success);
        let success = game.play_move(&Color::White, "Bb5");
        assert!(success);
        let success = game.play_move(&Color::Black, "Bb4");
        assert!(success);
        let success = game.play_move(&Color::White, "h3");
        assert!(success);
        let success = game.play_move(&Color::Black, "Nf6");
        assert!(success);
        let success = game.play_move(&Color::White, "O-O");
        assert!(success);
        let success = game.play_move(&Color::Black, "O-O");
        assert!(success);
        let success = game.play_move(&Color::White, "d3");
        assert!(success);
        let success = game.play_move(&Color::Black, "d6");
        assert!(success);
        let success = game.play_move(&Color::White, "c3");
        assert!(success);
        let success = game.play_move(&Color::Black, "c6");
        assert!(success);
        let success = game.play_move(&Color::White, "Bc4");
        assert!(success);
        let success = game.play_move(&Color::Black, "b5");
        assert!(success);
    }
}
