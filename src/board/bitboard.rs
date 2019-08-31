use board::square::Square;
use std::ops::Shl;


pub fn set_bit(bb:&mut u64, sq: Square){
    let mask = to_mask(sq);
    *bb = *bb | mask
}

pub fn clear_bit(bb: &mut u64, sq: Square){
    let mask = to_mask(sq);
    *bb = *bb & !mask
}

pub fn is_set(bb:u64, sq: Square) -> bool {
    let mask = to_mask(sq);
    let ret = bb & mask;
    ret != 0
}

pub fn count_set_bits(bb:u64) -> u8 {
    bb.count_ones() as u8
}

pub fn pop_1st_bit(bb:&mut u64) -> Square {
    let bit_being_cleared = bb.trailing_zeros();
    let sq_clear = Square::from_u8(bit_being_cleared as u8);

    clear_bit(bb, sq_clear);

    return sq_clear;
}

pub fn display_squares(bb: u64){
    let mut slider = bb;
    while slider != 0 {
        let sq = pop_1st_bit(& mut slider);
        print!("{:?},", sq);
    }
    println!(" ");
}



fn to_mask(sq: Square) -> u64 {
    let bit: u64 = 1;
    bit.shl(sq.to_offset())
}





#[cfg(test)]
pub mod tests {
    use utils;
    use board::bitboard;

    #[test]
    pub fn set_bit_test_bit_clear_bit() {
        let mut bb:u64  = 0;

        let map = utils::get_square_rank_file_map();
        for (sq, (_, _)) in map {
            bitboard::set_bit(&mut bb, sq);
            assert!(bitboard::is_set(bb, sq));

            bitboard::clear_bit(&mut bb, sq);
            assert!(bitboard::is_set(bb, sq) == false);
        }
    }

    #[test]
    pub fn count_set_bits() {
        let map = utils::get_square_rank_file_map();
        let mut i: u8 = 0;
        let mut bb:u64 = 0;
        for (square, (_, _)) in map {
            bitboard::set_bit(&mut bb, square);
            i = i + 1;

            assert_eq!(i, bitboard::count_set_bits(bb));
        }
    }

    #[test]
    pub fn pop_bit_all_bits() {
        let map = utils::get_square_rank_file_map();
        for (square, (_, _)) in map {
            let mut bb:u64 = 0;
            bitboard::set_bit(&mut bb, square);

            let s = bitboard::pop_1st_bit(&mut bb);

            assert_eq!(s, square);
            assert_eq!(bitboard::count_set_bits(bb), 0);
        }
    }
}
