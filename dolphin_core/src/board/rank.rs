use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u64)]
pub enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
}

impl Rank {
    pub fn from_num(num: u64) -> Option<Rank> {
        let rank = Rank::try_from(num);
        match rank {
            Ok(rank) => Some(rank),
            _ => None,
        }
    }

    pub fn add_one(&self) -> Option<Rank> {
        let new_rank = *self as u64 + 1;
        Rank::from_num(new_rank)
    }

    pub fn add_two(&self) -> Option<Rank> {
        let new_rank = *self as u64 + 2;
        Rank::from_num(new_rank)
    }

    pub fn subtract_one(&self) -> Option<Rank> {
        match self {
            Rank::Rank1 => None,
            _ => {
                let new_rank = *self as u64 - 1;
                Rank::from_num(new_rank)
            }
        }
    }

    pub fn subtract_two(&self) -> Option<Rank> {
        match self {
            Rank::Rank1 | Rank::Rank2 => None,
            _ => {
                let new_rank = *self as u64 - 2;
                Rank::from_num(new_rank)
            }
        }
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
    pub fn to_char(&self) -> char {
        match self {
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("{}", self.to_char()));

        write!(f, "{}", debug_str)
    }
}

#[cfg(test)]
pub mod tests {
    use super::Rank;
    use std::collections::HashMap;

    #[test]
    pub fn rank_from_u8() {
        assert!(Rank::from_num(0) == Some(Rank::Rank1));
        assert!(Rank::from_num(1) == Some(Rank::Rank2));
        assert!(Rank::from_num(2) == Some(Rank::Rank3));
        assert!(Rank::from_num(3) == Some(Rank::Rank4));
        assert!(Rank::from_num(4) == Some(Rank::Rank5));
        assert!(Rank::from_num(5) == Some(Rank::Rank6));
        assert!(Rank::from_num(6) == Some(Rank::Rank7));
        assert!(Rank::from_num(7) == Some(Rank::Rank8));
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
    pub fn rank_as_u8() {
        assert!(Rank::Rank1 as u8 == 0);
        assert!(Rank::Rank8 as u8 == 7);
    }

    #[test]
    pub fn rank_to_char() {
        let map = get_rank_map();
        for (rank, ch) in map {
            let cc = rank.to_char();
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
        map
    }
}
