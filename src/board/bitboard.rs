#[allow(non_camel_case_types)]
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
    use board::square::Square;
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
        let mut bb = BitBoard::new(0x2);
        assert!(bb.count_set_bits() == 1);

        bb = BitBoard::new(0x3);
        assert!(bb.count_set_bits() == 2);

        bb = BitBoard::new(0xc027deab4563bd21);
        assert!(bb.count_set_bits() == 32);
    }

    #[test]
    pub fn test_pop_bit() {
        let mut bb = BitBoard::new(0x1);
        let mut sq = bb.pop_1st_bit();
        assert!(bb.count_set_bits() == 0);
        assert!(sq == Square::a1);

        bb = BitBoard::new(0xFF0102);
        assert!(bb.count_set_bits() == 10);
        sq = bb.pop_1st_bit();
        assert!(bb.count_set_bits() == 9);
        assert!(sq == Square::b1);
    }

}
