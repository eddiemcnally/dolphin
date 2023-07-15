use crate::board::bitboard::Bitboard;
use crate::board::file::*;
use crate::board::rank::*;
use crate::core::array_offset::EnumAsOffset;
use num_enum::TryFromPrimitive;
use std::fmt;
use std::slice::Iter;

#[rustfmt::skip]
#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Square{
    #[default]
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8       
}

impl EnumAsOffset for Square {
    fn as_index(&self) -> usize {
        *self as usize
    }
}

impl Square {
    pub const NUM_SQUARES: usize = 64;

    pub fn new(num: u8) -> Option<Square> {
        match Square::try_from(num) {
            Ok(square) => Some(square),
            Err(_) => None,
        }
    }

    pub fn plus_1_rank(self) -> Option<Square> {
        Square::new(self.as_index() as u8 + 8)
    }

    pub fn minus_1_rank(self) -> Option<Square> {
        Square::new(self.as_index() as u8 - 8)
    }

    pub fn plus_2_ranks(self) -> Option<Square> {
        Square::new(self.as_index() as u8 + 16)
    }

    pub fn minus_2_ranks(self) -> Option<Square> {
        Square::new(self.as_index() as u8 - 16)
    }

    pub fn rank(self) -> Rank {
        Rank::new(self.rank_as_u8()).unwrap()
    }

    pub fn file(self) -> File {
        File::new(self.file_as_u8()).unwrap()
    }

    pub fn from_rank_file(rank: Rank, file: File) -> Square {
        let sq = (rank.as_index() << 3) + file.as_index();
        Square::new(sq as u8).unwrap()
    }

    pub fn get_square_as_bb(self) -> Bitboard {
        Bitboard::new(0x01u64 << (self.as_index()))
    }

    pub fn get_from_string(str: &str) -> Option<Square> {
        let f = str.chars().next().unwrap();
        let r = str.chars().nth(1).unwrap();

        if let Some(file) = File::from_char(f) {
            if let Some(rank) = Rank::from_char(r) {
                return Some(Square::from_rank_file(rank, file));
            }
        }
        None
    }

    pub fn same_rank(self, other: Square) -> bool {
        self.rank_as_u8() == other.rank_as_u8()
    }

    pub fn same_file(self, other: Square) -> bool {
        self.file_as_u8() == other.file_as_u8()
    }

    fn rank_as_u8(self) -> u8 {
        self.as_index() as u8 >> 3
    }
    fn file_as_u8(self) -> u8 {
        self.as_index() as u8 & 0x07
    }

    pub fn iterator() -> Iter<'static, Square> {
        #[rustfmt::skip]
        static SQUARES: [Square; Square::NUM_SQUARES] = [
            Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
            Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
            Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
            Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
            Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
            Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
            Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
            Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
        ];
        SQUARES.iter()
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

#[cfg(test)]
pub mod tests {
    use super::Square;
    use crate::board::file::File;
    use crate::board::rank::Rank;
    use crate::core::array_offset::EnumAsOffset;

    #[test]
    pub fn rank_from_square() {
        assert!(Square::A1.rank() == Rank::R1);
        assert!(Square::B2.rank() == Rank::R2);
        assert!(Square::H3.rank() == Rank::R3);
        assert!(Square::G4.rank() == Rank::R4);
        assert!(Square::A5.rank() == Rank::R5);
        assert!(Square::C6.rank() == Rank::R6);
        assert!(Square::D7.rank() == Rank::R7);
        assert!(Square::F8.rank() == Rank::R8);
    }

    #[test]
    pub fn file_from_square() {
        assert!(Square::A1.file() == File::A);
        assert!(Square::E5.file() == File::E);
        assert!(Square::D4.file() == File::D);
    }

    #[test]
    pub fn convert_square_to_uint() {
        let sq: Square = Square::B1;
        let num = sq.as_index();

        assert_eq!(num, 1);

        let sq1: Square = Square::D7;
        let num1 = sq1.as_index();

        assert_eq!(num1, 51);
    }

    #[test]
    pub fn from_rank_and_file() {
        let map = Square::iterator();
        for square in map {
            let rank = square.rank();
            let file = square.file();
            let sq = Square::from_rank_file(rank, file);
            assert_eq!(*square, sq);
        }
        assert!(Square::from_rank_file(Rank::R3, File::G) == Square::G3);
    }

    #[test]
    pub fn from_string() {
        let map = Square::iterator();
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
    pub fn from_rank_file() {
        assert!(Square::from_rank_file(Rank::R1, File::A) == Square::A1);
        assert!(Square::from_rank_file(Rank::R8, File::A) == Square::A8);

        assert!(Square::from_rank_file(Rank::R1, File::H) == Square::H1);
        assert!(Square::from_rank_file(Rank::R8, File::H) == Square::H8);
    }

    #[test]
    pub fn values() {
        for (i, square) in Square::iterator().enumerate() {
            assert_eq!(square.as_index(), i);
        }
    }
}
