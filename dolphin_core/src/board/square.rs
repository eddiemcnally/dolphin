use crate::board::file::File;
use crate::board::rank::Rank;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::fmt;

use Square::a1;

pub const NUM_SQUARES: usize = 64;

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Clone, Copy, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(u64)]
pub enum Square {
    a1, b1, c1, d1, e1, f1, g1, h1,
    a2, b2, c2, d2, e2, f2, g2, h2, 
    a3, b3, c3, d3, e3, f3, g3, h3, 
    a4, b4, c4, d4, e4, f4, g4, h4,
    a5, b5, c5, d5, e5, f5, g5, h5,
    a6, b6, c6, d6, e6, f6, g6, h6,
    a7, b7, c7, d7, e7, f7, g7, h7,
    a8, b8, c8, d8, e8, f8, g8, h8,
}

impl Square {
    pub fn square_plus_1_rank(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank8 => None,
            _ => {
                let s = self as u64 + 8;
                Square::from_num(s)
            }
        }
    }

    pub fn square_minus_1_rank(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank1 => None,
            _ => {
                let s = self as u64 - 8;
                Square::from_num(s)
            }
        }
    }

    pub fn square_plus_2_ranks(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank7 => None,
            Rank::Rank8 => None,
            _ => {
                let s = self as u64 + 16;
                Square::from_num(s)
            }
        }
    }

    pub fn square_minus_2_ranks(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank1 => None,
            Rank::Rank2 => None,
            _ => {
                let s = self as u64 - 16;
                Square::from_num(s)
            }
        }
    }

    pub fn rank(self) -> Rank {
        let rank_num = self.rank_as_u64();
        Rank::from_num(rank_num).unwrap()
    }

    pub fn file(self) -> File {
        let file_num = self.file_as_u64();
        File::from_num(file_num).unwrap()
    }

    pub fn get_square(rank: Rank, file: File) -> Square {
        let sq = (((rank as u64) << 3) + file as u64) as u64;
        Square::from_num(sq).unwrap()
    }

    pub const fn get_square_as_bb(self) -> u64 {
        0x01u64 << (self.offset())
    }

    pub fn get_from_string(square_str: &str) -> Option<Square> {
        let f = square_str.chars().next().unwrap();
        let r = square_str.chars().nth(1).unwrap();

        if let Some(file) = File::from_char(f) {
            if let Some(rank) = Rank::from_char(r) {
                return Some(Square::get_square(rank, file));
            }
        }
        None
    }

    pub fn from_num(num: u64) -> Option<Square> {
        let sq = Square::try_from(num);
        match sq {
            Ok(pce) => Some(pce),
            _ => None,
        }
    }

    pub const fn same_rank(self, other: Square) -> bool {
        self.rank_as_u64() == other.rank_as_u64()
    }

    pub const fn same_file(self, other: Square) -> bool {
        self.file_as_u64() == other.file_as_u64()
    }

    pub const fn offset(self) -> usize {
        self as usize
    }

    const fn rank_as_u64(self) -> u64 {
        self as u64 >> 3
    }
    const fn file_as_u64(self) -> u64 {
        (self as u64 % 8) as u64
    }
}

impl Default for Square {
    fn default() -> Square {
        a1
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
pub const SQUARES: &[Square] = &[
    Square::a1, Square::b1, Square::c1, Square::d1, Square::e1, Square::f1, Square::g1, Square::h1, 
    Square::a2, Square::b2, Square::c2, Square::d2, Square::e2, Square::f2, Square::g2, Square::h2, 
    Square::a3, Square::b3, Square::c3, Square::d3, Square::e3, Square::f3, Square::g3, Square::h3, 
    Square::a4, Square::b4, Square::c4, Square::d4, Square::e4, Square::f4, Square::g4, Square::h4, 
    Square::a5, Square::b5, Square::c5, Square::d5, Square::e5, Square::f5, Square::g5, Square::h5, 
    Square::a6, Square::b6, Square::c6, Square::d6, Square::e6, Square::f6, Square::g6, Square::h6, 
    Square::a7, Square::b7, Square::c7, Square::d7, Square::e7, Square::f7, Square::g7, Square::h7, 
    Square::a8, Square::b8, Square::c8, Square::d8, Square::e8, Square::f8, Square::g8, Square::h8,
];

#[cfg(test)]
pub mod tests {
    use super::Square;
    use crate::board::file::File;
    use crate::board::rank::Rank;

    #[test]
    pub fn rank_from_square() {
        assert!(Square::a1.rank() == Rank::Rank1);
        assert!(Square::b2.rank() == Rank::Rank2);
        assert!(Square::h3.rank() == Rank::Rank3);
        assert!(Square::g4.rank() == Rank::Rank4);
        assert!(Square::a5.rank() == Rank::Rank5);
        assert!(Square::c6.rank() == Rank::Rank6);
        assert!(Square::d7.rank() == Rank::Rank7);
        assert!(Square::f8.rank() == Rank::Rank8);
    }

    #[test]
    pub fn file_from_square() {
        assert!(Square::a1.file() == File::FileA);
        assert!(Square::e5.file() == File::FileE);
        assert!(Square::d4.file() == File::FileD);
    }

    #[test]
    pub fn convert_square_to_uint() {
        let sq: Square = Square::b1;
        let num: u16 = sq as u16;

        assert_eq!(num, 1);

        let sq1: Square = Square::d7;
        let num1: u16 = sq1 as u16;

        assert_eq!(num1, 51);
    }

    #[test]
    pub fn square_from_rank_and_file() {
        let map = super::SQUARES;
        for square in map {
            let rank = square.rank();
            let file = square.file();
            let sq = Square::get_square(rank, file);
            assert_eq!(*square, sq);
        }
        assert!(Square::get_square(Rank::Rank3, File::FileG) == Square::g3);
    }

    #[test]
    pub fn square_from_string() {
        let map = super::SQUARES;
        for square in map {
            let str = square.to_string();
            let sq = Square::get_from_string(&str);
            match sq {
                Some(_) => assert_eq!(*square, sq.unwrap()),
                None => assert!(false),
            }
        }
    }

    #[test]
    pub fn square_from_rank_file() {
        assert!(Square::get_square(Rank::Rank1, File::FileA) == Square::a1);
        assert!(Square::get_square(Rank::Rank8, File::FileA) == Square::a8);

        assert!(Square::get_square(Rank::Rank1, File::FileH) == Square::h1);
        assert!(Square::get_square(Rank::Rank8, File::FileH) == Square::h8);
    }
}
