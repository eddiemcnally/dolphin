use crate::bitboard;
use crate::fen::ParsedFen;
use crate::piece;
use crate::piece::Colour;
use crate::piece::Piece;
use crate::square::File;
use crate::square::Rank;
use crate::square::Square;
use std::fmt;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

#[derive(Eq, PartialEq)]
pub struct Board {
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [u64; piece::NUM_PIECE_TYPES],
    // bitboard for each Colour
    colour_bb: [u64; piece::NUM_COLOURS],
    // material value
    material: [u32; piece::NUM_COLOURS],
    // pieces on squares
    pieces: [Option<&'static Piece>; NUM_SQUARES],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            piece_bb: [0; piece::NUM_PIECE_TYPES],
            colour_bb: [0; piece::NUM_COLOURS],
            material: [0; piece::NUM_COLOURS],
            pieces: [None; NUM_SQUARES],
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        debug_str.push_str("\n\n");

        for r in Rank::reverse_iterator() {
            debug_str.push(r.to_char());
            debug_str.push('\t');

            for f in File::iterator() {
                let sq = Square::get_square(*r, *f);

                let pce = &mut None;
                self.get_piece_on_square(sq, pce);
                match pce {
                    Some(pce) => {
                        debug_str.push_str(&pce.label().to_string());
                        debug_str.push_str("\t");
                    }
                    _ => debug_str.push_str(".\t"),
                }
            }

            debug_str.push_str("\n");
        }
        debug_str.push_str("\n\tA\tB\tC\tD\tE\tF\tG\tH\n\n");
        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Board {
    pub fn new() -> Board {
        Board::default()
    }

    pub fn from_fen(parsed_fen: &ParsedFen) -> Board {
        let mut brd = Board::new();

        let positions = parsed_fen.piece_positions.iter();
        for (sq, pce) in positions {
            brd.add_piece(*pce, *sq);
        }
        brd
    }

    pub fn add_piece(&mut self, piece: &'static Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "add_piece, square not empty. {:?}",
            sq
        );

        self.set_bitboards(piece, sq);
        self.material[piece.colour().offset()] += piece.value();
        self.pieces[sq.offset()] = Some(piece);
    }

    pub fn remove_piece(&mut self, piece: &'static Piece, sq: Square) {
        debug_assert!(
            !self.is_sq_empty(sq),
            "remove_piece, square is empty. {:?}",
            sq
        );
        let colour = piece.colour();

        self.clear_bitboards(piece, sq);
        self.material[colour.offset()] -= piece.value();
        self.pieces[sq.offset()] = None;
    }

    pub fn remove_from_sq(&mut self, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "remove_from_sq, square not empty. {:?}",
            sq
        );

        let pce = &mut None;
        self.get_piece_on_square(sq, pce);

        self.remove_piece(pce.unwrap(), sq);
    }

    pub fn get_colour_bb(&self, colour: Colour) -> u64 {
        self.colour_bb[colour.offset()]
    }

    pub fn get_material(&self) -> (u32, u32) {
        (
            self.material[Colour::White.offset()],
            self.material[Colour::Black.offset()],
        )
    }

    pub fn move_piece(&mut self, from_sq: Square, to_sq: Square, piece: &'static Piece) {
        debug_assert!(
            !self.is_sq_empty(from_sq),
            "move piece, from square is empty. {:?}",
            from_sq
        );

        debug_assert!(
            self.is_sq_empty(to_sq),
            "move piece, to square not empty. {:?}",
            from_sq
        );

        self.clear_bitboards(piece, from_sq);
        self.pieces[from_sq.offset()] = None;

        self.set_bitboards(piece, to_sq);
        self.pieces[to_sq.offset()] = Some(piece);
    }

    pub fn get_piece_on_square(&self, sq: Square, pce: &mut Option<&'static Piece>) {
        *pce = self.pieces[sq.offset()];
    }

    pub fn is_sq_empty(&self, sq: Square) -> bool {
        let board_bb = self.get_bitboard();
        bitboard::is_clear(board_bb, sq)
    }

    pub fn get_piece_bitboard(&self, piece: &Piece) -> u64 {
        self.piece_bb[piece.offset()]
    }

    pub fn get_white_rook_queen_bitboard(&self) -> u64 {
        let wr_off = piece::WHITE_ROOK.offset();
        let wq_off = piece::WHITE_QUEEN.offset();
        self.piece_bb[wr_off] | self.piece_bb[wq_off]
    }

    pub fn get_black_rook_queen_bitboard(&self) -> u64 {
        let br_off = piece::BLACK_ROOK.offset();
        let bq_off = piece::BLACK_QUEEN.offset();
        self.piece_bb[br_off] | self.piece_bb[bq_off]
    }

    pub fn get_white_bishop_queen_bitboard(&self) -> u64 {
        let wb_off = piece::WHITE_BISHOP.offset();
        let wq_off = piece::WHITE_QUEEN.offset();

        self.piece_bb[wb_off] | self.piece_bb[wq_off]
    }

    pub fn get_black_bishop_queen_bitboard(&self) -> u64 {
        let bb_off = piece::BLACK_BISHOP.offset();
        let bq_off = piece::BLACK_QUEEN.offset();

        self.piece_bb[bb_off] | self.piece_bb[bq_off]
    }

    pub fn get_bitboard(&self) -> u64 {
        self.get_colour_bb(Colour::White) | self.get_colour_bb(Colour::Black)
    }

    pub fn get_king_sq(&self, colour: Colour) -> Square {
        let king = match colour {
            Colour::White => &piece::WHITE_KING,
            Colour::Black => &piece::BLACK_KING,
        };

        let mut king_bb = self.piece_bb[king.offset()];
        bitboard::pop_1st_bit(&mut king_bb)
    }

    fn clear_bitboards(&mut self, piece: &'static Piece, sq: Square) {
        let pce_off = piece.offset();
        let col_off = piece.colour().offset();

        self.piece_bb[pce_off] = bitboard::clear_bit(self.piece_bb[pce_off], sq);
        self.colour_bb[col_off] = bitboard::clear_bit(self.colour_bb[col_off], sq);
    }

    fn set_bitboards(&mut self, piece: &'static Piece, sq: Square) {
        let pce_off = piece.offset();
        let col_off = piece.colour().offset();

        self.piece_bb[pce_off] = bitboard::set_bit(self.piece_bb[pce_off], sq);
        self.colour_bb[col_off] = bitboard::set_bit(self.colour_bb[col_off], sq);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::bitboard;
    use crate::board::Board;
    use crate::fen::get_position;
    use crate::fen::ParsedFen;
    use crate::piece;
    use crate::square;
    use crate::square::Square;
    use std::collections::HashMap;

    #[test]
    pub fn add_piece_king_square_as_expected() {
        let kings = [&piece::WHITE_KING, &piece::BLACK_KING];

        for pce in kings.iter() {
            let mut board = Board::new();
            for sq in square::SQUARES {
                let king = pce;
                board.add_piece(&king, *sq);

                let colour = king.colour();
                assert_eq!(board.get_king_sq(colour), *sq);

                // remove so state is restored.
                board.remove_piece(&king, *sq);
            }
        }
    }

    #[test]
    pub fn add_remove_piece_square_state_as_expected() {
        let pce = &piece::WHITE_KNIGHT;
        let mut board = Board::new();

        let map = square::SQUARES.iter();
        for square in map {
            assert!(board.is_sq_empty(*square) == true);

            board.add_piece(&pce, *square);
            assert!(board.is_sq_empty(*square) == false);

            board.remove_piece(&pce, *square);
            assert!(board.is_sq_empty(*square) == true);
        }
    }

    #[test]
    pub fn add_remove_white_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = &piece::WHITE_BISHOP;
        let pce2 = &piece::WHITE_QUEEN;

        board.add_piece(&pce1, Square::a1);
        board.add_piece(&pce2, Square::d3);
        let material_after_add = (pce1.value() + pce2.value(), 0);

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(&pce1, Square::a1);

        let material_after_remove = (pce2.value(), 0);

        assert_eq!(material_after_remove, board.get_material());
    }

    #[test]
    pub fn add_remove_black_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = &piece::BLACK_BISHOP;
        let pce2 = &piece::BLACK_QUEEN;

        board.add_piece(&pce1, Square::a1);
        board.add_piece(&pce2, Square::d3);
        let material_after_add = (0, pce1.value() + pce2.value());

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(&pce1, Square::a1);

        let material_after_remove = (0, pce2.value());

        assert_eq!(material_after_remove, board.get_material());
    }

    #[test]
    pub fn move_white_piece_material_unchanged() {
        let pce = &piece::WHITE_KNIGHT;
        let from_sq = Square::d4;
        let to_sq = Square::c6;

        let mut board = Board::new();

        board.add_piece(&pce, from_sq);
        let start_material = board.get_material();

        assert_eq!(start_material.0, pce.value());
        assert_eq!(start_material.1, 0);

        board.move_piece(from_sq, to_sq, &pce);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }

    #[test]
    pub fn move_black_piece_material_unchanged() {
        let pce = &piece::BLACK_KNIGHT;
        let from_sq = Square::d4;
        let to_sq = Square::c6;

        let mut board = Board::new();

        board.add_piece(&pce, Square::d4);
        let start_material = board.get_material();

        assert_eq!(start_material.1, pce.value());

        board.move_piece(from_sq, to_sq, &pce);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }

    #[test]
    pub fn move_piece_square_state_as_expected() {
        let pce = &piece::BLACK_KNIGHT;
        let mut board = Board::new();

        for from_sq in square::SQUARES.iter() {
            for to_sq in square::SQUARES.iter() {
                if *from_sq == *to_sq {
                    continue;
                }

                assert!(board.is_sq_empty(*from_sq) == true);
                assert!(board.is_sq_empty(*to_sq) == true);
                assert!(board.pieces[from_sq.offset()] == None);
                assert!(board.pieces[to_sq.offset()] == None);

                board.add_piece(&pce, *from_sq);
                assert!(board.is_sq_empty(*from_sq) == false);
                assert!(board.is_sq_empty(*to_sq) == true);
                assert!(board.pieces[from_sq.offset()] == Some(&pce));
                assert!(board.pieces[to_sq.offset()] == None);

                board.move_piece(*from_sq, *to_sq, &pce);
                assert!(board.is_sq_empty(*from_sq) == true);
                assert!(board.is_sq_empty(*to_sq) == false);
                assert!(board.pieces[to_sq.offset()] == Some(&pce));
                assert!(board.pieces[from_sq.offset()] == None);

                // clean up
                board.remove_piece(&pce, *to_sq);
            }
        }
    }

    #[test]
    pub fn get_piece_on_square_as_expected() {
        let pce = &piece::WHITE_KNIGHT;
        let mut board = Board::new();

        for square in square::SQUARES {
            assert!(board.is_sq_empty(*square) == true);

            board.add_piece(&pce, *square);
            assert!(board.is_sq_empty(*square) == false);

            let retr_pce = &mut None;
            board.get_piece_on_square(*square, retr_pce);

            assert_eq!(retr_pce.is_some(), true);
            assert_eq!(retr_pce.unwrap(), pce);

            // clean up
            board.remove_piece(&pce, *square);
        }
    }

    #[test]
    pub fn get_bitboard_value_as_expected() {
        let mut board = Board::new();

        for pce in piece::ALL_PIECES {
            for square in square::SQUARES {
                board.add_piece(pce, *square);
                let bb = board.get_piece_bitboard(pce);

                assert!(bitboard::is_set(bb, *square));

                // clean up
                board.remove_piece(pce, *square);
            }
        }
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_a1() {
        let fen = "8/8/8/8/8/8/3N4/k7 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);
        let pce = &mut None;
        brd.get_piece_on_square(Square::a1, pce);

        assert!(*pce.unwrap() == piece::BLACK_KING);
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_a8() {
        let fen = "k7/8/8/8/8/8/3N4/8 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = &mut None;
        brd.get_piece_on_square(Square::a8, pce);

        assert!(*pce.unwrap() == piece::BLACK_KING);
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_h1() {
        let fen = "8/8/8/8/8/8/3N4/7k w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = &mut None;
        brd.get_piece_on_square(Square::h1, pce);

        assert!(*pce.unwrap() == piece::BLACK_KING);
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_h8() {
        let fen = "7k/8/8/8/8/8/3N4/8 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = &mut None;
        brd.get_piece_on_square(Square::h8, pce);

        assert!(*pce.unwrap() == piece::BLACK_KING);
    }

    #[test]
    pub fn build_board_from_parsed_fen() {
        let mut map = HashMap::new();

        map.insert(Square::a1, &piece::BLACK_KNIGHT);
        map.insert(Square::h8, &piece::WHITE_KING);
        map.insert(Square::d5, &piece::BLACK_QUEEN);
        map.insert(Square::c3, &piece::WHITE_PAWN);
        map.insert(Square::a7, &piece::WHITE_ROOK);

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd = Board::from_fen(&parsed_fen);

        for sq in square::SQUARES {
            if parsed_fen.piece_positions.contains_key(sq) {
                // should contain piece
                let brd_pce = &mut None;
                brd.get_piece_on_square(*sq, brd_pce);

                let map_pce = parsed_fen.piece_positions.get(sq).unwrap();

                assert_eq!(brd_pce.unwrap(), *map_pce);
            } else {
                // shouldn't contain a piece
                let retr_pce = &mut None;
                brd.get_piece_on_square(*sq, retr_pce);

                assert_eq!(retr_pce.is_some(), false);
            }
        }
    }

    #[test]
    pub fn board_equality_as_expected() {
        let mut map = HashMap::new();

        map.insert(Square::a1, &piece::BLACK_KNIGHT);
        map.insert(Square::h8, &piece::WHITE_KING);
        map.insert(Square::d5, &piece::BLACK_QUEEN);
        map.insert(Square::c3, &piece::WHITE_PAWN);
        map.insert(Square::a7, &piece::WHITE_ROOK);

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd_1 = Board::from_fen(&parsed_fen);
        let brd_2 = Board::from_fen(&parsed_fen);

        assert_eq!(brd_1, brd_2);
    }
}
