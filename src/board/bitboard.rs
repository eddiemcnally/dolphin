#[allow(non_camel_case_types)]
use board::square::Square;

// bitboard type
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct BitBoard {
    bits: u64,
}

impl BitBoard {
    pub fn new(val: u64) -> BitBoard {
        BitBoard { bits: val }
    }
    pub fn empty() -> BitBoard {
        BitBoard { bits: 0 }
    }

    pub fn set_bit(&self, sq: Square) -> BitBoard {
        let mask: u64 = to_mask(sq);
        BitBoard::new(self.bits | mask)
    }

    pub fn clear_bit(&self, sq: Square) -> BitBoard {
        let mask: u64 = to_mask(sq);
        BitBoard::new(self.bits & !mask)
    }

    pub fn is_set(&self, sq: Square) -> bool {
        let mask: u64 = to_mask(sq);
        let ret = self.bits & mask;
        ret != 0
    }

    pub fn count_set_bits(&self) -> u8 {
        self.bits.count_ones() as u8
    }

    pub fn pop_1st_bit(&self) -> (BitBoard, Square) {
        let bit_being_cleared = self.bits.trailing_zeros();
        let sq_clear = Square::from_u8(bit_being_cleared as u8);

        let cl_new_bb = BitBoard::clear_bit(self, sq_clear);

        return (cl_new_bb, sq_clear);
    }
}

fn to_mask(sq: Square) -> u64 {
    0x01 << sq as u8
}
