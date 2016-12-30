#[allow(non_camel_case_types)]

use board;

// bitboard type
pub type BitBoard = u64;


pub fn set_bit(bb: &mut BitBoard, sq: board::Square) {
    *bb = *bb | (0x01 << sq as u8);
}

pub fn clear_bit(bb: &mut BitBoard, sq: board::Square) {
    *bb = *bb & (!(0x01 << sq as u8));
}

pub fn check_bit(bb: BitBoard, sq: board::Square) -> bool{
    let ret = bb & (0x01 << sq as u8);
    return ret != 0;
}


#[cfg(test)]
mod tests {
    use super::set_bit;
    use super::clear_bit;
    use super::check_bit;
    use super::BitBoard;
    use board;

    #[test]
    fn test_all_bits(){
        test_set_check_clear_bits(board::Square::a1);
        test_set_check_clear_bits(board::Square::a2);
        test_set_check_clear_bits(board::Square::a3);
        test_set_check_clear_bits(board::Square::a4);
        test_set_check_clear_bits(board::Square::a5);
        test_set_check_clear_bits(board::Square::a6);
        test_set_check_clear_bits(board::Square::a7);
        test_set_check_clear_bits(board::Square::a8);

        test_set_check_clear_bits(board::Square::b1);
        test_set_check_clear_bits(board::Square::b2);
        test_set_check_clear_bits(board::Square::b3);
        test_set_check_clear_bits(board::Square::b4);
        test_set_check_clear_bits(board::Square::b5);
        test_set_check_clear_bits(board::Square::b6);
        test_set_check_clear_bits(board::Square::b7);
        test_set_check_clear_bits(board::Square::b8);

        test_set_check_clear_bits(board::Square::h1);
        test_set_check_clear_bits(board::Square::h2);
        test_set_check_clear_bits(board::Square::h3);
        test_set_check_clear_bits(board::Square::h4);
        test_set_check_clear_bits(board::Square::h5);
        test_set_check_clear_bits(board::Square::h6);
        test_set_check_clear_bits(board::Square::h7);
        test_set_check_clear_bits(board::Square::h8);


    }


    fn test_set_check_clear_bits(sq: board::Square) {
        let mut bb: BitBoard = 0;

        set_bit(&mut bb, sq);
        let is_set = check_bit(bb, sq);
        assert!(is_set == true);

        clear_bit(&mut bb, sq);
        let is_set_1 = check_bit(bb, sq);
        assert!(is_set_1 == false);
    }
}
