use super::board::Square;

#[derive(PartialEq, Clone, Copy)]
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
            if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                moves.push(Square::new(new_file as usize, new_rank as usize));
                if increment {
                    new_file += dx;
                    new_rank += dy;
                }
            }
        }
        moves
    }

    pub fn move_piece(&self, square: Square) -> Vec<Square> {
        let mut moves: Vec<Square> = Vec::new();
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
                let bishop_moves = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
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
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
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

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn as_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    fn from_char(c: char) -> Option<Color> {
        match c {
            'w' => Some(Color::White),
            'b' => Some(Color::Black),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

