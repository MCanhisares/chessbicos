use super::square::Square;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}
impl Piece {
    pub fn new(color: Color, kind: Kind) -> Piece {
        Piece { color, kind }
    }
    pub fn as_char(&self) -> char {
        let kind = match self.kind {
            Kind::Pawn => 'p',
            Kind::Knight => 'n',
            Kind::Bishop => 'b',
            Kind::Rook => 'r',
            Kind::Queen => 'q',
            Kind::King => 'k',
        };
        if self.color == Color::White {
            kind.to_ascii_uppercase()
        } else {
            kind
        }
    }

    pub fn from_char(c: char) -> Option<Piece> {
        let color = if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        let kind = match c.to_ascii_lowercase() {
            'p' => Kind::Pawn,
            'n' => Kind::Knight,
            'b' => Kind::Bishop,
            'r' => Kind::Rook,
            'q' => Kind::Queen,
            'k' => Kind::King,
            _ => return None,
        };
        Some(Piece::new(color, kind))
    }

    fn generate_moves(
        &self,
        candidate_moves: Vec<(i8, i8)>,
        square: Square,
        increment: bool,
    ) -> Vec<Square> {
        let mut moves: Vec<Square> = Vec::new();
        for (dx, dy) in candidate_moves.iter() {
            let mut new_file = square.file as i8 + dx;
            let mut new_rank = square.rank as i8 + dy;

            // Check if the new position is within the board
            if increment {
                while new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                    moves.push(Square::new(new_file as usize, new_rank as usize));
                    new_file += dx;
                    new_rank += dy;
                }
            } else if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                moves.push(Square::new(new_file as usize, new_rank as usize));
            }
        }
        moves
    }

    pub fn move_piece(&self, square: Square) -> Vec<Square> {
        let file = square.file;
        let rank = square.rank;
        match self.kind {
            Kind::Pawn => {
                let mut moves: Vec<Square> = Vec::new();

                // White pawn moves
                if self.color == Color::White {
                    // Move one square forward
                    if rank < 7 {
                        moves.push(Square::new(file, rank + 1));
                    }

                    // Move two squares forward from starting position
                    if rank == 1 {
                        moves.push(Square::new(file, rank + 2));
                    }

                    // Capture diagonally to the right
                    if file < 7 {
                        moves.push(Square::new(file + 1, rank + 1));
                    }

                    // Capture diagonally to the left
                    if file > 0 {
                        moves.push(Square::new(file - 1, rank + 1));
                    }
                }

                // Black pawn moves
                if self.color == Color::Black {
                    // Move one square forward
                    if rank > 0 {
                        moves.push(Square::new(file, rank - 1));
                    }

                    // Move two squares forward from starting position
                    if rank == 6 {
                        moves.push(Square::new(file, rank - 2));
                    }

                    // Capture diagonally to the right
                    if file < 7 {
                        moves.push(Square::new(file + 1, rank - 1));
                    }

                    // Capture diagonally to the left
                    if file > 0 {
                        moves.push(Square::new(file - 1, rank - 1));
                    }
                }
                moves
            }
            Kind::Knight => {
                // Possible knight moves
                let knight_moves = [
                    (2, 1),
                    (1, 2),
                    (-1, 2),
                    (-2, 1),
                    (-2, -1),
                    (-1, -2),
                    (1, -2),
                    (2, -1),
                ];
                self.generate_moves(knight_moves.to_vec(), square, false)
            }
            Kind::Bishop => {
                // Bishop moves
                let bishop_moves = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
                self.generate_moves(bishop_moves.to_vec(), square, true)
            }
            Kind::Rook => {
                // Rook moves
                let rook_moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                self.generate_moves(rook_moves.to_vec(), square, true)
            }
            Kind::Queen => {
                // Queen moves
                let queen_moves = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ];

                self.generate_moves(queen_moves.to_vec(), square, true)
            }
            Kind::King => {
                // King moves
                let king_moves = [
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                ];
                self.generate_moves(king_moves.to_vec(), square, false)
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::White => "w",
            Color::Black => "b",
        }
    }

    pub fn from_str(s: &str) -> Option<Color> {
        match s {
            "w" => Some(Color::White),
            "b" => Some(Color::Black),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_as_char() {
        let piece = Piece::new(Color::White, Kind::Pawn);
        assert_eq!(piece.as_char(), 'P');
    }

    #[test]
    fn test_piece_from_char() {
        let piece = Piece::from_char('p').unwrap();
        assert_eq!(piece, Piece::new(Color::Black, Kind::Pawn));
    }

    #[test]
    fn test_piece_move_piece_pawn() {
        let piece = Piece::new(Color::White, Kind::Pawn);
        let square = Square::new(3, 3);
        let moves = piece.move_piece(square);
        assert_eq!(moves.len(), 3);
        assert_eq!(moves[0].file, 3);
        assert_eq!(moves[0].rank, 4);
        assert_eq!(moves[1].file, 4);
        assert_eq!(moves[1].rank, 4);
        assert_eq!(moves[2].file, 2);
        assert_eq!(moves[2].rank, 4);
    }

    #[test]
    fn test_piece_move_piece_knight() {
        let piece = Piece::new(Color::White, Kind::Knight);
        let square = Square::new(3, 3);
        let moves = piece.move_piece(square);
        assert_eq!(moves.len(), 8);
        assert_eq!(moves[0].file, 5);
        assert_eq!(moves[0].rank, 4);
        assert_eq!(moves[1].file, 4);
        assert_eq!(moves[1].rank, 5);
        assert_eq!(moves[2].file, 2);
        assert_eq!(moves[2].rank, 5);
        assert_eq!(moves[3].file, 1);
        assert_eq!(moves[3].rank, 4);
        assert_eq!(moves[4].file, 1);
        assert_eq!(moves[4].rank, 2);
        assert_eq!(moves[5].file, 2);
        assert_eq!(moves[5].rank, 1);
        assert_eq!(moves[6].file, 4);
        assert_eq!(moves[6].rank, 1);
        assert_eq!(moves[7].file, 5);
        assert_eq!(moves[7].rank, 2);
    }

    #[test]
    fn test_piece_move_piece_bishop() {
        let piece = Piece::new(Color::White, Kind::Bishop);
        let square = Square::new(3, 3);
        let moves = piece.move_piece(square);
        assert_eq!(moves.len(), 13);
        assert_eq!(moves[0].file, 4);
        assert_eq!(moves[0].rank, 4);
        assert_eq!(moves[1].file, 5);
        assert_eq!(moves[1].rank, 5);
        assert_eq!(moves[2].file, 6);
        assert_eq!(moves[2].rank, 6);
        assert_eq!(moves[3].file, 7);
        assert_eq!(moves[3].rank, 7);
        assert_eq!(moves[4].file, 2);
        assert_eq!(moves[4].rank, 2);
        assert_eq!(moves[5].file, 1);
        assert_eq!(moves[5].rank, 1);
        assert_eq!(moves[6].file, 0);
        assert_eq!(moves[6].rank, 0);
        assert_eq!(moves[7].file, 4);
        assert_eq!(moves[7].rank, 2);
        assert_eq!(moves[8].file, 5);
        assert_eq!(moves[8].rank, 1);
        assert_eq!(moves[9].file, 6);
        assert_eq!(moves[9].rank, 0);
        assert_eq!(moves[10].file, 2);
        assert_eq!(moves[10].rank, 4);
        assert_eq!(moves[11].file, 1);
        assert_eq!(moves[11].rank, 5);
        assert_eq!(moves[12].file, 0);
        assert_eq!(moves[12].rank, 6);
    }

    #[test]
    fn test_piece_move_piece_rook() {
        let piece = Piece::new(Color::White, Kind::Rook);
        let square = Square::new(3, 3);
        let moves = piece.move_piece(square);
        assert_eq!(moves.len(), 14);
        assert_eq!(moves[0].file, 4);
        assert_eq!(moves[0].rank, 3);
        assert_eq!(moves[1].file, 5);
        assert_eq!(moves[1].rank, 3);
        assert_eq!(moves[2].file, 6);
        assert_eq!(moves[2].rank, 3);
        assert_eq!(moves[3].file, 7);
        assert_eq!(moves[3].rank, 3);
        assert_eq!(moves[4].file, 2);
        assert_eq!(moves[4].rank, 3);
        assert_eq!(moves[5].file, 1);
        assert_eq!(moves[5].rank, 3);
        assert_eq!(moves[6].file, 0);
        assert_eq!(moves[6].rank, 3);
        assert_eq!(moves[7].file, 3);
        assert_eq!(moves[7].rank, 4);
        assert_eq!(moves[8].file, 3);
        assert_eq!(moves[8].rank, 5);
        assert_eq!(moves[9].file, 3);
        assert_eq!(moves[9].rank, 6);
        assert_eq!(moves[10].file, 3);
        assert_eq!(moves[10].rank, 7);
        assert_eq!(moves[11].file, 3);
        assert_eq!(moves[11].rank, 2);
        assert_eq!(moves[12].file, 3);
        assert_eq!(moves[12].rank, 1);
        assert_eq!(moves[13].file, 3);
        assert_eq!(moves[13].rank, 0);
    }

    #[test]
    fn test_piece_move_piece_queen() {
        let piece = Piece::new(Color::White, Kind::Queen);
        let square = Square::new(3, 3);
        let moves = piece.move_piece(square);
        assert_eq!(moves.len(), 27);
        assert_eq!(moves[0].file, 4);
        assert_eq!(moves[0].rank, 3);
        assert_eq!(moves[1].file, 5);
        assert_eq!(moves[1].rank, 3);
        assert_eq!(moves[2].file, 6);
        assert_eq!(moves[2].rank, 3);
        assert_eq!(moves[3].file, 7);
        assert_eq!(moves[3].rank, 3);
        assert_eq!(moves[4].file, 2);
        assert_eq!(moves[4].rank, 3);
        assert_eq!(moves[5].file, 1);
        assert_eq!(moves[5].rank, 3);
        assert_eq!(moves[6].file, 0);
        assert_eq!(moves[6].rank, 3);
        assert_eq!(moves[7].file, 3);
        assert_eq!(moves[7].rank, 4);
        assert_eq!(moves[8].file, 3);
        assert_eq!(moves[8].rank, 5);
        assert_eq!(moves[9].file, 3);
        assert_eq!(moves[9].rank, 6);
        assert_eq!(moves[10].file, 3);
        assert_eq!(moves[10].rank, 7);
        assert_eq!(moves[11].file, 3);
        assert_eq!(moves[11].rank, 2);
        assert_eq!(moves[12].file, 3);
        assert_eq!(moves[12].rank, 1);
        assert_eq!(moves[13].file, 3);
        assert_eq!(moves[13].rank, 0);
        assert_eq!(moves[14].file, 4);
        assert_eq!(moves[14].rank, 4);
        assert_eq!(moves[15].file, 5);
        assert_eq!(moves[15].rank, 5);
        assert_eq!(moves[16].file, 6);
        assert_eq!(moves[16].rank, 6);
        assert_eq!(moves[17].file, 7);
        assert_eq!(moves[17].rank, 7);
        assert_eq!(moves[18].file, 2);
        assert_eq!(moves[18].rank, 2);
        assert_eq!(moves[19].file, 1);
        assert_eq!(moves[19].rank, 1);
        assert_eq!(moves[20].file, 0);
        assert_eq!(moves[20].rank, 0);
        assert_eq!(moves[21].file, 4);
        assert_eq!(moves[21].rank, 2);
        assert_eq!(moves[22].file, 5);
        assert_eq!(moves[22].rank, 1);
        assert_eq!(moves[23].file, 6);
        assert_eq!(moves[23].rank, 0);
        assert_eq!(moves[24].file, 2);
        assert_eq!(moves[24].rank, 4);
        assert_eq!(moves[25].file, 1);
        assert_eq!(moves[25].rank, 5);
        assert_eq!(moves[26].file, 0);
        assert_eq!(moves[26].rank, 6);
    }

    #[test]
    fn test_piece_move_piece_king() {
        let piece = Piece::new(Color::White, Kind::King);
        let square = Square::new(3, 3);
        let moves = piece.move_piece(square);
        assert_eq!(moves.len(), 8);
        //  (1, 1),
        assert_eq!(moves[0].file, 4);
        assert_eq!(moves[0].rank, 4);
        //  (1, -1),
        assert_eq!(moves[1].file, 4);
        assert_eq!(moves[1].rank, 2);
        //  (-1, 1),
        assert_eq!(moves[2].file, 2);
        assert_eq!(moves[2].rank, 4);
        //  (-1, -1),
        assert_eq!(moves[3].file, 2);
        assert_eq!(moves[3].rank, 2);
        //  (1, 0),
        assert_eq!(moves[4].file, 4);
        assert_eq!(moves[4].rank, 3);
        //  (-1, 0),
        assert_eq!(moves[5].file, 2);
        assert_eq!(moves[5].rank, 3);
        //  (0, 1),
        assert_eq!(moves[6].file, 3);
        assert_eq!(moves[6].rank, 4);
        //  (0, -1),
        assert_eq!(moves[7].file, 3);
        assert_eq!(moves[7].rank, 2);
    }
}
