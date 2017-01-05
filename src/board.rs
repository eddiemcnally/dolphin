#[allow(dead_code)]
#[allow(non_camel_case_types)]

use bitboard::BitBoard;
use piece;
use square::Square;

#[allow(dead_code)]
pub enum CastlePermission {
    WK = 0x01,
    WQ = 0x02,
    BK = 0x04,
    BQ = 0x08,
}



#[allow(dead_code)]
pub struct Board {
    board_bb: BitBoard,
    piece_bb: [BitBoard; 12],
    colour_bb: [BitBoard; 2],
    side_to_move: piece::Colour,
    en_pass_sq: Square,
    castle_perm: u8,
}



#[allow(dead_code)]
pub const NUM_SQUARES: usize = 64;
