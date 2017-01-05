#[allow(non_camel_case_types)]

use square::Square;
use std::mem::transmute;

// bitboard type
pub type BitBoard = u64;


pub fn set_bit(bb: &mut BitBoard, sq: Square) {
    *bb = *bb | (0x01 << sq as u8);
}

pub fn clear_bit(bb: &mut BitBoard, sq: Square) {
    *bb = *bb & (!(0x01 << sq as u8));
}

pub fn check_bit(bb: BitBoard, sq: Square) -> bool {
    let ret = bb & (0x01 << sq as u8);
    return ret != 0;

}

/// Counts the number of set bits in the BitBoard
pub fn count_bits(bb: BitBoard) -> u8 {
    return bb.count_ones() as u8;
}

/// Clears the LSB of the board, and returns the bit # that was cleared.
pub fn pop_1st_bit(bb: &mut BitBoard) -> Square {
    let bit_being_cleared = bb.trailing_zeros();

    // todo: find a way of removing the "unsafe" code
    let sq_clear: Square = unsafe { transmute(bit_being_cleared as u8) };

    // clear the bit
    let mask = !(1u64 << bit_being_cleared);
    *bb = *bb & mask;

    return sq_clear;
}






#[cfg(test)]
mod tests {
    use super::set_bit;
    use super::clear_bit;
    use super::check_bit;
    use super::count_bits;
    use super::pop_1st_bit;
    use super::BitBoard;
    use square::Square;


    #[test]
    fn test_bit_counting() {
        let n = 0b01001100u8;
        assert_eq!(count_bits(n as BitBoard), 3);
        let m = 0b010010010010001001000111100000010011000000001011111111001100u64;
        assert_eq!(count_bits(m as BitBoard), 24);
        let p = 1;
        assert_eq!(count_bits(p as BitBoard), 1);
        let q = 0;
        assert_eq!(count_bits(q as BitBoard), 0);
        let r = 0xffffffffffffffffu64;
        assert_eq!(count_bits(r as BitBoard), 64);

    }


    #[test]
    fn test_pop_bit() {

        let mut bb: BitBoard = 0;
        set_bit(&mut bb, Square::a1);
        set_bit(&mut bb, Square::f1);
        set_bit(&mut bb, Square::b2);
        set_bit(&mut bb, Square::h2);
        set_bit(&mut bb, Square::a3);
        set_bit(&mut bb, Square::e3);
        set_bit(&mut bb, Square::h8);


        let mut popped = pop_1st_bit(&mut bb);
        assert_eq!(popped, Square::a1);

        popped = pop_1st_bit(&mut bb);
        assert_eq!(popped, Square::f1);

        popped = pop_1st_bit(&mut bb);
        assert_eq!(popped, Square::b2);

        popped = pop_1st_bit(&mut bb);
        assert_eq!(popped, Square::h2);

        popped = pop_1st_bit(&mut bb);
        assert_eq!(popped, Square::a3);

        popped = pop_1st_bit(&mut bb);
        assert_eq!(popped, Square::e3);

        popped = pop_1st_bit(&mut bb);
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
        set_bit(&mut bb, sq);
        let mut is_set = check_bit(bb, sq);
        assert!(is_set == true);
        assert!(bb as u64 != 0);

        // clear the bit and check it
        clear_bit(&mut bb, sq);
        is_set = check_bit(bb, sq);
        assert!(is_set == false);
        assert!(bb as u64 == 0);
    }
}
