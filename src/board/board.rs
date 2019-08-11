use board::bitboard;
use board::piece::Colour;
use board::piece::Piece;
use board::piece::NUM_COLOURS;
use board::piece::NUM_PIECES;
use board::square::Square;
use input::fen::ParsedFen;
use std::option::Option;


pub const NUM_SQUARES: usize = 64;

pub struct Board {
    // bitboard representing occupied/vacant squares (for all pieces)
    board_bb: u64,
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [u64; NUM_PIECES],
    // bitboard for each Colour
    colour_bb: [u64; NUM_COLOURS],
    // the pieces on each square
    pieces: [Option<Piece>; NUM_SQUARES],
}


impl Board {
    pub fn new() -> Board {
        return Board {
            board_bb: 0,
            piece_bb: [0; NUM_PIECES],
            colour_bb: [0; NUM_COLOURS],
            pieces: [None; NUM_SQUARES],
        };
    }

    pub fn from_fen(parsed_fen:&ParsedFen) -> Board{
        let mut brd = Board::new();

        let positions = parsed_fen.piece_positions.iter();
        for (sq, pce) in positions{
            brd.add_piece(*pce, *sq);
        }
        return brd;
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

    pub fn get_colour_bb(&self, colour: Colour) -> u64 {
        self.colour_bb[colour.offset()]
    }

    pub fn move_piece(&mut self, from_sq: Square, to_sq: Square, piece: Piece) {
        self.clear_bitboards(piece, from_sq);
        self.set_bitboards(piece, to_sq);
    }

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        return self.pieces[sq.to_offset()];
    }

    pub fn is_sq_empty(&self, sq: Square) -> bool {
        bitboard::is_set(self.board_bb, sq) == false
    }

    pub fn get_piece_bitboard(&self, piece: Piece) -> u64 {
        return self.piece_bb[piece.offset()];
    }

    pub fn get_bitboard(&self) -> u64 {
        return self.board_bb;
    }

    fn set_bitboards(&mut self, piece: Piece, sq: Square) {
        bitboard::set_bit(&mut self.board_bb, sq);
        bitboard::set_bit(&mut self.piece_bb[piece.offset()], sq);
        bitboard::set_bit(&mut self.colour_bb[piece.colour().offset()], sq);
        self.pieces[sq.to_offset()] = Some(piece);
    }

    fn clear_bitboards(&mut self, piece: Piece, sq: Square) {
        bitboard::clear_bit(&mut self.board_bb, sq);
        bitboard::clear_bit(&mut self.piece_bb[piece.offset()], sq);
        bitboard::clear_bit(&mut self.colour_bb[piece.colour().offset()], sq);
        self.pieces[sq.to_offset()] = None;
    }
}
#[cfg(test)]
pub mod tests {
    use board::bitboard;
    use board::board::Board;
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;
    use std::collections::HashMap;
    use board::square::Square;
    use input::fen::ParsedFen;
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
                let bb = board.get_piece_bitboard(pce);

                assert!(bitboard::is_set(bb, square));

                // clean up
                board.remove_piece(pce, square);
            }
        }
    }

    #[test]
    pub fn test_build_board_from_parsed_fen(){
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::new(PieceRole::Knight, Colour::Black));
        map.insert(Square::h8, Piece::new(PieceRole::King, Colour::White));
        map.insert(Square::d5, Piece::new(PieceRole::Queen, Colour::Black));
        map.insert(Square::c3, Piece::new(PieceRole::Pawn, Colour::White));
        map.insert(Square::a7, Piece::new(PieceRole::Rook, Colour::White));

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd = Board::from_fen(&parsed_fen);


        for sq in utils::get_ordered_square_list_by_file(){            
            if parsed_fen.piece_positions.contains_key(&sq){
                // should contain piece
                let brd_pce = brd.get_piece_on_square(sq).unwrap();
                let map_pce = parsed_fen.piece_positions.get(&sq).unwrap();

                assert_eq!(brd_pce, *map_pce);   
            } else {
                // shouldn't contain a piece
                let retr_pce: Option<Piece> = brd.get_piece_on_square(sq);
                assert_eq!(retr_pce.is_some(), false);
            }
        }

    }

}
