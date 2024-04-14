use std::slice::Iter;
#[derive(PartialEq, Clone, Debug)]
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
        rank: 7 - (coordinates / 8),
      }
    }

    pub fn to_1d_arr_coordinates(&self) -> usize {
        (7 - self.rank) * 8 + self.file
    }

    pub fn to_san(&self) -> String {
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
    //a8 = { file: 0, rank: 7 } = 0
    //b8 = { file: 1, rank: 7 } = 1
    //c8 = { file: 2, rank: 7 } = 2
    //d8 = { file: 3, rank: 7 } = 3
    //e8 = { file: 4, rank: 7 } = 4
    //f8 = { file: 5, rank: 7 } = 5
    //g8 = { file: 6, rank: 7 } = 6
    //h8 = { file: 7, rank: 7 } = 7
    //a7 = { file: 0, rank: 6 } = 8
    //b7 = { file: 1, rank: 6 } = 9
    //c7 = { file: 2, rank: 6 } = 10
    //d7 = { file: 3, rank: 6 } = 11
    //e7 = { file: 4, rank: 6 } = 12
    //f7 = { file: 5, rank: 6 } = 13
    //g7 = { file: 6, rank: 6 } = 14
    //h7 = { file: 7, rank: 6 } = 15
    //a6 = { file: 0, rank: 5 } = 16
    //b6 = { file: 1, rank: 5 } = 17
    //c6 = { file: 2, rank: 5 } = 18
    //d6 = { file: 3, rank: 5 } = 19
    //e6 = { file: 4, rank: 5 } = 20
    //f6 = { file: 5, rank: 5 } = 21
    //g6 = { file: 6, rank: 5 } = 22
    //h6 = { file: 7, rank: 5 } = 23
    //a5 = { file: 0, rank: 4 } = 24
    //b5 = { file: 1, rank: 4 } = 25
    //c5 = { file: 2, rank: 4 } = 26
    //d5 = { file: 3, rank: 4 } = 27
    //e5 = { file: 4, rank: 4 } = 28
    //f5 = { file: 5, rank: 4 } = 29
    //g5 = { file: 6, rank: 4 } = 30
    //h5 = { file: 7, rank: 4 } = 31
    //a4 = { file: 0, rank: 3 } = 32
    //b4 = { file: 1, rank: 3 } = 33
    //c4 = { file: 2, rank: 3 } = 34
    //d4 = { file: 3, rank: 3 } = 35
    //e4 = { file: 4, rank: 3 } = 36
    //f4 = { file: 5, rank: 3 } = 37
    //g4 = { file: 6, rank: 3 } = 38
    //h4 = { file: 7, rank: 3 } = 39
    //a3 = { file: 0, rank: 2 } = 40
    //b3 = { file: 1, rank: 2 } = 41
    //c3 = { file: 2, rank: 2 } = 42
    //d3 = { file: 3, rank: 2 } = 43
    //e3 = { file: 4, rank: 2 } = 44
    //f3 = { file: 5, rank: 2 } = 45
    //g3 = { file: 6, rank: 2 } = 46
    //h3 = { file: 7, rank: 2 } = 47
    //a2 = { file: 0, rank: 1 } = 48
    //b2 = { file: 1, rank: 1 } = 49
    //c2 = { file: 2, rank: 1 } = 50
    //d2 = { file: 3, rank: 1 } = 51
    //e2 = { file: 4, rank: 1 } = 52
    //f2 = { file: 5, rank: 1 } = 53
    //g2 = { file: 6, rank: 1 } = 54
    //h2 = { file: 7, rank: 1 } = 55
    //a1 = { file: 0, rank: 0 } = 56
    //b1 = { file: 1, rank: 0 } = 57
    //c1 = { file: 2, rank: 0 } = 58
    //d1 = { file: 3, rank: 0 } = 59
    //e1 = { file: 4, rank: 0 } = 60
    //f1 = { file: 5, rank: 0 } = 61
    //g1 = { file: 6, rank: 0 } = 62
    //h1 = { file: 7, rank: 0 } = 63
    #[test]
    fn test_square_from_san() {
        let a8 = super::Square::from_san_str("a8").unwrap();
        assert_eq!(a8.file, 0);
        assert_eq!(a8.rank, 7);
        let b1 = super::Square::from_san_str("b1").unwrap();
        assert_eq!(b1.file, 1);
        assert_eq!(b1.rank, 0);
        let h1 = super::Square::from_san_str("h1").unwrap();
        assert_eq!(h1.file, 7);
        assert_eq!(h1.rank, 0);
        let jiberrish = super::Square::from_san_str("jiberrish");
        assert_eq!(jiberrish, None);
    }
    #[test]
    fn test_square_to_1d_arr_coordinates() {
        let square = super::Square::new(0, 0);
        assert_eq!(square.to_1d_arr_coordinates(), 56);
        let square = super::Square::new(1, 0);
        assert_eq!(square.to_1d_arr_coordinates(), 57);
        let square = super::Square::new(0, 1);
        assert_eq!(square.to_1d_arr_coordinates(), 48);
        let square = super::Square::new(1, 1);
        assert_eq!(square.to_1d_arr_coordinates(), 49);
        let square = super::Square::from_san_str("h1").unwrap();
        assert_eq!(square.to_1d_arr_coordinates(), 63);
        let square = super::Square::from_san_str("e4").unwrap();
        assert_eq!(square.to_1d_arr_coordinates(), 36);
    }

    #[test]
    fn test_square_new_from_1d_arr_coordinates() {
        let square = super::Square::new_from_1d_arr_coordinates(56);
        assert_eq!(square.file, 0);
        assert_eq!(square.rank, 0);
        let square = super::Square::new_from_1d_arr_coordinates(57);
        assert_eq!(square.file, 1);
        assert_eq!(square.rank, 0);
        let square = super::Square::new_from_1d_arr_coordinates(48);
        assert_eq!(square.file, 0);
        assert_eq!(square.rank, 1);
        let square = super::Square::new_from_1d_arr_coordinates(49);
        assert_eq!(square.file, 1);
        assert_eq!(square.rank, 1);
        let square = super::Square::new_from_1d_arr_coordinates(0);
        assert_eq!(square.file, 0);
        assert_eq!(square.rank, 7);
        let square = super::Square::new_from_1d_arr_coordinates(44);
        assert_eq!(square.file, 4);
        assert_eq!(square.rank, 2);
    }
}
