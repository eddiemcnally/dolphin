#[allow(dead_code)]
#[allow(non_camel_case_types)]

use bitboard::BitBoard;
use piece;
use square;
// use bitboard::BitManipulation;


// TODO: look into decomposing this into a set of sub-structs and Impl's
// to improve manageability
#[allow(dead_code)]
pub struct Board {
    // bitboard representing occupied/vacant squares (for all pieces)
    board_bb: BitBoard,
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [BitBoard; piece::NUM_PIECES],
    // bitboard for each Colour
    colour_bb: [BitBoard; piece::NUM_COLOURS],
}


impl Board {
    pub fn add_piece(&self) {
        // *self.board_bb.set_bit(sq);
        // *self.piece_bb[pce as u64].set_bit(sq);
        // *self.colour_bb[pce.colour() as u8].set_bit(sq);
    }

    pub fn remove_piece(&self) {}
}







#[allow(dead_code)]
pub const NUM_SQUARES: usize = 64;
