use board::bitboard;
use board::piece::Colour;
use board::piece::Piece;
use board::piece::NUM_COLOURS;
use board::piece::NUM_PIECES;
use board::square;
use board::square::File;
use board::square::Rank;
use board::square::Square;
use core::core_traits::ArrayAccessor;
use input::fen::ParsedFen;
use std::fmt;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

#[derive(Copy)]
pub struct Board {
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [u64; NUM_PIECES],
    // bitboard for each Colour
    colour_bb: [u64; NUM_COLOURS],
    // the pieces on each square
    pieces: [Option<Piece>; NUM_SQUARES],
    // material value
    material: [u32; NUM_COLOURS],
}

impl Default for Board {
    fn default() -> Self {
        let brd = Board {
            piece_bb: [0; NUM_PIECES],
            colour_bb: [0; NUM_COLOURS],
            pieces: [None; NUM_SQUARES],
            material: [0; NUM_COLOURS],
        };
        return brd;
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..NUM_PIECES - 1 {
            if self.piece_bb[i] != other.piece_bb[i] {
                println!("BOARD: piece_bb are different");
                return false;
            }
        }

        for i in 0..NUM_COLOURS - 1 {
            if self.colour_bb[i] != other.colour_bb[i] {
                println!("BOARD: colour_bb are different");
                return false;
            }
            if self.material[i] != other.material[i] {
                println!("BOARD: material values are different");
                return false;
            }
        }

        for i in 0..NUM_SQUARES - 1 {
            if self.pieces[i] != other.pieces[i] {
                println!("BOARD: pieces array are different");
                return false;
            }
        }

        return true;
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        debug_str.push_str("\n");

        for r in Rank::reverse_iterator() {
            debug_str.push(square::Rank::to_char(*r));
            debug_str.push(' ');

            for f in File::iterator() {
                let sq = square::Square::get_square(*r, *f);

                let pce = self.get_piece_on_square(sq);
                match pce {
                    Some(pce) => {
                        debug_str.push_str(&pce.to_label());
                        debug_str.push_str(" ");
                    }
                    _ => debug_str.push_str(" . "),
                }
            }

            debug_str.push_str("\n");
        }
        debug_str.push_str("   A  B  C  D  E  F  G  H");
        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut cp_pieces: [Option<Piece>; NUM_SQUARES] = [None; NUM_SQUARES];
        for sq in square::get_square_array().iter() {
            cp_pieces[sq.to_offset()] = self.pieces[sq.to_offset()];
        }

        let brd = Board {
            piece_bb: self.piece_bb,
            colour_bb: self.colour_bb,
            pieces: cp_pieces,
            material: self.material,
        };
        return brd;
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
        return brd;
    }

    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "add_piece, square not empty. {:?}",
            sq
        );
        self.set_bitboards_with_material(piece, sq);
    }

    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq) == false,
            "remove_piece, square is empty. {:?}",
            sq
        );

        self.clear_bitboards_with_material(piece, sq);
    }

    pub fn get_colour_bb(&self, colour: Colour) -> u64 {
        return self.colour_bb[colour.to_offset()];
    }

    pub fn get_material(&self) -> (u32, u32) {
        (
            self.material[Colour::White.to_offset()],
            self.material[Colour::Black.to_offset()],
        )
    }

    pub fn move_piece(&mut self, from_sq: Square, to_sq: Square, piece: Piece) {
        debug_assert!(
            self.is_sq_empty(from_sq) == false,
            "move piece, from square is empty. {:?}",
            from_sq
        );

        debug_assert!(
            self.is_sq_empty(to_sq) == true,
            "move piece, to square not empty. {:?}",
            from_sq
        );

        self.clear_bitboards(piece, from_sq);
        self.set_bitboards(piece, to_sq);
    }

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        return self.pieces[sq.to_offset()];
    }

    pub fn is_sq_empty(&self, sq: Square) -> bool {
        let bb = self.get_bitboard();
        bitboard::is_set(bb, sq) == false
    }

    pub fn get_piece_bitboard(&self, piece: Piece) -> u64 {
        return self.piece_bb[piece.to_offset()];
    }

    pub fn get_bitboard(&self) -> u64 {
        return self.get_colour_bb(Colour::White) | self.get_colour_bb(Colour::Black);
    }

    pub fn get_king_sq(&self, colour: Colour) -> Square {
        let mut king_bb = match colour {
            Colour::White => self.piece_bb[Piece::WhiteKing.to_offset()],
            Colour::Black => self.piece_bb[Piece::BlackKing.to_offset()],
        };
        let sq = bitboard::pop_1st_bit(&mut king_bb);
        return sq;
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
        bitboard::clear_bit(&mut self.piece_bb[piece.to_offset()], sq);
        bitboard::clear_bit(&mut self.colour_bb[piece.colour().to_offset()], sq);
        self.pieces[sq.to_offset()] = None;
    }

    fn set_bitboards(&mut self, pce: Piece, sq: Square) {
        bitboard::set_bit(&mut self.piece_bb[pce.to_offset()], sq);
        bitboard::set_bit(&mut self.colour_bb[pce.colour().to_offset()], sq);
        self.pieces[sq.to_offset()] = Some(pce);
    }
}

#[cfg(test)]
pub mod tests {
    use board::bitboard;
    use board::board::Board;
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;
    use board::square::Square;
    use input::fen::get_position;
    use input::fen::ParsedFen;
    use std::collections::HashMap;
    use utils;

    #[test]
    pub fn add_piece_king_square_as_expected() {
        let cols = vec![Colour::White, Colour::Black];

        for col in cols {
            let mut board = Board::new();
            let king = Piece::new(PieceRole::King, col);
            for sq in utils::get_ordered_square_list_by_file() {
                board.add_piece(king, sq);

                assert_eq!(board.get_king_sq(col), sq);

                // remove so state is restored.
                board.remove_piece(king, sq);
            }
        }
    }

    #[test]
    pub fn add_remove_piece_square_state_as_expected() {
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
    pub fn add_remove_white_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = Piece::new(PieceRole::Bishop, Colour::White);
        let pce2 = Piece::new(PieceRole::Queen, Colour::White);

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

        let pce1 = Piece::new(PieceRole::Bishop, Colour::Black);
        let pce2 = Piece::new(PieceRole::Queen, Colour::Black);

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
        let pce = Piece::new(PieceRole::Knight, Colour::White);
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
        let pce = Piece::new(PieceRole::Knight, Colour::Black);
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
    pub fn get_piece_on_square_as_expected() {
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
    pub fn get_bitboard_value_as_expected() {
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
    pub fn parsed_fen_edge_squares_as_expected_a1() {
        let fen = "8/8/8/8/8/8/3N4/k7 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::a1).unwrap();
        assert!(pce == Piece::new(PieceRole::King, Colour::Black))
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_a8() {
        let fen = "k7/8/8/8/8/8/3N4/8 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::a8).unwrap();
        assert!(pce == Piece::new(PieceRole::King, Colour::Black))
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_h1() {
        let fen = "8/8/8/8/8/8/3N4/7k w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::h1).unwrap();
        assert!(pce == Piece::new(PieceRole::King, Colour::Black))
    }

    #[test]
    pub fn parsed_fen_edge_squares_as_expected_h8() {
        let fen = "7k/8/8/8/8/8/3N4/8 w - - 0 1";
        let parsed_fen = get_position(fen);

        let brd = Board::from_fen(&parsed_fen);

        let pce = brd.get_piece_on_square(Square::h8).unwrap();
        assert!(pce == Piece::new(PieceRole::King, Colour::Black))
    }

    #[test]
    pub fn build_board_from_parsed_fen() {
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::new(PieceRole::Knight, Colour::Black));
        map.insert(Square::h8, Piece::new(PieceRole::King, Colour::White));
        map.insert(Square::d5, Piece::new(PieceRole::Queen, Colour::Black));
        map.insert(Square::c3, Piece::new(PieceRole::Pawn, Colour::White));
        map.insert(Square::a7, Piece::new(PieceRole::Rook, Colour::White));

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd = Board::from_fen(&parsed_fen);

        for sq in utils::get_ordered_square_list_by_file() {
            if parsed_fen.piece_positions.contains_key(&sq) {
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

    #[test]
    pub fn clone_board_as_expected() {
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::new(PieceRole::Knight, Colour::Black));
        map.insert(Square::h8, Piece::new(PieceRole::King, Colour::White));
        map.insert(Square::d5, Piece::new(PieceRole::Queen, Colour::Black));
        map.insert(Square::c3, Piece::new(PieceRole::Pawn, Colour::White));
        map.insert(Square::a7, Piece::new(PieceRole::Rook, Colour::White));

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd = Board::from_fen(&parsed_fen);

        let cloned = brd.clone();

        for sq in utils::get_ordered_square_list_by_file() {
            let pce = cloned.get_piece_on_square(sq);

            if parsed_fen.piece_positions.contains_key(&sq) {
                assert!(pce.unwrap() == *parsed_fen.piece_positions.get(&sq).unwrap());
            } else {
                assert!(pce.is_none());
            }
        }

        assert_eq!(brd, cloned);
    }

    #[test]
    pub fn board_equality_as_expected() {
        let mut map = HashMap::new();

        map.insert(Square::a1, Piece::new(PieceRole::Knight, Colour::Black));
        map.insert(Square::h8, Piece::new(PieceRole::King, Colour::White));
        map.insert(Square::d5, Piece::new(PieceRole::Queen, Colour::Black));
        map.insert(Square::c3, Piece::new(PieceRole::Pawn, Colour::White));
        map.insert(Square::a7, Piece::new(PieceRole::Rook, Colour::White));

        let mut parsed_fen: ParsedFen = Default::default();
        parsed_fen.piece_positions = map;

        let brd_1 = Board::from_fen(&parsed_fen);
        let brd_2 = Board::from_fen(&parsed_fen);

        assert_eq!(brd_1, brd_2);
    }
}
