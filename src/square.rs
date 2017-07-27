use std::mem::transmute;

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

// inline uint8_t get_rank(enum square sq){
// 	return (uint8_t)(sq >> 3);
// }
//
// inline uint8_t get_file(enum square sq){
// 	return (uint8_t)(sq % 8);
// }

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
}





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
    pub fn incr(self) -> Rank {
        match self {
            Rank::Rank1 => Rank::Rank2,
            Rank::Rank2 => Rank::Rank3,
            Rank::Rank3 => Rank::Rank4,
            Rank::Rank4 => Rank::Rank5,
            Rank::Rank5 => Rank::Rank6,
            Rank::Rank6 => Rank::Rank7,
            Rank::Rank7 => Rank::Rank8,
            _ => panic!(),
        }
    }
    pub fn decr(self) -> Rank {
        match self {
            Rank::Rank2 => Rank::Rank1,
            Rank::Rank3 => Rank::Rank2,
            Rank::Rank4 => Rank::Rank3,
            Rank::Rank5 => Rank::Rank4,
            Rank::Rank6 => Rank::Rank5,
            Rank::Rank7 => Rank::Rank6,
            Rank::Rank8 => Rank::Rank7,
            _ => panic!(),
        }
    }
}

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
    pub fn incr(self) -> File {
        match self {
            File::FileA => File::FileB,
            File::FileB => File::FileC,
            File::FileC => File::FileD,
            File::FileD => File::FileE,
            File::FileE => File::FileF,
            File::FileF => File::FileG,
            File::FileG => File::FileH,
            _ => panic!(),
        }
    }
    pub fn decr(self) -> File {
        match self {
            File::FileB => File::FileA,
            File::FileC => File::FileB,
            File::FileD => File::FileC,
            File::FileE => File::FileD,
            File::FileF => File::FileE,
            File::FileG => File::FileF,
            File::FileH => File::FileG,
            _ => panic!(),
        }
    }
}


#[cfg(test)]
mod tests {
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



}
