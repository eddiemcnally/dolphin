use crate::board::bitboard::Bitboard;
use crate::board::file::*;
use crate::board::rank::*;
use std::fmt;
use std::slice::Iter;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Square(u8);

#[rustfmt::skip]
const SQUARES: [Square; Square::NUM_SQUARES] = [
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
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);

    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);

    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);

    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);

    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    pub const NUM_SQUARES: usize = 64;

    #[inline(always)]
    pub const fn new(num: u8) -> Option<Square> {
        match num {
            0..=63 => Some(Square(num)),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn as_index(self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub fn north(self) -> Option<Square> {
        let rank_plus_1 = self.rank().add_one();
        let file = self.file();

        match rank_plus_1 {
            Some(rank_plus_1) => Square::from_rank_file(rank_plus_1, file),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn south(self) -> Option<Square> {
        let rank_minus_1 = self.rank().subtract_one();
        let file = self.file();

        match rank_minus_1 {
            Some(rank_minus_1) => Square::from_rank_file(rank_minus_1, file),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn north_east(self) -> Option<Square> {
        let rank_plus_1 = self.rank().add_one();
        let file_plus_1 = self.file().add_one();

        match (rank_plus_1, file_plus_1) {
            (Some(rank_plus_1), Some(file_plus_1)) => {
                Square::from_rank_file(rank_plus_1, file_plus_1)
            }
            (_, _) => None,
        }
    }

    #[inline(always)]
    pub fn south_east(self) -> Option<Square> {
        let rank_minus_1 = self.rank().subtract_one();
        let file_plus_1 = self.file().add_one();

        match (rank_minus_1, file_plus_1) {
            (Some(rank_minus_1), Some(file_plus_1)) => {
                Square::from_rank_file(rank_minus_1, file_plus_1)
            }
            (_, _) => None,
        }
    }

    #[inline(always)]
    pub fn south_west(self) -> Option<Square> {
        let rank_minus_1 = self.rank().subtract_one();
        let file_minus_1 = self.file().subtract_one();

        match (rank_minus_1, file_minus_1) {
            (Some(rank_minus_1), Some(file_minus_1)) => {
                Square::from_rank_file(rank_minus_1, file_minus_1)
            }
            (_, _) => None,
        }
    }

    #[inline(always)]
    pub fn north_west(self) -> Option<Square> {
        let rank_plus_1 = self.rank().add_one();
        let file_minus_1 = self.file().subtract_one();

        match (rank_plus_1, file_minus_1) {
            (Some(rank_plus_1), Some(file_minus_1)) => {
                Square::from_rank_file(rank_plus_1, file_minus_1)
            }
            (_, _) => None,
        }
    }

    #[inline(always)]
    pub fn rank(self) -> Rank {
        Rank::new(self.rank_as_u8()).unwrap()
    }

    #[inline(always)]
    pub fn file(self) -> File {
        File::new(self.file_as_u8()).unwrap()
    }

    #[inline(always)]
    pub fn from_rank_file(rank: Rank, file: File) -> Option<Square> {
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
                return Square::from_rank_file(rank, file);
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
    pub fn north() {
        assert!(Square::A1.north() == Some(Square::A2));
        assert!(Square::B1.north() == Some(Square::B2));
        assert!(Square::C1.north() == Some(Square::C2));
        assert!(Square::D1.north() == Some(Square::D2));
        assert!(Square::E1.north() == Some(Square::E2));
        assert!(Square::F1.north() == Some(Square::F2));
        assert!(Square::G1.north() == Some(Square::G2));
        assert!(Square::H1.north() == Some(Square::H2));

        assert!(Square::A7.north() == Some(Square::A8));
        assert!(Square::B7.north() == Some(Square::B8));
        assert!(Square::C7.north() == Some(Square::C8));
        assert!(Square::D7.north() == Some(Square::D8));
        assert!(Square::E7.north() == Some(Square::E8));
        assert!(Square::F7.north() == Some(Square::F8));
        assert!(Square::G7.north() == Some(Square::G8));
        assert!(Square::H7.north() == Some(Square::H8));

        assert!(Square::A8.north() == None);
        assert!(Square::B8.north() == None);
        assert!(Square::C8.north() == None);
        assert!(Square::D8.north() == None);
        assert!(Square::E8.north() == None);
        assert!(Square::F8.north() == None);
        assert!(Square::G8.north() == None);
        assert!(Square::H8.north() == None);
    }

    #[test]
    pub fn south() {
        assert!(Square::A1.south() == None);
        assert!(Square::B1.south() == None);
        assert!(Square::C1.south() == None);
        assert!(Square::D1.south() == None);
        assert!(Square::E1.south() == None);
        assert!(Square::F1.south() == None);
        assert!(Square::G1.south() == None);
        assert!(Square::H1.south() == None);

        assert!(Square::A8.south() == Some(Square::A7));
        assert!(Square::B8.south() == Some(Square::B7));
        assert!(Square::C8.south() == Some(Square::C7));
        assert!(Square::D8.south() == Some(Square::D7));
        assert!(Square::E8.south() == Some(Square::E7));
        assert!(Square::F8.south() == Some(Square::F7));
        assert!(Square::G8.south() == Some(Square::G7));
        assert!(Square::H8.south() == Some(Square::H7));

        assert!(Square::A2.south() == Some(Square::A1));
        assert!(Square::B2.south() == Some(Square::B1));
        assert!(Square::C2.south() == Some(Square::C1));
        assert!(Square::D2.south() == Some(Square::D1));
        assert!(Square::E2.south() == Some(Square::E1));
        assert!(Square::F2.south() == Some(Square::F1));
        assert!(Square::G2.south() == Some(Square::G1));
        assert!(Square::H2.south() == Some(Square::H1));
    }

    #[test]
    pub fn north_east() {
        assert!(Square::A1.north_east() == Some(Square::B2));
        assert!(Square::B1.north_east() == Some(Square::C2));
        assert!(Square::C1.north_east() == Some(Square::D2));
        assert!(Square::D1.north_east() == Some(Square::E2));
        assert!(Square::E1.north_east() == Some(Square::F2));
        assert!(Square::F1.north_east() == Some(Square::G2));
        assert!(Square::G1.north_east() == Some(Square::H2));
        assert!(Square::H1.north_east() == None);

        assert!(Square::A8.north_east() == None);
        assert!(Square::B8.north_east() == None);
        assert!(Square::C8.north_east() == None);
        assert!(Square::D8.north_east() == None);
        assert!(Square::E8.north_east() == None);
        assert!(Square::F8.north_east() == None);
        assert!(Square::G8.north_east() == None);
        assert!(Square::H8.north_east() == None);
    }

    #[test]
    pub fn north_west() {
        assert!(Square::A1.north_west() == None);
        assert!(Square::B1.north_west() == Some(Square::A2));
        assert!(Square::C1.north_west() == Some(Square::B2));
        assert!(Square::D1.north_west() == Some(Square::C2));
        assert!(Square::E1.north_west() == Some(Square::D2));
        assert!(Square::F1.north_west() == Some(Square::E2));
        assert!(Square::G1.north_west() == Some(Square::F2));
        assert!(Square::H1.north_west() == Some(Square::G2));

        assert!(Square::A8.north_west() == None);
        assert!(Square::B8.north_west() == None);
        assert!(Square::C8.north_west() == None);
        assert!(Square::D8.north_west() == None);
        assert!(Square::E8.north_west() == None);
        assert!(Square::F8.north_west() == None);
        assert!(Square::G8.north_west() == None);
        assert!(Square::H8.north_west() == None);
    }

    #[test]
    pub fn south_east() {
        assert!(Square::A1.south_east() == None);
        assert!(Square::B1.south_east() == None);
        assert!(Square::C1.south_east() == None);
        assert!(Square::D1.south_east() == None);
        assert!(Square::E1.south_east() == None);
        assert!(Square::F1.south_east() == None);
        assert!(Square::G1.south_east() == None);
        assert!(Square::H1.south_east() == None);

        assert!(Square::A8.south_east() == Some(Square::B7));
        assert!(Square::B8.south_east() == Some(Square::C7));
        assert!(Square::C8.south_east() == Some(Square::D7));
        assert!(Square::D8.south_east() == Some(Square::E7));
        assert!(Square::E8.south_east() == Some(Square::F7));
        assert!(Square::F8.south_east() == Some(Square::G7));
        assert!(Square::G8.south_east() == Some(Square::H7));
        assert!(Square::H8.south_east() == None);
    }

    #[test]
    pub fn south_west() {
        assert!(Square::A8.south_west() == None);
        assert!(Square::B8.south_west() == Some(Square::A7));
        assert!(Square::C8.south_west() == Some(Square::B7));
        assert!(Square::D8.south_west() == Some(Square::C7));
        assert!(Square::E8.south_west() == Some(Square::D7));
        assert!(Square::F8.south_west() == Some(Square::E7));
        assert!(Square::G8.south_west() == Some(Square::F7));
        assert!(Square::H8.south_west() == Some(Square::G7));

        assert!(Square::A1.south_west() == None);
        assert!(Square::B1.south_west() == None);
        assert!(Square::C1.south_west() == None);
        assert!(Square::D1.south_west() == None);
        assert!(Square::E1.south_west() == None);
        assert!(Square::F1.south_west() == None);
        assert!(Square::G1.south_west() == None);
        assert!(Square::H1.south_west() == None);
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
            assert_eq!(*square, sq.expect("Invalid square"));
        }
        assert!(Square::from_rank_file(Rank::R3, File::G) == Some(Square::G3));
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
        assert!(Square::from_rank_file(Rank::R1, File::A) == Some(Square::A1));
        assert!(Square::from_rank_file(Rank::R8, File::A) == Some(Square::A8));

        assert!(Square::from_rank_file(Rank::R1, File::H) == Some(Square::H1));
        assert!(Square::from_rank_file(Rank::R8, File::H) == Some(Square::H8));
    }

    #[test]
    pub fn values() {
        for (i, square) in Square::iterator().enumerate() {
            assert_eq!(square.as_index(), i);
        }
    }
}
