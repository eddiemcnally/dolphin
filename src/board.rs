#[allow(dead_code)]
#[allow(non_camel_case_types)]

use bitboard::BitBoard;
use bitboard::BitManipulation;
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
    pieces: [Option<Piece>; NUM_SQUARES],

    // todo - add material,
}


impl Board {
    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        if self.is_sq_empty(sq) == false {
            panic!("add_piece, square not empty. {:?}", sq);
        }

        self.board_bb.set_bit(sq);
        self.piece_bb[piece as usize].set_bit(sq);
        self.pieces[sq as usize] = Some(piece);

        let col = piece.colour();
        self.colour_bb[col as usize].set_bit(sq);
    }

    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        if self.is_sq_empty(sq) == true {
            panic!("remove_piece, square is empty. {:?}", sq);
        }

        self.board_bb.clear_bit(sq);
        self.piece_bb[piece as usize].clear_bit(sq);
        self.pieces[sq as usize] = None;

        let col = piece.colour();
        self.colour_bb[col as usize].clear_bit(sq);
    }

    pub fn move_piece(&mut self, from_sq: Square, to_sq: Square, piece: Piece) {
        self.remove_piece(piece, from_sq);
        self.add_piece(piece, to_sq);
    }

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        return self.pieces[sq as usize];
    }

    pub fn assert_board_ok(&self) {}


    pub fn get_bitboard(&self, piece: Piece) -> BitBoard {
        return self.piece_bb[piece as usize];
    }


    fn is_sq_empty(&self, sq: Square) -> bool {
        if self.board_bb.is_set(sq) == true {
            return false;
        }
        return true;
    }
}
