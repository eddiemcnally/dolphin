use board::square::Square;
use std::ops::Shl;

// bitboard type
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct BitBoard {
    bits: u64,
}

impl BitBoard {
    pub fn new(val: u64) -> BitBoard {
        BitBoard { bits: val }
    }
    pub fn empty() -> BitBoard {
        BitBoard { bits: 0 }
    }

    pub fn set_bit(&mut self, sq: Square) {
        let mask = to_mask(sq);
        self.bits = self.bits | mask
    }

    pub fn clear_bit(&mut self, sq: Square) {
        let mask = to_mask(sq);
        self.bits = self.bits & !mask
    }

    pub fn is_set(&self, sq: Square) -> bool {
        let mask = to_mask(sq);
        let ret = self.bits & mask;
        ret != 0
    }

    pub fn count_set_bits(&self) -> u8 {
        self.bits.count_ones() as u8
    }

    pub fn pop_1st_bit(&mut self) -> Square {
        let bit_being_cleared = self.bits.trailing_zeros();
        let sq_clear = Square::from_u8(bit_being_cleared as u8);

        self.clear_bit(sq_clear);

        return sq_clear;
    }
}

fn to_mask(sq: Square) -> u64 {
    let bit: u64 = 1;
    bit.shl(sq as u8)
}

#[cfg(test)]
pub mod tests {
    use board::bitboard::BitBoard;
    use utils;

    #[test]
    pub fn test_empty_not_bits_set() {
        let bb = BitBoard::empty();

        let map = utils::get_square_rank_file_map();
        for (sq, (_, _)) in map {
            assert!(bb.is_set(sq) == false);
        }
    }

    #[test]
    pub fn test_set_bit_test_bit_clear_bit() {
        let mut bb = BitBoard::empty();

        let map = utils::get_square_rank_file_map();
        for (sq, (_, _)) in map {
            bb.set_bit(sq);
            assert!(bb.is_set(sq));

            bb.clear_bit(sq);
            assert!(bb.is_set(sq) == false);
        }
    }

    #[test]
    pub fn test_count_set_bits() {
        let map = utils::get_square_rank_file_map();
        let mut i: u8 = 0;
        let mut bb = BitBoard::empty();
        for (square, (_, _)) in map {
            bb.set_bit(square);
            i = i + 1;

            assert_eq!(i, bb.count_set_bits());
        }
    }

    #[test]
    pub fn test_pop_bit_all_bits() {
        let map = utils::get_square_rank_file_map();
        for (square, (_, _)) in map {
            let mut bb = BitBoard::empty();
            bb.set_bit(square);

            let s = bb.pop_1st_bit();

            assert_eq!(s, square);
            assert_eq!(bb.count_set_bits(), 0);
        }
    }
}
