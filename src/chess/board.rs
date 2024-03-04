use super::pieces::Piece;

const ARRAY_REPEAT_VALUE: Option<Piece> = None;
pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
  pub fn new () -> Self {
    Board { squares: [ARRAY_REPEAT_VALUE; 64]}
  }
}

pub struct Square {
    file: char,
    rank: u8,
}

