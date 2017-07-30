use std::mem::transmute;
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
    #[derive(Eq, PartialEq)]
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
    #[derive(Eq, PartialEq)]
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

    #[test]
    pub fn test_square_rank() {
        let mut sq = Square::b4;
        assert_eq!(sq.rank(), Rank::Rank4);

        sq = Square::a1;
        assert_eq!(sq.rank(), Rank::Rank1);

        sq = Square::h8;
        assert_eq!(sq.rank(), Rank::Rank8);
    }


    #[test]
    pub fn test_square_file() {
        let mut sq = Square::b4;
        assert_eq!(sq.file(), File::FileB);

        sq = Square::a1;
        assert_eq!(sq.file(), File::FileA);

        sq = Square::h8;
        assert_eq!(sq.file(), File::FileH);
    }

    #[test]
    pub fn test_file_from_char() {
        let mut f = File::from_char('a');
        assert_eq!(f, File::FileA);
        f = File::from_char('a');
        assert_eq!(f, File::FileA);
        f = File::from_char('b');
        assert_eq!(f, File::FileB);
        f = File::from_char('c');
        assert_eq!(f, File::FileC);
        f = File::from_char('d');
        assert_eq!(f, File::FileD);
        f = File::from_char('e');
        assert_eq!(f, File::FileE);
        f = File::from_char('f');
        assert_eq!(f, File::FileF);
        f = File::from_char('g');
        assert_eq!(f, File::FileG);
        f = File::from_char('h');
        assert_eq!(f, File::FileH);
    }
    pub fn test_file_to_char() {
        let mut f = File::to_char(File::FileA);
        assert_eq!(f, 'a');
        f = File::to_char(File::FileB);
        assert_eq!(f, 'b');
        f = File::to_char(File::FileC);
        assert_eq!(f, 'c');
        f = File::to_char(File::FileD);
        assert_eq!(f, 'd');
        f = File::to_char(File::FileE);
        assert_eq!(f, 'e');
        f = File::to_char(File::FileF);
        assert_eq!(f, 'f');
        f = File::to_char(File::FileG);
        assert_eq!(f, 'g');
        f = File::to_char(File::FileH);
        assert_eq!(f, 'h');
    }



}
