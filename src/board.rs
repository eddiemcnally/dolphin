#[allow(dead_code)]
#[allow(non_camel_case_types)]

use bitboard::BitBoard;
use piece::Piece;
use piece::NUM_PIECES;
use piece::NUM_COLOURS;
use square::Square;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

// TODO: look into decomposing this into a set of sub-structs and Impl's
// to improve manageability
#[allow(dead_code)]
pub struct Board {
    // bitboard representing occupied/vacant squares (for all pieces)
    board_bb: BitBoard,
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [BitBoard; NUM_PIECES],
    // bitboard for each Colour
    colour_bb: [BitBoard; NUM_COLOURS],
    // the pieces on each square
    pieces: [Piece; NUM_SQUARES],
}


impl Board {
    pub fn add_piece(&self, piece: Piece, sq: Square) {
        // *self.board_bb.set_bit(sq);
        // *self.piece_bb[pce as u64].set_bit(sq);
        // *self.colour_bb[pce.colour() as u8].set_bit(sq);
    }

    pub fn remove_piece(&self, piece: Piece, sq: Square) {}

    pub fn move_piece(&self, from_sq: Square, to_sq: Square, piece: Piece) {}

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        return None;
    }
}
