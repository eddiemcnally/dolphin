#[allow(non_camel_case_types)]
use board::square::Square;

// bitboard type
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn empty() -> BitBoard {
        BitBoard(0)
    }

    pub fn set_bit(bb: &BitBoard, sq: &Square) -> BitBoard {
        let new_bb = bb.0 | (0x01 << *sq as u8);
        BitBoard(new_bb)
    }

    pub fn clear_bit(bb: &BitBoard, sq: &Square) -> BitBoard {
        let new_bb = bb.0 & (!(0x01 << *sq as u8));
        BitBoard(new_bb)
    }

    pub fn is_set(bb: &BitBoard, sq: &Square) -> bool {
        let ret = bb.0 & (0x01 << *sq as u8);
        ret != 0
    }

    pub fn count_set_bits(bb: &BitBoard) -> u8 {
        bb.0.count_ones() as u8
    }

    pub fn pop_1st_bit(bb: &BitBoard) -> (BitBoard, Square) {
        let bit_being_cleared = bb.0.trailing_zeros();
        let sq_clear = Square::from_u8(bit_being_cleared as u8);

        let cl_new_bb = BitBoard::clear_bit(&BitBoard(bb.0), &sq_clear);

        return (cl_new_bb, sq_clear);
    }
}
