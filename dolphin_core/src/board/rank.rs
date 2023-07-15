use crate::core::array_offset::EnumAsOffset;
use num_enum::TryFromPrimitive;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl EnumAsOffset for Rank {
    fn as_index(&self) -> usize {
        *self as usize
    }
}

impl Rank {
    pub fn new(num: u8) -> Option<Rank> {
        match Rank::try_from(num) {
            Ok(rank) => Some(rank),
            Err(_) => None,
        }
    }

    pub fn add_one(self) -> Option<Rank> {
        Rank::new(self.as_index() as u8 + 1)
    }

    pub fn add_two(self) -> Option<Rank> {
        Rank::new(self.as_index() as u8 + 2)
    }

    pub fn subtract_one(self) -> Option<Rank> {
        Rank::new((self.as_index() as u8).overflowing_sub(1).0)
    }

    pub fn subtract_two(self) -> Option<Rank> {
        Rank::new((self.as_index() as u8).overflowing_sub(2).0)
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
    use crate::core::array_offset::EnumAsOffset;

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
    pub fn rank_as_index() {
        assert!(Rank::R1.as_index() == 0);
        assert!(Rank::R2.as_index() == 1);
        assert!(Rank::R3.as_index() == 2);
        assert!(Rank::R4.as_index() == 3);
        assert!(Rank::R5.as_index() == 4);
        assert!(Rank::R6.as_index() == 5);
        assert!(Rank::R7.as_index() == 6);
        assert!(Rank::R8.as_index() == 7);
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
