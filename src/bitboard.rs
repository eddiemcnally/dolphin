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
mod tests {
    use super::BitBoard;
    use super::BitManipulation;
    use square::Square;


    #[test]
    fn test_bit_counting() {
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
    fn test_pop_bit() {

        let mut bb: BitBoard = 0;
        bb.set_bit(Square::a1);
        bb.set_bit(Square::f1);
        bb.set_bit(Square::b2);
        bb.set_bit(Square::h2);
        bb.set_bit(Square::a3);
        bb.set_bit(Square::e3);
        bb.set_bit(Square::h8);


        let mut popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::a1);

        popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::f1);

        popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::b2);

        popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::h2);

        popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::a3);

        popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::e3);

        popped = bb.pop_1st_bit();
        assert_eq!(popped, Square::h8);

        assert_eq!(bb as u64, 0);
    }


    #[test]
    fn test_bit_manipulation() {
        test_set_check_clear_bits(Square::a1);
        test_set_check_clear_bits(Square::a2);
        test_set_check_clear_bits(Square::a3);
        test_set_check_clear_bits(Square::a4);
        test_set_check_clear_bits(Square::a5);
        test_set_check_clear_bits(Square::a6);
        test_set_check_clear_bits(Square::a7);
        test_set_check_clear_bits(Square::a8);

        test_set_check_clear_bits(Square::b1);
        test_set_check_clear_bits(Square::b2);
        test_set_check_clear_bits(Square::b3);
        test_set_check_clear_bits(Square::b4);
        test_set_check_clear_bits(Square::b5);
        test_set_check_clear_bits(Square::b6);
        test_set_check_clear_bits(Square::b7);
        test_set_check_clear_bits(Square::b8);

        test_set_check_clear_bits(Square::h1);
        test_set_check_clear_bits(Square::h2);
        test_set_check_clear_bits(Square::h3);
        test_set_check_clear_bits(Square::h4);
        test_set_check_clear_bits(Square::h5);
        test_set_check_clear_bits(Square::h6);
        test_set_check_clear_bits(Square::h7);
        test_set_check_clear_bits(Square::h8);


    }


    fn test_set_check_clear_bits(sq: Square) {
        let mut bb: BitBoard = 0;

        // check the set an check functionality
        bb.set_bit(sq);
        let mut is_set = bb.is_set(sq);
        assert!(is_set == true);
        assert!(bb as u64 != 0);

        // clear the bit and check it
        bb.clear_bit(sq);
        is_set = bb.is_set(sq);
        assert!(is_set == false);
        assert!(bb as u64 == 0);
    }
}
