use std::slice::Iter;

use super::{
    game::ChessMove,
    pieces::{Color, Kind, Piece},
};
pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    pub fn default() -> Self {
        Board {
            squares: [
                Some(Piece::new(Color::White, Kind::Rook)),
                Some(Piece::new(Color::White, Kind::Knight)),
                Some(Piece::new(Color::White, Kind::Bishop)),
                Some(Piece::new(Color::White, Kind::Queen)),
                Some(Piece::new(Color::White, Kind::King)),
                Some(Piece::new(Color::White, Kind::Bishop)),
                Some(Piece::new(Color::White, Kind::Knight)),
                Some(Piece::new(Color::White, Kind::Rook)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Rook)),
                Some(Piece::new(Color::Black, Kind::Knight)),
                Some(Piece::new(Color::Black, Kind::Bishop)),
                Some(Piece::new(Color::Black, Kind::Queen)),
                Some(Piece::new(Color::Black, Kind::King)),
                Some(Piece::new(Color::Black, Kind::Bishop)),
                Some(Piece::new(Color::Black, Kind::Knight)),
                Some(Piece::new(Color::Black, Kind::Rook)),
            ],
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        unimplemented!()
    }

    pub fn to_fen(&self) -> String {
        unimplemented!()
    }

    pub fn play_move(&mut self, chess_move: &ChessMove) -> bool {
        unimplemented!()
    }

    fn can_move_piece(&self, piece: Piece, source_square: usize, target_square: usize) -> bool {
        // Implement the logic to check if the piece can move to the target square
        // based on its kind, color, and the current state of the board
        // Return true if the move is valid, false otherwise
        unimplemented!()
    }
}

pub struct Square {
    pub file: usize,
    pub rank: usize,
}

impl Square {
    pub fn new(file: usize, rank: usize) -> Square {
        Square { file, rank }
    }

    pub fn from_san(chars: &mut Iter<char>) -> Option<Square> {
        let file = match chars.next()? {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
        };
        let rank = match chars.next()? {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };
        Some(Square { file, rank })
    }
}
