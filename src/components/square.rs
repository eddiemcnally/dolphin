use core::core_traits::ArrayAccessor;
use num::FromPrimitive;
use std::fmt;
use std::ops::Shl;
use std::slice::Iter;

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Square {
    a1 = 0,
    b1,
    c1,
    d1,
    e1,
    f1,
    g1,
    h1,
    a2,
    b2,
    c2,
    d2,
    e2,
    f2,
    g2,
    h2,
    a3,
    b3,
    c3,
    d3,
    e3,
    f3,
    g3,
    h3,
    a4,
    b4,
    c4,
    d4,
    e4,
    f4,
    g4,
    h4,
    a5,
    b5,
    c5,
    d5,
    e5,
    f5,
    g5,
    h5,
    a6,
    b6,
    c6,
    d6,
    e6,
    f6,
    g6,
    h6,
    a7,
    b7,
    c7,
    d7,
    e7,
    f7,
    g7,
    h7,
    a8,
    b8,
    c8,
    d8,
    e8,
    f8,
    g8,
    h8,
}
}
impl Default for Square {
    fn default() -> Square {
        Square::a1
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_square_array() -> &'static [Square] {
    SQUARES
}

pub static SQUARES: &[Square] = &[
    Square::a1,
    Square::b1,
    Square::c1,
    Square::d1,
    Square::e1,
    Square::f1,
    Square::g1,
    Square::h1,
    Square::a2,
    Square::b2,
    Square::c2,
    Square::d2,
    Square::e2,
    Square::f2,
    Square::g2,
    Square::h2,
    Square::a3,
    Square::b3,
    Square::c3,
    Square::d3,
    Square::e3,
    Square::f3,
    Square::g3,
    Square::h3,
    Square::a4,
    Square::b4,
    Square::c4,
    Square::d4,
    Square::e4,
    Square::f4,
    Square::g4,
    Square::h4,
    Square::a5,
    Square::b5,
    Square::c5,
    Square::d5,
    Square::e5,
    Square::f5,
    Square::g5,
    Square::h5,
    Square::a6,
    Square::b6,
    Square::c6,
    Square::d6,
    Square::e6,
    Square::f6,
    Square::g6,
    Square::h6,
    Square::a7,
    Square::b7,
    Square::c7,
    Square::d7,
    Square::e7,
    Square::f7,
    Square::g7,
    Square::h7,
    Square::a8,
    Square::b8,
    Square::c8,
    Square::d8,
    Square::e8,
    Square::f8,
    Square::g8,
    Square::h8,
];

impl Square {
    pub fn derive_relative_square(sq: Square, rank_offset: i8, file_offset: i8) -> Option<Square> {
        let target_rank = sq.rank_as_u8() as i8 + rank_offset;
        let target_file = sq.file_as_u8() as i8 + file_offset;

        let rank = Rank::from_num(target_rank);
        if rank.is_none() {
            return None;
        }

        let file = File::from_num(target_file);
        if file.is_none() {
            return None;
        }

        Some(Square::get_square(rank.unwrap(), file.unwrap()))
    }

    pub fn square_plus_1_rank(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank8 => None,
            _ => {
                let s = self as u8 + 8;
                Square::from_num(s)
            }
        }
    }

    pub fn square_minus_1_rank(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank1 => None,
            _ => {
                let s = self as u8 - 8;
                Square::from_num(s)
            }
        }
    }

    pub fn square_plus_2_ranks(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank7 | Rank::Rank8 => None,
            _ => {
                let s = self as u8 + 16;
                Square::from_num(s)
            }
        }
    }

    pub fn square_minus_2_ranks(self) -> Option<Square> {
        match self.rank() {
            Rank::Rank1 | Rank::Rank2 => None,
            _ => {
                let s = self as u8 - 16;
                Square::from_num(s)
            }
        }
    }

    pub fn rank(self) -> Rank {
        let rank_num = self.rank_as_u8();
        Rank::from_num(rank_num as i8).unwrap()
    }

    pub fn file(self) -> File {
        let file_num = self.file_as_u8();
        File::from_num(file_num as i8).unwrap()
    }

    pub fn get_square(rank: Rank, file: File) -> Square {
        let sq = rank as u8 * 8 + file as u8;
        Square::from_num(sq).unwrap()
    }

    pub fn get_square_as_bb(self) -> u64 {
        let bit: u64 = 1;
        bit.shl(self.to_offset())
    }

    pub fn get_from_string(square_str: &str) -> Option<Square> {
        let f = square_str.chars().nth(0).unwrap();
        let r = square_str.chars().nth(1).unwrap();

        let file = File::from_char(f);
        let rank = Rank::from_char(r);

        if file.is_some() && rank.is_some() {
            return Some(Square::get_square(rank.unwrap(), file.unwrap()));
        }
        None
    }

    pub fn from_num(num: u8) -> Option<Square> {
        Square::from_u8(num)
    }

    pub const fn same_rank(self, other: Square) -> bool {
        let this_rank = self.rank_as_u8();
        let other_rank = other.rank_as_u8();
        this_rank == other_rank
    }

    pub const fn same_file(self, other: Square) -> bool {
        let this_file = self.file_as_u8();
        let other_file = other.file_as_u8();
        this_file == other_file
    }

    const fn rank_as_u8(self) -> u8 {
        self as u8 >> 3
    }
    const fn file_as_u8(self) -> u8 {
        (self as u8 % 8) as u8
    }
}

impl ArrayAccessor for Square {
    fn to_offset(self) -> usize {
        self as usize
    }
}

enum_from_primitive! {
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Rank {
    Rank1 = 0,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
}}

impl Rank {
    pub const fn from_num(num: i8) -> Option<Rank> {
        match num {
            0 => Some(Rank::Rank1),
            1 => Some(Rank::Rank2),
            2 => Some(Rank::Rank3),
            3 => Some(Rank::Rank4),
            4 => Some(Rank::Rank5),
            5 => Some(Rank::Rank6),
            6 => Some(Rank::Rank7),
            7 => Some(Rank::Rank8),
            _ => None,
        }
        //Rank::from_i8(num)
    }

    pub fn from_char(rank: char) -> Option<Rank> {
        match rank {
            '1' => Some(Rank::Rank1),
            '2' => Some(Rank::Rank2),
            '3' => Some(Rank::Rank3),
            '4' => Some(Rank::Rank4),
            '5' => Some(Rank::Rank5),
            '6' => Some(Rank::Rank6),
            '7' => Some(Rank::Rank7),
            '8' => Some(Rank::Rank8),
            _ => None,
        }
    }
    pub fn to_char(rank: Rank) -> char {
        match rank {
            Rank::Rank1 => '1',
            Rank::Rank2 => '2',
            Rank::Rank3 => '3',
            Rank::Rank4 => '4',
            Rank::Rank5 => '5',
            Rank::Rank6 => '6',
            Rank::Rank7 => '7',
            Rank::Rank8 => '8',
        }
    }

    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            Rank::Rank1,
            Rank::Rank2,
            Rank::Rank3,
            Rank::Rank4,
            Rank::Rank5,
            Rank::Rank6,
            Rank::Rank7,
            Rank::Rank8,
        ];
        RANKS.iter()
    }

    pub fn reverse_iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            Rank::Rank8,
            Rank::Rank7,
            Rank::Rank6,
            Rank::Rank5,
            Rank::Rank4,
            Rank::Rank3,
            Rank::Rank2,
            Rank::Rank1,
        ];
        RANKS.iter()
    }
}

enum_from_primitive! {
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
    pub enum File {
        FileA = 0,
        FileB,
        FileC,
        FileD,
        FileE,
        FileF,
        FileG,
        FileH,
    }
}

impl File {
    pub fn from_num(num: i8) -> Option<File> {
        File::from_i8(num)
    }

    pub fn from_char(file: char) -> Option<File> {
        match file {
            'a' => Some(File::FileA),
            'b' => Some(File::FileB),
            'c' => Some(File::FileC),
            'd' => Some(File::FileD),
            'e' => Some(File::FileE),
            'f' => Some(File::FileF),
            'g' => Some(File::FileG),
            'h' => Some(File::FileH),
            _ => None,
        }
    }
    pub fn to_char(file: File) -> char {
        match file {
            File::FileA => 'a',
            File::FileB => 'b',
            File::FileC => 'c',
            File::FileD => 'd',
            File::FileE => 'e',
            File::FileF => 'f',
            File::FileG => 'g',
            File::FileH => 'h',
        }
    }
    pub fn iterator() -> Iter<'static, File> {
        static FILES: [File; 8] = [
            File::FileA,
            File::FileB,
            File::FileC,
            File::FileD,
            File::FileE,
            File::FileF,
            File::FileG,
            File::FileH,
        ];
        FILES.iter()
    }
    pub fn reverse_iterator() -> Iter<'static, File> {
        static FILES: [File; 8] = [
            File::FileH,
            File::FileG,
            File::FileF,
            File::FileE,
            File::FileD,
            File::FileC,
            File::FileB,
            File::FileA,
        ];
        FILES.iter()
    }
}

#[cfg(test)]
pub mod tests {
    use super::File;
    use super::Rank;
    use super::Square;
    use std::collections::HashMap;
    use utils;

    #[test]
    pub fn derive_relative_square() {
        assert!(Square::derive_relative_square(Square::a1, -1, 0).is_some() == false);
        assert!(Square::derive_relative_square(Square::a1, 0, -1).is_some() == false);
        assert!(Square::derive_relative_square(Square::a8, 1, 0).is_some() == false);
        assert!(Square::derive_relative_square(Square::a8, 0, -1).is_some() == false);
        assert!(Square::derive_relative_square(Square::h1, -1, 0).is_some() == false);
        assert!(Square::derive_relative_square(Square::h1, 0, 1).is_some() == false);
        assert!(Square::derive_relative_square(Square::h8, 1, 0).is_some() == false);
        assert!(Square::derive_relative_square(Square::h8, 0, 1).is_some() == false);

        assert!(Square::derive_relative_square(Square::a1, 1, 0).is_some() == true);
        assert!(Square::derive_relative_square(Square::a1, 0, 1).is_some() == true);
        assert!(Square::derive_relative_square(Square::a8, -1, 0).is_some() == true);
        assert!(Square::derive_relative_square(Square::a8, 0, 1).is_some() == true);
        assert!(Square::derive_relative_square(Square::h1, 1, 0).is_some() == true);
        assert!(Square::derive_relative_square(Square::h1, 0, -1).is_some() == true);
        assert!(Square::derive_relative_square(Square::h8, -1, 0).is_some() == true);
        assert!(Square::derive_relative_square(Square::h8, 0, -1).is_some() == true);
    }

    #[test]
    pub fn rank_from_square() {
        let map = utils::get_square_rank_file_map();
        for (square, (rank, _)) in map {
            assert_eq!(square.rank(), rank);
        }
    }

    #[test]
    pub fn file_from_square() {
        let map = utils::get_square_rank_file_map();
        for (square, (_, file)) in map {
            assert_eq!(square.file(), file);
        }
    }

    #[test]
    pub fn file_from_char() {
        let map = get_file_map();
        for (file, ch) in map {
            let f = File::from_char(ch);
            assert_eq!(f.unwrap(), file);
        }
    }

    #[test]
    pub fn file_to_char() {
        let map = get_file_map();
        for (file, ch) in map {
            let cc = File::to_char(file);
            assert_eq!(cc, ch);
        }
    }

    #[test]
    pub fn rank_from_char() {
        let map = get_rank_map();
        for (rank, ch) in map {
            let r = Rank::from_char(ch);
            assert_eq!(r.unwrap(), rank);
        }
    }

    #[test]
    pub fn rank_to_char() {
        let map = get_rank_map();
        for (rank, ch) in map {
            let cc = Rank::to_char(rank);
            assert_eq!(cc, ch);
        }
    }

    fn get_rank_map() -> HashMap<Rank, char> {
        let mut map: HashMap<Rank, char> = HashMap::new();
        map.insert(Rank::Rank1, '1');
        map.insert(Rank::Rank2, '2');
        map.insert(Rank::Rank3, '3');
        map.insert(Rank::Rank4, '4');
        map.insert(Rank::Rank5, '5');
        map.insert(Rank::Rank6, '6');
        map.insert(Rank::Rank7, '7');
        map.insert(Rank::Rank8, '8');
        return map;
    }

    fn get_file_map() -> HashMap<File, char> {
        let mut map: HashMap<File, char> = HashMap::new();
        map.insert(File::FileA, 'a');
        map.insert(File::FileB, 'b');
        map.insert(File::FileC, 'c');
        map.insert(File::FileD, 'd');
        map.insert(File::FileE, 'e');
        map.insert(File::FileF, 'f');
        map.insert(File::FileG, 'g');
        map.insert(File::FileH, 'h');
        return map;
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
        let map = utils::get_square_rank_file_map();
        for (square, (rank, file)) in map {
            let sq = Square::get_square(rank, file);
            assert_eq!(square, sq);
        }
    }

    #[test]
    pub fn square_from_string() {
        let map = utils::get_square_rank_file_map();
        for (square, _) in map {
            let str = square.to_string();
            let sq = Square::get_from_string(&str);
            match sq {
                Some(_) => assert_eq!(square, sq.unwrap()),
                None => assert!(false),
            }
        }
    }
}
