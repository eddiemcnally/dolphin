use board::bitboard::BitBoard;
use board::piece::Piece;
use board::piece::NUM_COLOURS;
use board::piece::NUM_PIECES;
use board::square::Square;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

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

impl Board {
    pub fn new() -> Board {
        return Board {
            board_bb: BitBoard::empty(),
            piece_bb: [BitBoard::empty(); NUM_PIECES],
            colour_bb: [BitBoard::empty(); NUM_COLOURS],
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
            self.is_sq_empty(sq) == false,
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
        return self.pieces[sq.to_offset()];
    }

    pub fn is_sq_empty(&self, sq: Square) -> bool {
        self.board_bb.is_set(sq) == false
    }

    pub fn get_bitboard(&self, piece: Piece) -> BitBoard {
        return self.piece_bb[piece.offset()];
    }

    fn set_bitboards(&mut self, piece: Piece, sq: Square) {
        self.board_bb.set_bit(sq);
        self.piece_bb[piece.offset()].set_bit(sq);
        self.pieces[sq.to_offset()] = Some(piece);
        self.colour_bb[piece.colour().offset()].set_bit(sq);
    }

    fn clear_bitboards(&mut self, piece: Piece, sq: Square) {
        self.board_bb.clear_bit(sq);
        self.piece_bb[piece.offset()].clear_bit(sq);
        self.pieces[sq.to_offset()] = None;
        self.colour_bb[piece.colour().offset()].clear_bit(sq);
    }
}
#[cfg(test)]
pub mod tests {
    use board::board::Board;
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;
    use utils;

    #[test]
    pub fn test_add_remove_piece_square_state_as_expected() {
        let pce = Piece::new(PieceRole::Knight, Colour::White);
        let mut board = Board::new();

        let map = utils::get_square_rank_file_map();
        for (square, _) in map {
            assert!(board.is_sq_empty(square) == true);

            board.add_piece(pce, square);
            assert!(board.is_sq_empty(square) == false);

            board.remove_piece(pce, square);
            assert!(board.is_sq_empty(square) == true);
        }
    }

    #[test]
    pub fn test_move_piece_square_state_as_expected() {
        let pce = Piece::new(PieceRole::Knight, Colour::White);
        let mut board = Board::new();

        for (from_sq, _) in utils::get_square_rank_file_map() {
            for (to_sq, _) in utils::get_square_rank_file_map() {
                if from_sq == to_sq {
                    continue;
                }

                assert!(board.is_sq_empty(from_sq) == true);
                assert!(board.is_sq_empty(to_sq) == true);

                board.add_piece(pce, from_sq);
                assert!(board.is_sq_empty(from_sq) == false);
                assert!(board.is_sq_empty(to_sq) == true);

                board.move_piece(from_sq, to_sq, pce);
                assert!(board.is_sq_empty(from_sq) == true);
                assert!(board.is_sq_empty(to_sq) == false);

                // clean up
                board.remove_piece(pce, to_sq);
            }
        }
    }

    #[test]
    pub fn test_get_piece_on_square_as_expected() {
        let pce = Piece::new(PieceRole::Knight, Colour::White);
        let mut board = Board::new();

        let map = utils::get_square_rank_file_map();
        for (square, _) in map {
            assert!(board.is_sq_empty(square) == true);

            board.add_piece(pce, square);
            assert!(board.is_sq_empty(square) == false);

            let retr_pce: Option<Piece> = board.get_piece_on_square(square);
            assert_eq!(retr_pce.is_some(), true);
            assert_eq!(retr_pce.unwrap(), pce);

            // clean up
            board.remove_piece(pce, square);
        }
    }

    #[test]
    pub fn test_get_bitboard_value_as_expected() {
        let mut board = Board::new();

        for pce in utils::get_all_pieces() {
            for (square, _) in utils::get_square_rank_file_map() {
                board.add_piece(pce, square);
                let bb = board.get_bitboard(pce);

                assert!(bb.is_set(square));

                // clean up
                board.remove_piece(pce, square);
            }
        }
    }

}
