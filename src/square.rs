use std::mem::transmute;
use std::fmt;
use square::rank::Rank;
use square::file::File;


#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
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

impl Square {
    pub fn rank(self) -> Rank {
        let rank_num = self as u8 >> 3;

        match rank_num {
            0 => Rank::Rank1,
            1 => Rank::Rank2,
            2 => Rank::Rank3,
            4 => Rank::Rank5,
            3 => Rank::Rank4,
            5 => Rank::Rank6,
            6 => Rank::Rank7,
            7 => Rank::Rank8,
            _ => panic!("invalid rank number"),
        }
    }

    pub fn file(self) -> File {
        let file_num = (self as u8 % 8) as u8;

        match file_num {
            0 => File::FileA,
            1 => File::FileB,
            2 => File::FileC,
            3 => File::FileD,
            4 => File::FileE,
            5 => File::FileF,
            6 => File::FileG,
            7 => File::FileH,
            _ => panic!("invalid file number"),
        }
    }

    pub fn get_square(rank: Rank, file: File) -> Square {
        let sq = rank as u8 * 8 + file as u8;

        // todo: find a way of removing the "unsafe" code
        let retval: Square = unsafe { transmute(sq as u8) };
        return retval;
    }

    pub fn get_from_string(square_str: &str) -> Square {
        let f = square_str.chars().nth(0).unwrap();
        let r = square_str.chars().nth(1).unwrap();

        let file = File::from_char(f);
        let rank = Rank::from_char(r);

        Square::get_square(rank, file)
    }
}



pub mod rank {
    #[derive(Debug)]
    #[derive(Eq, PartialEq, Hash)]
    pub enum Rank {
        Rank1 = 0,
        Rank2,
        Rank3,
        Rank4,
        Rank5,
        Rank6,
        Rank7,
        Rank8,
    }

    impl Rank {
        pub fn from_char(rank: char) -> Rank {
            match rank {
                '1' => Rank::Rank1,
                '2' => Rank::Rank2,
                '3' => Rank::Rank3,
                '4' => Rank::Rank4,
                '5' => Rank::Rank5,
                '6' => Rank::Rank6,
                '7' => Rank::Rank7,
                '8' => Rank::Rank8,
                _ => panic!("Invalid rank character {}", rank),
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
    }
}

pub mod file {
    #[derive(Debug)]
    #[derive(Eq, PartialEq, Hash)]
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

    impl File {
        pub fn from_char(file: char) -> File {
            match file {
                'a' => File::FileA,
                'b' => File::FileB,
                'c' => File::FileC,
                'd' => File::FileD,
                'e' => File::FileE,
                'f' => File::FileF,
                'g' => File::FileG,
                'h' => File::FileH,
                _ => panic!("Invalid file character {}", file),
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
    }

}

#[cfg(test)]
pub mod tests {
    use super::Square;
    use super::Rank;
    use super::File;
    use utils;
    use std::collections::HashMap;



    #[test]
    pub fn test_rank_from_square() {
        let map = utils::get_square_rank_file_map();
        for (square, (rank, _)) in map {
            assert_eq!(square.rank(), rank);
        }
    }


    #[test]
    pub fn test_file_from_square() {
        let map = utils::get_square_rank_file_map();
        for (square, (_, file)) in map {
            assert_eq!(square.file(), file);
        }
    }

    #[test]
    pub fn test_file_from_char() {
        let map = get_file_map();
        for (file, ch) in map {
            let f = File::from_char(ch);
            assert_eq!(f, file);
        }
    }

    #[test]
    pub fn test_file_to_char() {
        let map = get_file_map();
        for (file, ch) in map {
            let cc = File::to_char(file);
            assert_eq!(cc, ch);
        }
    }


    #[test]
    pub fn test_rank_from_char() {
        let map = get_rank_map();
        for (rank, ch) in map {
            let r = Rank::from_char(ch);
            assert_eq!(r, rank);
        }
    }

    #[test]
    pub fn test_rank_to_char() {
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
    pub fn test_square_from_rank_and_file() {
        let map = utils::get_square_rank_file_map();
        for (square, (rank, file)) in map {
            let sq = Square::get_square(rank, file);
            assert_eq!(square, sq);
        }
    }


    #[test]
    pub fn test_square_from_string() {
        let map = utils::get_square_rank_file_map();
        for (square, _) in map {
            let str = square.to_string();
            let sq = Square::get_from_string(&str);
            assert_eq!(square, sq);
        }
    }


}
