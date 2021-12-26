use crate::board::square::Square;
use std::ops::Shl;

const BIT_0: u64 = 0x01;

pub fn set_bit(bb: u64, sq: Square) -> u64 {
    let mask = to_mask(sq);
    bb | mask
}

pub fn clear_bit(bb: u64, sq: Square) -> u64 {
    let mask = to_mask(sq);
    bb & !mask
}

pub fn is_set(bb: u64, sq: Square) -> bool {
    let mask = to_mask(sq);
    (bb & mask) != 0
}

pub fn is_clear(bb: u64, sq: Square) -> bool {
    let mask = to_mask(sq);
    (bb & mask) == 0
}

pub fn pop_1st_bit(bb: &mut u64) -> Square {
    debug_assert!(*bb != 0, "bitboard is already zero");

    let bit_being_cleared = bb.trailing_zeros();
    let sq_clear = Square::from_num(bit_being_cleared as u64).unwrap();

    *bb = clear_bit(*bb, sq_clear);
    sq_clear
}

pub fn display_squares(bb: u64) {
    let mut slider = bb;
    while slider != 0 {
        let sq = pop_1st_bit(&mut slider);
        print!("{:?},", sq);
    }
    println!(" ");
}

pub fn print_hex(bb: u64) {
    println!("{:#064X}", bb);
}
pub fn to_mask(sq: Square) -> u64 {
    BIT_0.shl(sq.offset())
}

#[cfg(test)]
pub mod tests {
    use crate::board::bitboard;
    use crate::board::square;
    use crate::board::square::Square;
    use std::u64;

    #[test]
    pub fn set_msb_check_square_as_h8() {
        let mut bb: u64 = 0x8000000000000000;
        let sq = bitboard::pop_1st_bit(&mut bb);
        assert_eq!(sq, Square::h8);
    }

    #[test]
    pub fn set_bit_test_bit_clear_bit() {
        let mut bb: u64 = 0;

        let map = square::SQUARES;
        for sq in map {
            bb = bitboard::set_bit(bb, *sq);
            assert!(bitboard::is_set(bb, *sq));

            bb = bitboard::clear_bit(bb, *sq);
            assert!(bitboard::is_set(bb, *sq) == false);
        }
    }

    #[test]
    pub fn pop_bit_all_bits() {
        let map = square::SQUARES;
        for square in map {
            let mut bb = bitboard::set_bit(0, *square);
            let s = bitboard::pop_1st_bit(&mut bb);

            assert_eq!(s, *square);
            assert_eq!(bb, 0);
        }
    }

    #[test]
    pub fn pop_all_bits_squares_as_expected() {
        let mut bb: u64 = 0x1;

        let sqs = square::SQUARES;

        for sq in sqs {
            let mut temp_bb = bb;

            let popped_sq = bitboard::pop_1st_bit(&mut temp_bb);
            assert_eq!(popped_sq, *sq);

            bb = bb << 1;
        }
    }
}
