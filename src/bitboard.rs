#[allow(non_camel_case_types)]

use square::Square;
use std::mem::transmute;

// bitboard type
pub type BitBoard = u64;

pub trait BitManipulation {
    fn set_bit(&mut self, sq: Square);
    fn clear_bit(&mut self, sq: Square);
    fn is_set(self, sq: Square) -> bool;
    fn count_bits(self) -> u8;
    fn pop_1st_bit(&mut self) -> Square;
}

impl BitManipulation for BitBoard {
    fn set_bit(&mut self, sq: Square) {
        *self = *self | (0x01 << sq as u8);
    }

    fn clear_bit(&mut self, sq: Square) {
        *self = *self & (!(0x01 << sq as u8));
    }

    fn is_set(self, sq: Square) -> bool {
        let ret = self & (0x01 << sq as u8);
        return ret != 0;
    }

    fn count_bits(self) -> u8 {
        return self.count_ones() as u8;
    }

    fn pop_1st_bit(&mut self) -> Square {
        let bit_being_cleared = self.trailing_zeros();

        // todo: find a way of removing the "unsafe" code
        let sq_clear: Square = unsafe { transmute(bit_being_cleared as u8) };

        // clear the bit
        let mask = !(1u64 << bit_being_cleared);
        *self = *self & mask;

        return sq_clear;
    }
}


#[cfg(test)]
pub mod tests {
    use super::BitBoard;
    use super::BitManipulation;
    use square::Square;
    use utils;
    use std::collections::HashMap;



    #[test]
    pub fn test_bit_counting() {
        let mut n: BitBoard = 0b01001100u64;
        assert_eq!(n.count_bits(), 3);

        n = 0b010010010010001001000111100000010011000000001011111111001100u64;
        assert_eq!(n.count_bits(), 24);

        n = 1;
        assert_eq!(n.count_bits(), 1);

        n = 0;
        assert_eq!(n.count_bits(), 0);

        n = 0xffffffffffffffffu64;
        assert_eq!(n.count_bits(), 64);
    }


    #[test]
    pub fn test_pop_bit() {
        let set = utils::get_square_set();

        // set all bits
        let mut bb: BitBoard = 0;
        for sq in &set {
            bb.set_bit(*sq);
        }
        // all bits set
        assert_eq!(0xffffffffffffffffu64, bb);

        // pop all bits
        for sq in &set {
            let mut popped: Square = bb.pop_1st_bit();
            assert_eq!(popped, *sq);
        }

        // all bits should be zero
        assert_eq!(bb as u64, 0);
    }


    #[test]
    pub fn test_bit_manipulation() {
        let map = utils::get_square_rank_file_map();
        for (square, _) in map {

            let mut bb: BitBoard = 0;
            // check the set an check functionality
            bb.set_bit(square);
            let mut is_set = bb.is_set(square);
            assert!(is_set == true);
            assert!(bb as u64 != 0);

            // clear the bit and check it
            bb.clear_bit(square);
            is_set = bb.is_set(square);
            assert!(is_set == false);
            assert!(bb as u64 == 0);
        }
    }


}
