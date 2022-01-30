use crate::core::types::ToInt;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Rank(u8);

impl Rank {
    pub const fn new(num: u8) -> Option<Rank> {
        if num <= Rank::R8.0 {
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
            Rank::R1 => None,
            _ => Rank::new(self.0 - 1),
        }
    }

    pub fn subtract_two(&self) -> Option<Rank> {
        match *self {
            Rank::R1 | Rank::R2 => None,
            _ => Rank::new(self.0 - 2),
        }
    }

    pub fn from_char(rank: char) -> Option<Rank> {
        match rank {
            '1' => Some(Rank::R1),
            '2' => Some(Rank::R2),
            '3' => Some(Rank::R3),
            '4' => Some(Rank::R4),
            '5' => Some(Rank::R5),
            '6' => Some(Rank::R6),
            '7' => Some(Rank::R7),
            '8' => Some(Rank::R8),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            Rank::R1 => '1',
            Rank::R2 => '2',
            Rank::R3 => '3',
            Rank::R4 => '4',
            Rank::R5 => '5',
            Rank::R6 => '6',
            Rank::R7 => '7',
            Rank::R8 => '8',
            _ => panic!("Invalid Rank {}", *self),
        }
    }

    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            Rank::R1,
            Rank::R2,
            Rank::R3,
            Rank::R4,
            Rank::R5,
            Rank::R6,
            Rank::R7,
            Rank::R8,
        ];
        RANKS.iter()
    }

    pub fn reverse_iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            Rank::R8,
            Rank::R7,
            Rank::R6,
            Rank::R5,
            Rank::R4,
            Rank::R3,
            Rank::R2,
            Rank::R1,
        ];
        RANKS.iter()
    }

    pub const R1: Rank = Rank(0);
    pub const R2: Rank = Rank(1);
    pub const R3: Rank = Rank(2);
    pub const R4: Rank = Rank(3);
    pub const R5: Rank = Rank(4);
    pub const R6: Rank = Rank(5);
    pub const R7: Rank = Rank(6);
    pub const R8: Rank = Rank(7);
}

impl ToInt for Rank {
    fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    fn to_usize(&self) -> usize {
        self.0 as usize
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
        assert!(Rank::new(0) == Some(Rank::R1));
        assert!(Rank::new(1) == Some(Rank::R2));
        assert!(Rank::new(2) == Some(Rank::R3));
        assert!(Rank::new(3) == Some(Rank::R4));
        assert!(Rank::new(4) == Some(Rank::R5));
        assert!(Rank::new(5) == Some(Rank::R6));
        assert!(Rank::new(6) == Some(Rank::R7));
        assert!(Rank::new(7) == Some(Rank::R8));
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
        assert!(Rank::R1.0 as u8 == 0);
        assert!(Rank::R8.0 as u8 == 7);
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
        map.insert(Rank::R1, '1');
        map.insert(Rank::R2, '2');
        map.insert(Rank::R3, '3');
        map.insert(Rank::R4, '4');
        map.insert(Rank::R5, '5');
        map.insert(Rank::R6, '6');
        map.insert(Rank::R7, '7');
        map.insert(Rank::R8, '8');
        map
    }
}
