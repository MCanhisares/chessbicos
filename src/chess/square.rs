use std::slice::Iter;
#[derive(PartialEq, Clone)]
pub struct Square {
    pub file: usize,
    pub rank: usize,
}

impl Square {
    pub fn new(file: usize, rank: usize) -> Square {
        Square { file, rank }
    }

    pub fn new_from_1d_arr_coordinates(coordinates: usize) -> Square {
        Square {
            file: coordinates % 8,
            rank: coordinates / 8,
        }
    }

    pub fn to_1d_arr_coordinates(&self) -> usize {
        self.rank * 8 + self.file
    }

    pub fn to_san (&self) -> String {
        let file = match self.file {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Invalid file"),
        };
        let rank = match self.rank {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!("Invalid rank"),
        };
        format!("{}{}", file, rank)
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_square_from_san() {
        let square = super::Square::from_san_str("b1").unwrap();
        assert_eq!(square.file, 1);
        assert_eq!(square.rank, 0);
    }
    #[test]
    fn test_square_to_1d_arr_coordinates() {
        let square = super::Square::new(1, 0);
        assert_eq!(square.to_1d_arr_coordinates(), 1);
    }

    #[test]
    fn test_square_new_from_1d_arr_coordinates() {
        let square = super::Square::new_from_1d_arr_coordinates(1);
        assert_eq!(square.file, 1);
        assert_eq!(square.rank, 0);
    }
}
