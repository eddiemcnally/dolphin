use super::types::ToInt;
use crate::board::square::Square;
use std::ops::Shl;

const BIT_0: u64 = 0x01;

pub struct SquareIterator {
    bb: u64,
}

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

pub fn display_squares(bb: u64) {
    let iter = SquareIterator::new(bb);
    for sq in iter {
        print!("{:?},", sq);
    }
    println!(" ");
}

pub fn get_square_iterator(bb: u64) -> SquareIterator {
    SquareIterator::new(bb)
}

pub fn print_hex(bb: u64) {
    println!("{:#064X}", bb);
}

pub fn to_mask(sq: Square) -> u64 {
    BIT_0.shl(sq.to_u8())
}

impl SquareIterator {
    pub fn new(num: u64) -> SquareIterator {
        SquareIterator { bb: num }
    }
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bb > 0 {
            let sq = Square::new(self.bb.trailing_zeros() as u8);
            self.bb &= self.bb - 1;
            return sq;
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::bitboard;
    use crate::board::bitboard::SquareIterator;
    use crate::board::square;
    use crate::board::square::*;
    use std::u64;

    #[test]
    pub fn set_msb_check_square_as_h8() {
        let bb: u64 = 0x8000000000000000;
        let iter = SquareIterator::new(bb);
        for sq in iter {
            assert_eq!(sq, SQUARE_H8);
        }
    }

    #[test]
    pub fn set_lsb_check_square_as_a1() {
        let bb: u64 = 0x0000000000000001;
        let iter = SquareIterator::new(bb);
        for sq in iter {
            assert_eq!(sq, SQUARE_A1);
        }
    }

    #[test]
    pub fn set_bit_test_bit_clear_bit() {
        let mut bb: u64 = 0;

        let map = square::SQUARES;
        for sq in map {
            bb = bitboard::set_bit(bb, *sq);
            assert!(bitboard::is_set(bb, *sq));

            bb = bitboard::clear_bit(bb, *sq);
            assert!(!bitboard::is_set(bb, *sq));
        }
    }

    #[test]
    pub fn pop_bit_all_bits() {
        let map = square::SQUARES;
        for square in map {
            let bb = bitboard::set_bit(0, *square);
            let iter = SquareIterator::new(bb);
            for sq in iter {
                assert_eq!(sq, *square);
            }
        }
    }
}
