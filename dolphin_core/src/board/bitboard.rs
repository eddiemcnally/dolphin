use crate::board::square::Square;
use crate::core::types::ToInt;
use core::ops::BitOr;
use core::ops::BitOrAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Not;
use std::ops::Shl;
use std::ops::Shr;

const BIT_0: u64 = 0x01;

pub struct SquareIterator(u64);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Default)]
pub struct Bitboard(u64);

impl Bitboard {
    #[inline(always)]
    pub const fn new(bb: u64) -> Bitboard {
        Bitboard(bb)
    }

    #[inline(always)]
    pub fn set_bit(&mut self, sq: Square) {
        let mask = to_mask(sq);
        self.0 |= mask.0
    }

    #[inline(always)]
    pub fn clear_bit(&mut self, sq: Square) {
        let mask = to_mask(sq);
        self.0 &= !mask.0
    }

    #[inline(always)]
    pub fn is_set(&self, sq: Square) -> bool {
        let mask = to_mask(sq);
        (self.0 & mask.0) != 0
    }
    #[inline(always)]
    pub fn is_clear(&self, sq: Square) -> bool {
        let mask = to_mask(sq);
        (self.0 & mask.0) == 0
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn display_squares(&self) {
        let iter = SquareIterator::new(self.0);
        for sq in iter {
            print!("{:?},", sq);
        }
        println!(" ");
    }

    pub fn print_hex(&self) {
        println!("{:#X}", self.0);
    }

    pub fn iterator(&self) -> SquareIterator {
        SquareIterator(self.0)
    }

    #[inline(always)]
    pub const fn reverse_bits(&self) -> Bitboard {
        Bitboard(self.0.reverse_bits())
    }
    #[inline(always)]
    pub const fn overflowing_mul(&self, rhs: u64) -> (u64, bool) {
        let (result, overflowed) = u64::overflowing_mul(self.0, rhs);
        (result, overflowed)
    }
    #[inline(always)]
    pub const fn overflowing_sub(&self, rhs: u64) -> (u64, bool) {
        let (result, overflowed) = u64::overflowing_sub(self.0, rhs);
        (result, overflowed)
    }
}

#[inline(always)]
fn to_mask(sq: Square) -> Bitboard {
    Bitboard(BIT_0.shl(sq.to_u8()))
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Bitboard(self.0 & other.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Bitboard(self.0 | other.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        Bitboard(self.0 ^ other.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl Not for Bitboard {
    fn not(self) -> Self {
        Bitboard(!self.0)
    }
    type Output = Bitboard;
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, shift: u8) -> Self {
        Bitboard(self.0 << shift)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, shift: u8) -> Self {
        Bitboard(self.0 >> shift)
    }
}

impl SquareIterator {
    pub fn new(num: u64) -> SquareIterator {
        SquareIterator(num)
    }
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 0 {
            let sq = Square::new(self.0.trailing_zeros() as u8);
            self.0 &= self.0 - 1;
            return sq;
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::Bitboard;
    use crate::board::bitboard::SquareIterator;
    use crate::board::square::{self, Square};
    use std::u64;

    #[test]
    pub fn set_msb_check_square_as_h8() {
        let bb: u64 = 0x8000000000000000;
        let iter = SquareIterator::new(bb);
        for sq in iter {
            assert_eq!(sq, Square::H8);
        }
    }

    #[test]
    pub fn set_lsb_check_square_as_a1() {
        let bb: u64 = 0x0000000000000001;
        let iter = SquareIterator::new(bb);
        for sq in iter {
            assert_eq!(sq, Square::A1);
        }
    }

    #[test]
    pub fn set_bit_test_bit_clear_bit() {
        let mut bb = Bitboard::new(0);

        for sq in square::iterator() {
            bb.set_bit(*sq);
            assert!(bb.is_set(*sq));
            assert!(bb.0 != 0);

            bb.clear_bit(*sq);
            assert!(!bb.is_set(*sq));
            assert!(bb.0 == 0);
        }
    }

    #[test]
    pub fn pop_bit_all_bits() {
        for sq in square::iterator() {
            let mut bb = Bitboard::new(0);
            bb.set_bit(*sq);
            for sqq in bb.iterator() {
                assert_eq!(sqq, *sq);
            }
        }
    }
}
