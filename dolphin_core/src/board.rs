use crate::bitboard;
use crate::fen::ParsedFen;
use crate::piece;
use crate::piece::Colour;
use crate::piece::Piece;
use crate::square;
use crate::square::File;
use crate::square::Rank;
use crate::square::Square;
use std::fmt;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

#[derive(Copy, Clone)]
pub struct Board {
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [u64; piece::NUM_PIECES],
    // bitboard for each Colour
    colour_bb: [u64; piece::NUM_COLOURS],
    // material value
    material: [u32; piece::NUM_COLOURS],
    // pieces on sqaures
    pieces: [Option<Piece>; square::NUM_SQUARES],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            piece_bb: [0; piece::NUM_PIECES],
            colour_bb: [0; piece::NUM_COLOURS],
            material: [0; piece::NUM_COLOURS],
            pieces: [None; square::NUM_SQUARES],
        }
    }
}
// set up some frequently used array offsets
const WQ_ARR_OFFSET: usize = Piece::WhiteQueen.to_offset();
const WK_ARR_OFFSET: usize = Piece::WhiteKing.to_offset();
const WR_ARR_OFFSET: usize = Piece::WhiteRook.to_offset();
const BQ_ARR_OFFSET: usize = Piece::BlackQueen.to_offset();
const BK_ARR_OFFSET: usize = Piece::BlackKing.to_offset();
const BR_ARR_OFFSET: usize = Piece::BlackRook.to_offset();
const WB_ARR_OFFSET: usize = Piece::WhiteBishop.to_offset();
const BB_ARR_OFFSET: usize = Piece::BlackBishop.to_offset();
const WHITE_ARR_OFFSET: usize = Colour::White.to_offset();
const BLACK_ARR_OFFSET: usize = Colour::Black.to_offset();

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..piece::NUM_PIECES {
            if self.piece_bb[i] != other.piece_bb[i] {
                println!("BOARD: piece_bb are different");
                return false;
            }
        }

        for i in 0..piece::NUM_PIECES {
            if self.pieces[i] != other.pieces[i] {
                println!("BOARD: pieces are different");
                return false;
            }
        }
        for i in 0..piece::NUM_COLOURS as usize {
            if self.colour_bb[i] != other.colour_bb[i] {
                println!("BOARD: colour_bb are different");
                return false;
            }
            if self.material[i] != other.material[i] {
                println!("BOARD: material values are different");
                return false;
            }
        }

        true
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

                let pce = self.get_piece_on_square(sq);
                match pce {
                    Some(pce) => {
                        debug_str.push_str(&pce.to_label().to_string());
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

    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "add_piece, square not empty. {:?}",
            sq
        );
        self.set_bitboards_with_material(piece, sq);
        self.pieces[sq.to_offset()] = Some(piece);
    }

    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(
            !self.is_sq_empty(sq),
            "remove_piece, square is empty. {:?}",
            sq
        );

        self.clear_bitboards_with_material(piece, sq);
        self.pieces[sq.to_offset()] = None;
    }

    pub fn remove_from_sq(&mut self, sq: Square) {
        let piece = self.get_piece_on_square(sq);
        if piece.is_none() {
            panic!("attempt to remove from square but square is empty");
        }

        self.remove_piece(piece.unwrap(), sq);
    }

    pub fn get_colour_bb(&self, colour: Colour) -> u64 {
        self.colour_bb[colour.to_offset()]
    }

    pub fn get_material(&self) -> (u32, u32) {
        (
            self.material[WHITE_ARR_OFFSET],
            self.material[BLACK_ARR_OFFSET],
        )
    }

    pub fn move_piece(&mut self, from_sq: Square, to_sq: Square, piece: Piece) {
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

        debug_assert!(
            self.get_piece_on_square(from_sq) == Some(piece),
            "move piece, piece on from_sq not as expected. Expected: {:?}, found: {:?}, from_sq: {:?}, to_sq: {:?}",
            piece,
            self.get_piece_on_square(from_sq),
            from_sq,
            to_sq
        );

        self.clear_bitboards(piece, from_sq);
        self.pieces[from_sq.to_offset()] = None;

        self.set_bitboards(piece, to_sq);
        self.pieces[to_sq.to_offset()] = Some(piece);
    }

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        self.pieces[sq.to_offset()]
    }

    pub fn is_sq_empty(&self, sq: Square) -> bool {
        self.pieces[sq.to_offset()] == None
    }

    pub const fn get_piece_bitboard(&self, piece: Piece) -> u64 {
        self.piece_bb[piece.to_offset()]
    }

    pub const fn get_white_rook_queen_bitboard(&self) -> u64 {
        self.piece_bb[WR_ARR_OFFSET] | self.piece_bb[WQ_ARR_OFFSET]
    }

    pub const fn get_black_rook_queen_bitboard(&self) -> u64 {
        self.piece_bb[BR_ARR_OFFSET] | self.piece_bb[BQ_ARR_OFFSET]
    }

    pub const fn get_white_bishop_queen_bitboard(&self) -> u64 {
        self.piece_bb[WB_ARR_OFFSET] | self.piece_bb[WQ_ARR_OFFSET]
    }

    pub const fn get_black_bishop_queen_bitboard(&self) -> u64 {
        self.piece_bb[BB_ARR_OFFSET] | self.piece_bb[BQ_ARR_OFFSET]
    }

    pub fn get_bitboard(&self) -> u64 {
        self.get_colour_bb(Colour::White) | self.get_colour_bb(Colour::Black)
    }

    pub fn get_king_sq(&self, colour: Colour) -> Square {
        let mut king_bb = match colour {
            Colour::White => self.piece_bb[WK_ARR_OFFSET],
            Colour::Black => self.piece_bb[BK_ARR_OFFSET],
        };
        bitboard::pop_1st_bit(&mut king_bb)
    }

    fn set_bitboards_with_material(&mut self, piece: Piece, sq: Square) {
        self.set_bitboards(piece, sq);
        self.material[piece.colour().to_offset()] += piece.value();
    }

    fn clear_bitboards_with_material(&mut self, piece: Piece, sq: Square) {
        self.clear_bitboards(piece, sq);
        self.material[piece.colour().to_offset()] -= piece.value();
    }

    fn clear_bitboards(&mut self, piece: Piece, sq: Square) {
        self.piece_bb[piece.to_offset()] =
            bitboard::clear_bit(self.piece_bb[piece.to_offset()], sq);
        self.colour_bb[piece.colour().to_offset()] =
            bitboard::clear_bit(self.colour_bb[piece.colour().to_offset()], sq);
    }

    fn set_bitboards(&mut self, pce: Piece, sq: Square) {
        self.piece_bb[pce.to_offset()] = bitboard::set_bit(self.piece_bb[pce.to_offset()], sq);
        self.colour_bb[pce.colour().to_offset()] =
            bitboard::set_bit(self.colour_bb[pce.colour().to_offset()], sq);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::bitboard;
    use crate::board::Board;
    use crate::fen::get_position;
    use crate::fen::ParsedFen;
    use crate::piece;
    use crate::piece::Piece;
    use crate::square;
    use crate::square::Square;
    use std::collections::HashMap;

    #[test]
    pub fn add_piece_king_square_as_expected() {
        let kings = [Piece::WhiteKing, Piece::BlackKing];

        for pce in kings.iter() {
            let mut board = Board::new();
            for sq in square::SQUARES {
                let king = *pce;
                board.add_piece(king, *sq);

                let colour = king.colour();
                assert_eq!(board.get_king_sq(colour), *sq);

                // remove so state is restored.
                board.remove_piece(king, *sq);
            }
        }
    }

    #[test]
    pub fn add_remove_piece_square_state_as_expected() {
        let pce = Piece::WhiteKnight;
        let mut board = Board::new();

        let map = square::SQUARES.iter();
        for square in map {
            assert!(board.is_sq_empty(*square) == true);

            board.add_piece(pce, *square);
            assert!(board.is_sq_empty(*square) == false);

            board.remove_piece(pce, *square);
            assert!(board.is_sq_empty(*square) == true);
        }
    }

    #[test]
    pub fn add_remove_white_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = Piece::WhiteBishop;
        let pce2 = Piece::WhiteQueen;

        board.add_piece(pce1, Square::a1);
        board.add_piece(pce2, Square::d3);
        let material_after_add = (pce1.value() + pce2.value(), 0);

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(pce1, Square::a1);

        let material_after_remove = (pce2.value(), 0);

        assert_eq!(material_after_remove, board.get_material());
    }

    #[test]
    pub fn add_remove_black_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = Piece::BlackBishop;
        let pce2 = Piece::BlackQueen;

        board.add_piece(pce1, Square::a1);
        board.add_piece(pce2, Square::d3);
        let material_after_add = (0, pce1.value() + pce2.value());

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(pce1, Square::a1);

        let material_after_remove = (0, pce2.value());

        assert_eq!(material_after_remove, board.get_material());
    }

    #[test]
    pub fn move_white_piece_material_unchanged() {
        let pce = Piece::WhiteKnight;
        let from_sq = Square::d4;
        let to_sq = Square::c6;

        let mut board = Board::new();

        board.add_piece(pce, from_sq);
        let start_material = board.get_material();

        assert_eq!(start_material.0, pce.value());
        assert_eq!(start_material.1, 0);

        board.move_piece(from_sq, to_sq, pce);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }

    #[test]
    pub fn move_black_piece_material_unchanged() {
        let pce = Piece::BlackKnight;
        let from_sq = Square::d4;
        let to_sq = Square::c6;

        let mut board = Board::new();

        board.add_piece(pce, Square::d4);
        let start_material = board.get_material();

        assert_eq!(start_material.1, pce.value());

        board.move_piece(from_sq, to_sq, pce);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }

    #[test]
    pub fn move_piece_square_state_as_expected() {
        let pce = Piece::WhiteKnight;
        let mut board = Board::new();

        for from_sq in square::SQUARES.iter() {
            for to_sq in square::SQUARES.iter() {
                if *from_sq == *to_sq {
                    continue;
                }

                assert!(board.is_sq_empty(*from_sq) == true);
                assert!(board.is_sq_empty(*to_sq) == true);
                assert!(board.pieces[from_sq.to_offset()] == None);
                assert!(board.pieces[to_sq.to_offset()] == None);

                board.add_piece(pce, *from_sq);
                assert!(board.is_sq_empty(*from_sq) == false);
                assert!(board.is_sq_empty(*to_sq) == true);
                assert!(board.pieces[from_sq.to_offset()] == Some(pce));
                assert!(board.pieces[to_sq.to_offset()] == None);

                board.move_piece(*from_sq, *to_sq, pce);
                assert!(board.is_sq_empty(*from_sq) == true);
                assert!(board.is_sq_empty(*to_sq) == false);
                assert!(board.pieces[to_sq.to_offset()] == Some(pce));
                assert!(board.pieces[from_sq.to_offset()] == None);

                // clean up
                board.remove_piece(pce, *to_sq);
            }
        }
    }

    #[test]
    pub fn get_piece_on_square_as_expected() {
        let pce = Piece::WhiteKnight;
        let mut board = Board::new();

        for square in square::SQUARES {
            assert!(board.is_sq_empty(*square) == true);

            board.add_piece(pce, *square);
            assert!(board.is_sq_empty(*square) == false);

            let retr_pce: Option<Piece> = board.get_piece_on_square(*square);
            assert_eq!(retr_pce.is_some(), true);
            assert_eq!(retr_pce.unwrap(), pce);

            // clean up
            board.remove_piece(pce, *square);
        }
    }

    #[test]
    pub fn get_bitboard_value_as_expected() {
        let mut board = Board::new();

        for pce in piece::get_all_pieces() {
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

        let pce = brd.get_piece_on_square(Square::a1).unwrap();
        assert!(pce == Piece::BlackKing);
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_a8() {
        let fen = "k7/8/8/8/8/8/3N4/8 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::a8).unwrap();
        assert!(pce == Piece::BlackKing);
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_h1() {
        let fen = "8/8/8/8/8/8/3N4/7k w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::h1).unwrap();
        assert!(pce == Piece::BlackKing);
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_h8() {
        let fen = "7k/8/8/8/8/8/3N4/8 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::h8).unwrap();
        assert!(pce == Piece::BlackKing);
    }

    #[test]
    pub fn build_board_from_parsed_fen() {
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::BlackKnight);
        map.insert(Square::h8, Piece::WhiteKing);
        map.insert(Square::d5, Piece::BlackQueen);
        map.insert(Square::c3, Piece::WhitePawn);
        map.insert(Square::a7, Piece::WhiteRook);

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd = Board::from_fen(&parsed_fen);

        for sq in square::SQUARES {
            if parsed_fen.piece_positions.contains_key(sq) {
                // should contain piece
                let brd_pce = brd.get_piece_on_square(*sq).unwrap();
                let map_pce = parsed_fen.piece_positions.get(sq).unwrap();

                assert_eq!(brd_pce, *map_pce);
            } else {
                // shouldn't contain a piece
                let retr_pce: Option<Piece> = brd.get_piece_on_square(*sq);
                assert_eq!(retr_pce.is_some(), false);
            }
        }
    }

    #[test]
    pub fn clone_board_as_expected() {
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::BlackKnight);
        map.insert(Square::h8, Piece::WhiteKing);
        map.insert(Square::d5, Piece::BlackQueen);
        map.insert(Square::c3, Piece::WhitePawn);
        map.insert(Square::a7, Piece::WhiteRook);

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd = Board::from_fen(&parsed_fen);

        let cloned = brd.clone();

        for sq in square::SQUARES {
            let pce = cloned.get_piece_on_square(*sq);

            if parsed_fen.piece_positions.contains_key(sq) {
                assert!(pce.unwrap() == *parsed_fen.piece_positions.get(sq).unwrap());
            } else {
                assert!(pce.is_none());
            }
        }

        assert_eq!(brd, cloned);
    }

    #[test]
    pub fn board_equality_as_expected() {
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::BlackKnight);
        map.insert(Square::h8, Piece::WhiteKing);
        map.insert(Square::d5, Piece::BlackQueen);
        map.insert(Square::c3, Piece::WhitePawn);
        map.insert(Square::a7, Piece::WhiteRook);

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd_1 = Board::from_fen(&parsed_fen);
        let brd_2 = Board::from_fen(&parsed_fen);

        assert_eq!(brd_1, brd_2);
    }
}
