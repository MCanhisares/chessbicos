pub struct Piece {
    color: Color,
    kind: Kind,
}

impl Piece {
    pub fn new(color: Color, kind: Kind) -> Piece {
        Piece { color, kind }
    }
    pub fn as_char(&self) -> char {
        let color = self.color.as_char();
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
}

#[derive(PartialEq)]
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

pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
