use board::square::Square;
use core::core_traits::ArrayAccessor;
use std::ops::Shl;

const BIT_0: u64 = 0x01;

pub fn set_bit(bb: &mut u64, sq: Square) {
    let mask = to_mask(sq);
    *bb = *bb | mask
}

pub fn clear_bit(bb: &mut u64, sq: Square) {
    let mask = to_mask(sq);
    *bb = *bb & !mask
}

pub fn is_set(bb: u64, sq: Square) -> bool {
    let mask = to_mask(sq);
    let ret = bb & mask;
    ret != 0
}

pub fn count_bits(bb: u64) -> u8 {
    return bb.count_ones() as u8;
}

pub fn pop_1st_bit(bb: &mut u64) -> Square {
    debug_assert!(*bb != 0, "bitboard is already zero");

    let bit_being_cleared = bb.trailing_zeros();
    let sq_clear = Square::from_num(bit_being_cleared as u8).unwrap();

    clear_bit(bb, sq_clear);
    return sq_clear;
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

fn to_mask(sq: Square) -> u64 {
    BIT_0.shl(sq.to_offset())
}

#[cfg(test)]
pub mod tests {
    use board::bitboard;
    use board::square::Square;
    use std::u64;
    use utils;

    #[test]
    pub fn set_msb_check_square_as_h8() {
        let mut bb: u64 = 0x8000000000000000;
        let sq = bitboard::pop_1st_bit(&mut bb);
        assert_eq!(sq, Square::h8);
    }

    #[test]
    pub fn set_bit_test_bit_clear_bit() {
        let mut bb: u64 = 0;

        let map = utils::get_square_rank_file_map();
        for (sq, (_, _)) in map {
            bitboard::set_bit(&mut bb, sq);
            assert!(bitboard::is_set(bb, sq));

            bitboard::clear_bit(&mut bb, sq);
            assert!(bitboard::is_set(bb, sq) == false);
        }
    }

    #[test]
    pub fn pop_bit_all_bits() {
        let map = utils::get_square_rank_file_map();
        for (square, (_, _)) in map {
            let mut bb: u64 = 0;
            bitboard::set_bit(&mut bb, square);

            let s = bitboard::pop_1st_bit(&mut bb);

            assert_eq!(s, square);
            assert_eq!(bb, 0);
        }
    }

    #[test]
    pub fn pop_all_bits_squares_as_expected() {
        let mut bb: u64 = 0x1;

        let sqs = utils::get_ordered_square_list_by_file();

        for sq in sqs {
            let mut temp_bb = bb;

            let popped_sq = bitboard::pop_1st_bit(&mut temp_bb);
            assert_eq!(popped_sq, sq);

            bb = bb << 1;
        }
    }
}
