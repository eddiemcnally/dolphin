#[allow(dead_code)]
#[allow(non_camel_case_types)]

use bitboard::BitBoard;
use piece;
use square::Square;
// use bitboard::BitManipulation;

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
    piece_bb: [BitBoard; piece::NUM_PIECES],
    colour_bb: [BitBoard; piece::NUM_COLOURS],
    side_to_move: piece::Colour,
    en_pass_sq: Square,
    castle_perm: u8,
}


impl Board {
    pub fn add_piece(&self, pce: piece::Piece, sq: Square) {
        // *self.board_bb.set_bit(sq);
        // *self.piece_bb[pce as u64].set_bit(sq);
        // *self.colour_bb[pce.colour() as u8].set_bit(sq);
    }

    pub fn remove_piece(&self, pce: piece::Piece) {}
}







#[allow(dead_code)]
pub const NUM_SQUARES: usize = 64;
