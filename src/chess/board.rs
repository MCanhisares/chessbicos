use std::slice::Iter;

use super::{
    game::ChessMove,
    pieces::{Color, Kind, Piece},
};
pub struct Board {
    squares: [Option<Piece>; 64],
    castling: Option<(Option<Piece>, Option<Piece>, Option<Piece>, Option<Piece>)>,
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
            // K Q k q
            castling: Some((
                Some(Piece::new(Color::White, Kind::King)),
                Some(Piece::new(Color::White, Kind::Queen)),
                Some(Piece::new(Color::Black, Kind::King)),
                Some(Piece::new(Color::Black, Kind::Queen)),
            )),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        unimplemented!()
    }

    pub fn to_fen(&self) -> String {
        unimplemented!()
    }

    pub fn play_move(&mut self, chess_move: &ChessMove, player: Color) -> bool {
        //Castling requires no to or from squares
        if chess_move.castling.is_some() {
            return self.castle(player, chess_move.castling.unwrap());
        }
        let piece = chess_move.piece;
        //Check if promotion is valid
        let promoted_piece = match chess_move.promotion {
            Some(kind) => {
                if piece.kind != Kind::Pawn {
                    return false;
                }
                Some(Piece::new(player, kind))
            }
            None => None,
        };
        //If no to square is provided, move is invalid
        let to_square = match chess_move.to {
            Some(square) => square,
            None => return false,
        };
        //From square can be provided; if not provided we need to find from candidate pieces
        let from_square: Option<Square> = match chess_move.from_square {
            Some(square) => Some(square),
            None => {
                if chess_move.from_file.is_none() || chess_move.from_rank.is_none() {
                    None
                } else {
                    Some(Square::new(
                        chess_move.from_file.unwrap(),
                        chess_move.from_rank.unwrap(),
                    ))
                }
            }
        };
        //If from square is provided, we need to check if the piece is in the correct position
        //If not, the move is invalid
        //If true, we move the piece into the new square
        //TODO: Check for promotion
        if from_square.is_some() {
            let from_square = from_square.unwrap();
            if let Some(p) = self.squares[from_square.to_1d_arr_coordinates()] {
                if p.color != player || p.kind != piece.kind {
                    return false;
                }
                self.move_piece(from_square, to_square, promoted_piece);                
            } else {
                return false;
            }
        }

        let candidate_pieces = match from_square {
            Some(square) => match self.squares[square.to_1d_arr_coordinates()] {
                Some(p) => {
                    if p.color == player && p.kind == piece.kind {
                        let mut pieces = Vec::new();
                        pieces.push(Some(p));
                        pieces
                    } else {
                        return false;
                    }
                }
                None => return false,
            },
            None => {
                let mut pieces = Vec::new();
                for (i, p) in self.squares.iter().enumerate() {
                    if let Some(p) = p {
                        if p.color == player && p.kind == piece.kind {
                            pieces.push(Some(*p));
                        }
                    }
                }
                pieces
            }
        };

        false
    }

    pub fn castle(&mut self, color: Color, kind: Kind) -> bool {
        let king_square = match color {
            Color::White => Square::from_san_str("a5"),
            Color::Black => Square::from_san_str("h5"),
        }
        .unwrap();

        //Check if the king is in the correct position
        match self.squares[king_square.to_1d_arr_coordinates()] {
            Some(p) => {
                if p.kind != Kind::King {
                    return false;
                }
            }
            None => return false,
        }

        let rook_square = match (color, kind) {
            (Color::White, Kind::King) => Square::from_san_str("h1"),
            (Color::White, Kind::Queen) => Square::from_san_str("a1"),
            (Color::Black, Kind::King) => Square::from_san_str("h8"),
            (Color::Black, Kind::Queen) => Square::from_san_str("a8"),
            _ => return false,
        }
        .unwrap();

        //Check if the rook is in the correct position
        if let Some(piece) = self.squares[rook_square.to_1d_arr_coordinates()] {
            if piece.kind != Kind::Rook {
                return false;
            }
        } else {
            return false;
        }

        let mut empty_squares: Vec<usize> = Vec::new();
        match (color, kind) {
            (Color::White, Kind::King) => {
                empty_squares.push(Square::from_san_str("f1").unwrap().to_1d_arr_coordinates());
                empty_squares.push(Square::from_san_str("g1").unwrap().to_1d_arr_coordinates());
            }
            (Color::White, Kind::Queen) => {
                empty_squares.push(Square::from_san_str("b1").unwrap().to_1d_arr_coordinates());
                empty_squares.push(Square::from_san_str("c1").unwrap().to_1d_arr_coordinates());
                empty_squares.push(Square::from_san_str("d1").unwrap().to_1d_arr_coordinates());
            }
            (Color::Black, Kind::King) => {
                empty_squares.push(Square::from_san_str("f8").unwrap().to_1d_arr_coordinates());
                empty_squares.push(Square::from_san_str("g8").unwrap().to_1d_arr_coordinates());
            }
            (Color::Black, Kind::Queen) => {
                empty_squares.push(Square::from_san_str("b8").unwrap().to_1d_arr_coordinates());
                empty_squares.push(Square::from_san_str("c8").unwrap().to_1d_arr_coordinates());
                empty_squares.push(Square::from_san_str("d8").unwrap().to_1d_arr_coordinates());
            }
            _ => return false,
        }
        //Check if the squares between the king and the rook are empty
        for square in empty_squares {
            if self.squares[square].is_some() {
                return false;
            }
        }

        //Move the king and the rook
        match (color, kind) {
            (Color::White, Kind::King) => {
                //Move the rook from h1 to f1
                self.move_piece(rook_square, Square::from_san_str("h1").unwrap(), None);
                //Move the king from e1 to g1
                self.move_piece(king_square, Square::from_san_str("g1").unwrap(), None);
            }
            (Color::White, Kind::Queen) => {
                //Move the rook from a1 to d1
                self.move_piece(rook_square, Square::from_san_str("d1").unwrap(), None);
                //Move the king from e1 to c1
                self.move_piece(king_square, Square::from_san_str("c1").unwrap(), None);
            }
            (Color::Black, Kind::King) => {
                //Move the rook from h8 to f8
                self.move_piece(rook_square, Square::from_san_str("f8").unwrap(), None);
                //Move the king from e8 to g8
                self.move_piece(king_square, Square::from_san_str("g8").unwrap(), None);
            }
            (Color::Black, Kind::Queen) => {
                //Move the rook from a8 to d8
                self.move_piece(rook_square, Square::from_san_str("d8").unwrap(), None);
                //Move the king from e8 to c8
                self.move_piece(king_square, Square::from_san_str("c8").unwrap(), None);
            }
            _ => return false,
        }

        true
    }

    fn move_piece(
        &self,
        from_square: Square,
        to_square: Square,
        promoted_piece: Option<Piece>,
    ) -> bool {
        self.move_piece_coordinate(
            from_square.to_1d_arr_coordinates(),
            to_square.to_1d_arr_coordinates(),
            promoted_piece,
        )
    }

    fn move_piece_coordinate(
        &self,
        from_square: usize,
        to_square: usize,
        promoted_piece: Option<Piece>,
    ) -> bool {
        if from_square < 0
            || from_square > 63
            || to_square < 0
            || to_square > 63
            || from_square == to_square
        {
            return false;
        }

        self.squares[to_square] = self.squares[from_square];
        self.squares[from_square] = None;
        return true;
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

    pub fn to_1d_arr_coordinates(&self) -> usize {
        self.file * 8 + self.rank
    }

    pub fn from_san_str(san_str: &str) -> Option<Square> {
        let mut chars = san_str.chars();
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
