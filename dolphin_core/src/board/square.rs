use crate::board::bitboard::Bitboard;
use crate::board::file::*;
use crate::board::rank::*;
use num_enum::TryFromPrimitive;
use std::fmt;
use std::slice::Iter;

#[rustfmt::skip]
#[allow(non_camel_case_types)]
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

impl Square {
    pub const NUM_SQUARES: usize = 64;

    pub const fn new(num: u8) -> Square {
        Self::get_sq(num)
    }
    pub const fn as_index(&self) -> usize {
        *self as usize
    }
    pub fn plus_1_rank(self) -> Square {
        Square::new(self.as_index() as u8 + 8)
    }

    const fn get_sq(num: u8) -> Square {
        // a match is more performant than looking up the array or using any existing
        // crates for converting ints to enums :-)
        match num {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,

            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,

            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,

            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,

            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,

            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,

            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,

            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,

            _ => panic!("Invalid square"),
        }
    }

    pub fn minus_1_rank(self) -> Square {
        Square::new(self.as_index() as u8 - 8)
    }

    pub fn plus_2_ranks(self) -> Square {
        Square::new(self.as_index() as u8 + 16)
    }

    pub fn minus_2_ranks(self) -> Square {
        Square::new(self.as_index() as u8 - 16)
    }

    pub fn rank(self) -> Rank {
        Rank::new(self.rank_as_u8())
    }

    pub fn file(self) -> File {
        File::new(self.file_as_u8())
    }

    pub fn from_rank_file(rank: Rank, file: File) -> Square {
        let sq = (rank.as_index() << 3) + file.as_index();
        Square::new(sq as u8)
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

    const fn rank_as_u8(self) -> u8 {
        self.as_index() as u8 >> 3
    }
    const fn file_as_u8(self) -> u8 {
        self.as_index() as u8 & 0x07
    }

    pub fn iterator() -> Iter<'static, Square> {
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
