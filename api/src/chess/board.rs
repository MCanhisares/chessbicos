use super::{
    chess_move::ChessMove,
    pieces::{Color, Kind, Piece},
    square::Square,
};

#[derive(PartialEq, Debug)]
pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    pub fn default() -> Self {
        Board {
            squares: [
                Some(Piece::new(Color::Black, Kind::Rook)),
                Some(Piece::new(Color::Black, Kind::Knight)),
                Some(Piece::new(Color::Black, Kind::Bishop)),
                Some(Piece::new(Color::Black, Kind::Queen)),
                Some(Piece::new(Color::Black, Kind::King)),
                Some(Piece::new(Color::Black, Kind::Bishop)),
                Some(Piece::new(Color::Black, Kind::Knight)),
                Some(Piece::new(Color::Black, Kind::Rook)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
                Some(Piece::new(Color::Black, Kind::Pawn)),
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
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Pawn)),
                Some(Piece::new(Color::White, Kind::Rook)),
                Some(Piece::new(Color::White, Kind::Knight)),
                Some(Piece::new(Color::White, Kind::Bishop)),
                Some(Piece::new(Color::White, Kind::Queen)),
                Some(Piece::new(Color::White, Kind::King)),
                Some(Piece::new(Color::White, Kind::Bishop)),
                Some(Piece::new(Color::White, Kind::Knight)),
                Some(Piece::new(Color::White, Kind::Rook)),
            ],
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board {
            squares: [None; 64],
        };
        let mut rank = 0;
        let mut file = 0;

        for c in fen.chars() {
            match c {
                ' ' => break,
                '/' => {
                    rank += 1;
                    file = 0;
                }
                '1'..='8' => {
                    let num_empty_squares = c.to_digit(10).unwrap() as usize;
                    for _ in 0..num_empty_squares {
                        board.squares[rank * 8 + file] = None;
                        file += 1;
                    }
                }
                _ => {
                    let piece = match c {
                        'P' => Some(Piece::new(Color::White, Kind::Pawn)),
                        'N' => Some(Piece::new(Color::White, Kind::Knight)),
                        'B' => Some(Piece::new(Color::White, Kind::Bishop)),
                        'R' => Some(Piece::new(Color::White, Kind::Rook)),
                        'Q' => Some(Piece::new(Color::White, Kind::Queen)),
                        'K' => Some(Piece::new(Color::White, Kind::King)),
                        'p' => Some(Piece::new(Color::Black, Kind::Pawn)),
                        'n' => Some(Piece::new(Color::Black, Kind::Knight)),
                        'b' => Some(Piece::new(Color::Black, Kind::Bishop)),
                        'r' => Some(Piece::new(Color::Black, Kind::Rook)),
                        'q' => Some(Piece::new(Color::Black, Kind::Queen)),
                        'k' => Some(Piece::new(Color::Black, Kind::King)),
                        _ => None,
                    };
                    if let Some(piece) = piece {
                        board.squares[rank * 8 + file] = Some(piece);
                    }
                    file += 1;
                }
            }
        }
        board
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty_squares = 0;
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                None => empty_squares += 1,
                Some(p) => {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    fen.push(p.as_char());
                }
            }
            if (i + 1) % 8 == 0 {
                if empty_squares > 0 {
                    fen.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                if i != 63 {
                    fen.push('/');
                }
            }
        }
        fen
    }


    pub fn print_board(&self) -> String {
        let mut board_str = String::new();
        for sq in self.squares.iter().enumerate().rev() {
            let piece = match sq.1 {
                Some(p) => p.as_char(),
                None => '.',
            };
            board_str.push(piece);
            board_str.push(' ');
            if sq.0 % 8 == 0 {
                board_str.push('\n');
            }
        }

        board_str
    }

    pub fn play_move(&mut self, chess_move: &ChessMove, player: &Color) -> bool {
        //Castling requires no to or from squares
        if chess_move.castling.is_some() {
            return self.castle(player, &chess_move.castling.as_ref().unwrap());
        }
        let piece = &chess_move.piece;
        //Check if promotion is valid
        let promoted_piece = match &chess_move.promotion {
            Some(kind) => {
                if piece.kind != Kind::Pawn {
                    return false;
                }
                Some(Piece::new(player.clone(), kind.clone()))
            }
            None => None,
        };
        //If no to square is provided, move is invalid
        let to_square = match &chess_move.to {
            Some(square) => square,
            None => return false,
        };
        //From square can be provided; if not provided we need to find from candidate pieces
        let from_square: Option<Square> = match &chess_move.from_square {
            Some(square) => Some(square.clone()),
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
        if from_square.is_some() {
            let from_square = from_square.unwrap();
            if let Some(p) = self.squares[from_square.to_1d_arr_coordinates()] {
                if p.color != *player || p.kind != piece.kind {
                    return false;
                }
                return self.move_piece(&from_square, &to_square, promoted_piece);
            } else {
                return false;
            }
        }

        if chess_move.from_file.is_some() {
            let file = chess_move.from_file.unwrap();
            for rank in 0..8 {
                if let Some(p) = self.squares[Square::new(file, rank).to_1d_arr_coordinates()] {
                    if p.color == *player && p.kind == piece.kind {
                        return self.move_piece(
                            &Square::new(file, rank),
                            to_square,
                            promoted_piece,
                        );
                    }
                }
            }
        }

        if chess_move.from_rank.is_some() {
            let rank = chess_move.from_rank.unwrap();
            for file in 0..8 {
                if let Some(p) = self.squares[Square::new(file, rank).to_1d_arr_coordinates()] {
                    if p.color == *player && p.kind == piece.kind {
                        return self.move_piece(
                            &Square::new(file, rank),
                            to_square,
                            promoted_piece,
                        );
                    }
                }
            }
        }

        for square in self.squares.iter().enumerate() {
            if let Some(p) = square.1 {
                if p.color == *player && p.kind == piece.kind {
                    let candidate_square = Square::new_from_1d_arr_coordinates(square.0);
                    let candidate_moves = p.move_piece(candidate_square);
                    if candidate_moves.contains(&to_square) {
                        return self.move_piece(
                            &Square::new_from_1d_arr_coordinates(square.0),
                            to_square,
                            promoted_piece,
                        );
                    }
                }
            }
        }

        false
    }

    pub fn castle(&mut self, color: &Color, kind: &Kind) -> bool {
        let king_square = match color {
            Color::White => Square::from_san_str("e1"),
            Color::Black => Square::from_san_str("e8"),
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
                self.move_piece(&rook_square, &Square::from_san_str("h1").unwrap(), None);
                //Move the king from e1 to g1
                self.move_piece(&king_square, &Square::from_san_str("g1").unwrap(), None);
            }
            (Color::White, Kind::Queen) => {
                //Move the rook from a1 to d1
                self.move_piece(&rook_square, &Square::from_san_str("d1").unwrap(), None);
                //Move the king from e1 to c1
                self.move_piece(&king_square, &Square::from_san_str("c1").unwrap(), None);
            }
            (Color::Black, Kind::King) => {
                //Move the rook from h8 to f8
                self.move_piece(&rook_square, &Square::from_san_str("f8").unwrap(), None);
                //Move the king from e8 to g8
                self.move_piece(&king_square, &Square::from_san_str("g8").unwrap(), None);
            }
            (Color::Black, Kind::Queen) => {
                //Move the rook from a8 to d8
                self.move_piece(&rook_square, &Square::from_san_str("d8").unwrap(), None);
                //Move the king from e8 to c8
                self.move_piece(&king_square, &Square::from_san_str("c8").unwrap(), None);
            }
            _ => return false,
        }

        true
    }

    fn move_piece(
        &mut self,
        from_square: &Square,
        to_square: &Square,
        promoted_piece: Option<Piece>,
    ) -> bool {
        self.move_piece_coordinate(
            from_square.to_1d_arr_coordinates(),
            to_square.to_1d_arr_coordinates(),
            promoted_piece,
        )
    }

    fn move_piece_coordinate(
        &mut self,
        from_square: usize,
        to_square: usize,
        promoted_piece: Option<Piece>,
    ) -> bool {
        if from_square > 63 || to_square > 63 || from_square == to_square {
            return false;
        }

        self.squares[to_square] = self.squares[from_square];
        self.squares[from_square] = None;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_piece() {
        let mut board = Board::default();
        let from_square = Square::from_san_str("e2").unwrap();
        let to_square = Square::from_san_str("e4").unwrap();

        assert!(board.move_piece(&from_square, &to_square, None));
        assert_eq!(board.squares[from_square.to_1d_arr_coordinates()], None);
        assert_eq!(
            board.squares[to_square.to_1d_arr_coordinates()],
            Some(Piece::new(Color::White, Kind::Pawn))
        );
    }

    #[test]
    fn test_fen() {
        let mut board = Board::default();
        let fen = board.to_fen();
        assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        let from_square = Square::from_san_str("e2").unwrap();
        let to_square = Square::from_san_str("e4").unwrap();

        assert!(board.move_piece(&from_square, &to_square, None));
        let fen = board.to_fen();
        assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR");
    }

    #[test]
    fn test_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        assert_eq!(board.squares.len(), 64);
        assert_eq!(board.squares[0], Some(Piece::new(Color::Black, Kind::Rook)));
        assert_eq!(
            board.squares[63],
            Some(Piece::new(Color::White, Kind::Rook))
        );
    }
    // Add more tests for other board functionalities
}
