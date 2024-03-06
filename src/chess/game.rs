use super::{
    board::{Board, Square},
    pieces::{Color, Kind, Piece},
};
struct ChessMove {
    piece: Piece,
    to: Square,
    promotion: Option<Kind>,
    castling: bool,
}

impl ChessMove {
    pub fn from_pgn(color: Color, pgn: &str) -> Option<ChessMove> {
        return {
            let mut chars = pgn.chars();
            let piece = match chars.next() {
                Some('N') => Piece::new(color, Kind::Knight),
                Some('B') => Piece::new(color, Kind::Bishop),
                Some('R') => Piece::new(color, Kind::Rook),
                Some('Q') => Piece::new(color, Kind::Queen),
                Some('K') => Piece::new(color, Kind::King),
                Some(_) => Piece::new(color, Kind::Pawn),
                _ => return None,
            };
            let to = Square::from_pgn(&mut chars)?;
            let promotion = match chars.next() {
                Some('=') => Some(Piece::from_char(chars.next()?)?.kind),
                _ => None,
            };
            let castling = match chars.next() {
                Some('O') => match chars.next() {
                    Some('-') => true,
                    _ => false,
                },
                _ => false,
            };
            Some(ChessMove {
                piece,
                to,
                promotion,
                castling,
            })
        };
    }
}
//Game state represented in FEN notation https://www.chessprogramming.org/Forsyth-Edwards_Notation
struct Game {
    board: Board,
    turn: Color,
    // If neither side can castle, the symbol '-' is used, otherwise each of four individual castling rights for king and queen castling for both sides are indicated by a sequence of one to four letters.
    castling: Option<(Option<Piece>, Option<Piece>, Option<Piece>, Option<Piece>)>,
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

    pub fn play_move(self, player: Color, pgn_move: &str) -> bool {
        if self.turn != player {
            return false;
        }
        let chess_move = ChessMove::from_pgn(player, pgn_move);
        return true;
    }
}
