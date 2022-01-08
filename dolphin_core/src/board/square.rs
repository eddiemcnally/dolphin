use crate::board::bitboard::Bitboard;
use crate::board::file::*;
use crate::board::rank::*;
use std::fmt;
use std::slice::Iter;

use super::types::ToInt;

pub const NUM_SQUARES: usize = 64;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Square(u8);

pub const SQUARE_A1: Square = Square(0);
pub const SQUARE_B1: Square = Square(1);
pub const SQUARE_C1: Square = Square(2);
pub const SQUARE_D1: Square = Square(3);
pub const SQUARE_E1: Square = Square(4);
pub const SQUARE_F1: Square = Square(5);
pub const SQUARE_G1: Square = Square(6);
pub const SQUARE_H1: Square = Square(7);

pub const SQUARE_A2: Square = Square(8);
pub const SQUARE_B2: Square = Square(9);
pub const SQUARE_C2: Square = Square(10);
pub const SQUARE_D2: Square = Square(11);
pub const SQUARE_E2: Square = Square(12);
pub const SQUARE_F2: Square = Square(13);
pub const SQUARE_G2: Square = Square(14);
pub const SQUARE_H2: Square = Square(15);

pub const SQUARE_A3: Square = Square(16);
pub const SQUARE_B3: Square = Square(17);
pub const SQUARE_C3: Square = Square(18);
pub const SQUARE_D3: Square = Square(19);
pub const SQUARE_E3: Square = Square(20);
pub const SQUARE_F3: Square = Square(21);
pub const SQUARE_G3: Square = Square(22);
pub const SQUARE_H3: Square = Square(23);

pub const SQUARE_A4: Square = Square(24);
pub const SQUARE_B4: Square = Square(25);
pub const SQUARE_C4: Square = Square(26);
pub const SQUARE_D4: Square = Square(27);
pub const SQUARE_E4: Square = Square(28);
pub const SQUARE_F4: Square = Square(29);
pub const SQUARE_G4: Square = Square(30);
pub const SQUARE_H4: Square = Square(31);

pub const SQUARE_A5: Square = Square(32);
pub const SQUARE_B5: Square = Square(33);
pub const SQUARE_C5: Square = Square(34);
pub const SQUARE_D5: Square = Square(35);
pub const SQUARE_E5: Square = Square(36);
pub const SQUARE_F5: Square = Square(37);
pub const SQUARE_G5: Square = Square(38);
pub const SQUARE_H5: Square = Square(39);

pub const SQUARE_A6: Square = Square(40);
pub const SQUARE_B6: Square = Square(41);
pub const SQUARE_C6: Square = Square(42);
pub const SQUARE_D6: Square = Square(43);
pub const SQUARE_E6: Square = Square(44);
pub const SQUARE_F6: Square = Square(45);
pub const SQUARE_G6: Square = Square(46);
pub const SQUARE_H6: Square = Square(47);

pub const SQUARE_A7: Square = Square(48);
pub const SQUARE_B7: Square = Square(49);
pub const SQUARE_C7: Square = Square(50);
pub const SQUARE_D7: Square = Square(51);
pub const SQUARE_E7: Square = Square(52);
pub const SQUARE_F7: Square = Square(53);
pub const SQUARE_G7: Square = Square(54);
pub const SQUARE_H7: Square = Square(55);

pub const SQUARE_A8: Square = Square(56);
pub const SQUARE_B8: Square = Square(57);
pub const SQUARE_C8: Square = Square(58);
pub const SQUARE_D8: Square = Square(59);
pub const SQUARE_E8: Square = Square(60);
pub const SQUARE_F8: Square = Square(61);
pub const SQUARE_G8: Square = Square(62);
pub const SQUARE_H8: Square = Square(63);

impl ToInt for Square {
    fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Square {
    pub fn new(num: u8) -> Option<Square> {
        if num <= SQUARE_H8.0 {
            return Some(Square(num));
        }
        None
    }

    pub fn square_plus_1_rank(self) -> Option<Square> {
        match self.rank() {
            RANK_8 => None,
            _ => Square::new(self.0 + 8),
        }
    }

    pub fn square_minus_1_rank(self) -> Option<Square> {
        match self.rank() {
            RANK_1 => None,
            _ => Square::new(self.0 - 8),
        }
    }

    pub fn square_plus_2_ranks(self) -> Option<Square> {
        match self.rank() {
            RANK_7 | RANK_8 => None,
            _ => Square::new(self.0 + 16),
        }
    }

    pub fn square_minus_2_ranks(self) -> Option<Square> {
        match self.rank() {
            RANK_1 | RANK_2 => None,
            _ => Square::new(self.0 - 16),
        }
    }

    pub fn rank(self) -> Rank {
        Rank::new(self.rank_as_u8()).unwrap()
    }

    pub fn file(self) -> File {
        File::new(self.file_as_u8()).unwrap()
    }

    pub fn from_rank_file(rank: Rank, file: File) -> Square {
        let sq = (rank.to_u8() << 3) + file.to_u8();
        Square::new(sq).unwrap()
    }

    pub fn get_square_as_bb(self) -> Bitboard {
        Bitboard::new(0x01u64 << (self.to_usize()))
    }

    pub fn get_from_string(square_str: &str) -> Option<Square> {
        let f = square_str.chars().next().unwrap();
        let r = square_str.chars().nth(1).unwrap();

        if let Some(file) = File::from_char(f) {
            if let Some(rank) = Rank::from_char(r) {
                return Some(Square::from_rank_file(rank, file));
            }
        }
        None
    }

    pub const fn same_rank(self, other: Square) -> bool {
        self.rank_as_u8() == other.rank_as_u8()
    }

    pub const fn same_file(self, other: Square) -> bool {
        self.file_as_u8() == other.file_as_u8()
    }

    const fn rank_as_u8(self) -> u8 {
        self.0 >> 3
    }
    const fn file_as_u8(self) -> u8 {
        self.0 & 0x07
        //(self.0 % 8) as u8
    }
}

impl Default for Square {
    fn default() -> Square {
        SQUARE_A1
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        let rank = self.rank();
        let file = self.file();

        debug_str.push_str(&format!("{}", file.to_char()));
        debug_str.push_str(&format!("{}", rank.to_char()));

        write!(f, "{}", debug_str)
    }
}

#[rustfmt::skip]

pub fn iterator() -> Iter<'static, Square> {
    static SQUARES: [Square; NUM_SQUARES] = [
        SQUARE_A1, SQUARE_B1, SQUARE_C1, SQUARE_D1, SQUARE_E1, SQUARE_F1, SQUARE_G1, SQUARE_H1, 
        SQUARE_A2, SQUARE_B2, SQUARE_C2, SQUARE_D2, SQUARE_E2, SQUARE_F2, SQUARE_G2, SQUARE_H2, 
        SQUARE_A3, SQUARE_B3, SQUARE_C3, SQUARE_D3, SQUARE_E3, SQUARE_F3, SQUARE_G3, SQUARE_H3, 
        SQUARE_A4, SQUARE_B4, SQUARE_C4, SQUARE_D4, SQUARE_E4, SQUARE_F4, SQUARE_G4, SQUARE_H4, 
        SQUARE_A5, SQUARE_B5, SQUARE_C5, SQUARE_D5, SQUARE_E5, SQUARE_F5, SQUARE_G5, SQUARE_H5, 
        SQUARE_A6, SQUARE_B6, SQUARE_C6, SQUARE_D6, SQUARE_E6, SQUARE_F6, SQUARE_G6, SQUARE_H6, 
        SQUARE_A7, SQUARE_B7, SQUARE_C7, SQUARE_D7, SQUARE_E7, SQUARE_F7, SQUARE_G7, SQUARE_H7, 
        SQUARE_A8, SQUARE_B8, SQUARE_C8, SQUARE_D8, SQUARE_E8, SQUARE_F8, SQUARE_G8, SQUARE_H8,
    ];
    SQUARES.iter()
}

#[cfg(test)]
pub mod tests {
    use super::Square;
    use crate::board::file::*;
    use crate::board::rank::*;
    use crate::board::square::*;

    #[test]
    pub fn rank_from_square() {
        assert!(SQUARE_A1.rank() == RANK_1);
        assert!(SQUARE_B2.rank() == RANK_2);
        assert!(SQUARE_H3.rank() == RANK_3);
        assert!(SQUARE_G4.rank() == RANK_4);
        assert!(SQUARE_A5.rank() == RANK_5);
        assert!(SQUARE_C6.rank() == RANK_6);
        assert!(SQUARE_D7.rank() == RANK_7);
        assert!(SQUARE_F8.rank() == RANK_8);
    }

    #[test]
    pub fn file_from_square() {
        assert!(SQUARE_A1.file() == FILE_A);
        assert!(SQUARE_E5.file() == FILE_E);
        assert!(SQUARE_D4.file() == FILE_D);
    }

    #[test]
    pub fn convert_square_to_uint() {
        let sq: Square = SQUARE_B1;
        let num = sq.to_usize();

        assert_eq!(num, 1);

        let sq1: Square = SQUARE_D7;
        let num1 = sq1.to_usize();

        assert_eq!(num1, 51);
    }

    #[test]
    pub fn square_from_rank_and_file() {
        let map = super::iterator();
        for square in map {
            let rank = square.rank();
            let file = square.file();
            let sq = Square::from_rank_file(rank, file);
            assert_eq!(*square, sq);
        }
        assert!(Square::from_rank_file(RANK_3, FILE_G) == SQUARE_G3);
    }

    #[test]
    pub fn square_from_string() {
        let map = super::iterator();
        for square in map {
            let str = square.to_string();
            let sq = Square::get_from_string(&str);
            match sq {
                Some(_) => assert_eq!(*square, sq.unwrap()),
                None => panic!("Unexpected square"),
            }
        }
    }

    #[test]
    pub fn square_from_rank_file() {
        assert!(Square::from_rank_file(RANK_1, FILE_A) == SQUARE_A1);
        assert!(Square::from_rank_file(RANK_8, FILE_A) == SQUARE_A8);

        assert!(Square::from_rank_file(RANK_1, FILE_H) == SQUARE_H1);
        assert!(Square::from_rank_file(RANK_8, FILE_H) == SQUARE_H8);
    }

    #[test]
    pub fn square_values() {
        for (i, square) in super::iterator().enumerate() {
            assert_eq!(square.0, i as u8);
        }
    }
}
