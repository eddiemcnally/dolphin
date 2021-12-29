use std::fmt;
use std::slice::Iter;

use super::types::ToInt;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Rank(u8);

pub const RANK_1: Rank = Rank(0);
pub const RANK_2: Rank = Rank(1);
pub const RANK_3: Rank = Rank(2);
pub const RANK_4: Rank = Rank(3);
pub const RANK_5: Rank = Rank(4);
pub const RANK_6: Rank = Rank(5);
pub const RANK_7: Rank = Rank(6);
pub const RANK_8: Rank = Rank(7);

impl ToInt for Rank {
    fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Rank {
    pub fn new(num: u8) -> Option<Rank> {
        if num <= RANK_8.0 {
            return Some(Rank(num));
        }
        None
    }

    pub fn add_one(&self) -> Option<Rank> {
        Rank::new(self.0 + 1)
    }

    pub fn add_two(&self) -> Option<Rank> {
        Rank::new(self.0 + 2)
    }

    pub fn subtract_one(&self) -> Option<Rank> {
        match *self {
            RANK_1 => None,
            _ => Rank::new(self.0 - 1),
        }
    }

    pub fn subtract_two(&self) -> Option<Rank> {
        match *self {
            RANK_1 | RANK_2 => None,
            _ => Rank::new(self.0 - 2),
        }
    }

    pub fn from_char(rank: char) -> Option<Rank> {
        match rank {
            '1' => Some(RANK_1),
            '2' => Some(RANK_2),
            '3' => Some(RANK_3),
            '4' => Some(RANK_4),
            '5' => Some(RANK_5),
            '6' => Some(RANK_6),
            '7' => Some(RANK_7),
            '8' => Some(RANK_8),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            RANK_1 => '1',
            RANK_2 => '2',
            RANK_3 => '3',
            RANK_4 => '4',
            RANK_5 => '5',
            RANK_6 => '6',
            RANK_7 => '7',
            RANK_8 => '8',
            _ => panic!("Invalid Rank {}", *self),
        }
    }

    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
        ];
        RANKS.iter()
    }

    pub fn reverse_iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            RANK_8, RANK_7, RANK_6, RANK_5, RANK_4, RANK_3, RANK_2, RANK_1,
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
    use crate::board::rank::*;
    use std::collections::HashMap;

    #[test]
    pub fn rank_from_u8() {
        assert!(Rank::new(0) == Some(RANK_1));
        assert!(Rank::new(1) == Some(RANK_2));
        assert!(Rank::new(2) == Some(RANK_3));
        assert!(Rank::new(3) == Some(RANK_4));
        assert!(Rank::new(4) == Some(RANK_5));
        assert!(Rank::new(5) == Some(RANK_6));
        assert!(Rank::new(6) == Some(RANK_7));
        assert!(Rank::new(7) == Some(RANK_8));
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
        assert!(RANK_1.0 as u8 == 0);
        assert!(RANK_8.0 as u8 == 7);
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
        map.insert(RANK_1, '1');
        map.insert(RANK_2, '2');
        map.insert(RANK_3, '3');
        map.insert(RANK_4, '4');
        map.insert(RANK_5, '5');
        map.insert(RANK_6, '6');
        map.insert(RANK_7, '7');
        map.insert(RANK_8, '8');
        map
    }
}
