#[allow(dead_code)]
#[allow(non_camel_case_types)]

use board::piece::Piece;
use board::piece::NUM_PIECES;
use board::piece::NUM_COLOURS;
use board::bitboard::BitBoard;
use board::bitboard::BitManipulation;
use board::square::Square;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

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
}

//let array: [Option<Box<Thing>>; SIZE] = Default::default();

impl Board {
    pub fn new() -> Board {
        return Board {
            board_bb: BitBoard::default(),
            piece_bb: [BitBoard::default(); NUM_PIECES],
            colour_bb: [BitBoard::default(); NUM_COLOURS],
            pieces: [None; NUM_SQUARES],
        };
    }

    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "add_piece, square not empty. {:?}",
            sq
        );

        self.set_bitboards(piece, sq);
    }

    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "remove_piece, square is empty. {:?}",
            sq
        );

        self.clear_bitboards(piece, sq);
    }

    pub fn move_piece(&mut self, from_sq: Square, to_sq: Square, piece: Piece) {
        self.clear_bitboards(piece, from_sq);
        self.set_bitboards(piece, to_sq);
    }

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        return self.pieces[sq as usize];
    }

    fn is_sq_empty(&self, sq: Square) -> bool {
        if self.board_bb.is_set(sq) == true {
            return false;
        }
        return true;
    }

    pub fn get_bitboard(&self, piece: Piece) -> BitBoard {
        return self.piece_bb[piece as usize];
    }


    fn set_bitboards(&mut self, piece: Piece, sq: Square) {
        self.board_bb.set_bit(sq);
        self.piece_bb[piece as usize].set_bit(sq);
        self.pieces[sq as usize] = Some(piece);
        let col = piece.colour();
        self.colour_bb[col as usize].set_bit(sq);
    }


    fn clear_bitboards(&mut self, piece: Piece, sq: Square) {
        self.board_bb.clear_bit(sq);
        self.piece_bb[piece as usize].clear_bit(sq);
        self.pieces[sq as usize] = None;
        let col = piece.colour();
        self.colour_bb[col as usize].clear_bit(sq);
    }
}
