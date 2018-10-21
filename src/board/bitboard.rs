#[allow(non_camel_case_types)]
use board::square::Square;

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
        let sq_clear = Square::from_u8(bit_being_cleared as u8);
        self.clear_bit(sq_clear);
        return sq_clear;
    }
}
