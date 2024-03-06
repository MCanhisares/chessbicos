use super::pieces::{Color, Kind, Piece};

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

                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,    

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
                Some(Piece::new(Color::Black, Kind::Rook)),],
        }
    }
}

pub struct Square {
    pub file: i8,
    pub rank: i8,
}

impl Square {
    pub fn new(file: i8, rank: i8) -> Square {
        Square { file, rank }
    }
    
    pub fn from_pgn(chars: &mut std::str::Chars) -> Option<Square> {
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
