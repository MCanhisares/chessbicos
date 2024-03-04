use super::{
    board::{Board, Square},
    pieces::{Color, Kind, Piece},
};

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
            board: Board::new(),
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
}
