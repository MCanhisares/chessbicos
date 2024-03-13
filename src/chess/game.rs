use super::{
    board::{Board, Square},
    pieces::{Color, Kind, Piece},
};
pub struct ChessMove {
    pub piece: Piece,
    pub from_square: Option<Square>,
    pub from_file: Option<usize>,
    pub from_rank: Option<usize>,
    pub to: Option<Square>,
    pub promotion: Option<Kind>,
    pub castling: Option<Kind>,
}

impl ChessMove {
    pub fn from_san(color: Color, san: &str) -> Option<ChessMove> {
        // O-O or O-O-O
        if san == "O-O" {
            return Some(ChessMove {
                piece: Piece::new(color, Kind::King),
                from_square: None,
                from_file: None,
                from_rank: None,
                to: None,
                promotion: None,
                castling: Some(Kind::King),
            });
        }
        if san == "O-O-O" {
            return Some(ChessMove {
                piece: Piece::new(color, Kind::King),
                from_square: None,
                from_file: None,
                from_rank: None,
                to: None,
                promotion: None,
                castling: Some(Kind::Queen),
            });
        }

        let chars: Vec<char> = san.chars().collect();
        if chars.len() < 2 {
            return None;
        }

        // Find what piece is moving
        let piece = match chars[0] {
            'N' => Piece::new(color, Kind::Knight),
            'B' => Piece::new(color, Kind::Bishop),
            'R' => Piece::new(color, Kind::Rook),
            'Q' => Piece::new(color, Kind::Queen),
            'K' => Piece::new(color, Kind::King),
            _ => Piece::new(color, Kind::Pawn),
        };

        // Capture
        // Bxc3 or exc8=Q
        if chars[1] == 'x' {
            if chars.len() < 4 {
                return None;
            }
            let mut capture_iter = chars[2..].iter();
            let to = Square::from_san(&mut capture_iter);

            let promotion = match capture_iter.next() {
                Some('=') => Some(Piece::from_char(*capture_iter.next()?)?.kind),
                _ => None,
            };
            return Some(ChessMove {
                piece,
                from_square: None,
                from_file: None,
                from_rank: None,
                to,
                promotion,
                castling: None,
            });
        }
        // Normal move
        // e4 or Nf3 or Nge2 or Qe2e3 or e8=Q
        // Pawn moves dont have piece prefix (e4)
        let first_index = if piece.kind == Kind::Pawn && chars[0].is_ascii_lowercase() {
            0
        } else {
            1
        };
        let mut chars_iter = chars[first_index..].iter();

        let first_square = Square::from_san(&mut chars_iter);
        let second_square = Square::from_san(&mut chars_iter);

        // First and second square found, order is from -> to
        // Qe2e3
        if first_square.is_some() && second_square.is_some() {
            //Promotion not possible
            return Some(ChessMove {
                piece,
                from_square: first_square,
                from_file: None,
                from_rank: None,
                to: second_square,
                promotion: None,
                castling: None,
            });
        }

        // Only first square is found, so no from square/file/rank
        if first_square.is_some() && second_square.is_none() {
            // Still check for promotion e8=Q
            let mut promotion_iter = chars[first_index + 2..].iter();
            let promotion = match promotion_iter.next() {
                Some('=') => Some(Piece::from_char(*promotion_iter.next()?)?.kind),
                _ => None,
            };
            return Some(ChessMove {
                piece,
                from_square: None,
                from_file: None,
                from_rank: None,
                to: first_square,
                promotion,
                castling: None,
            });
        }

        // Only formats left are Nge2 or R1e2
        // Find from file or rank
        let file_or_rank = chars[first_index];

        let mut to_square_iter = chars[first_index + 1..].iter();
        let to = Square::from_san(&mut to_square_iter);
        // R1e2
        if file_or_rank.is_numeric() {
            let rank = file_or_rank.to_digit(10).unwrap() as usize - 1; // 1 indexed
            return Some(ChessMove {
                piece,
                from_square: None,
                from_file: None,
                from_rank: Some(rank),
                to,
                promotion: None,
                castling: None,
            });
        }
        // Nge2
        let file = match file_or_rank {
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

        Some(ChessMove {
            piece,
            from_square: None,
            from_file: Some(file),
            from_rank: None,
            to,
            promotion: None,
            castling: None,
        })
    }
}
//Game state represented in FEN notation https://www.chessprogramming.org/Forsyth-Edwards_Notation
struct Game {
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
    halfmove: u32,
    // The number of the full moves in a game. It starts at 1, and is incremented after each Black's move.
    fullmove: u32,
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
            halfmove: 0,
            fullmove: 1,
        }
    }

    pub fn play_move(mut self, player: Color, san_move: &str) -> bool {
        if self.turn != player {
            return false;
        }
        let chess_move = ChessMove::from_san(player, san_move);
        if chess_move.is_none() {
            return false;
        }
        let mut board = self.board;
        let success = board.play_move(&chess_move.unwrap());
        if success {
            self.board = board
        }
        success
    }
}
