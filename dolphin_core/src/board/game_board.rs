use crate::board::bitboard;
use crate::board::colour;
use crate::board::colour::Colour;
use crate::board::file::File;
use crate::board::material::Material;
use crate::board::piece;
use crate::board::piece::Piece;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::board::types::ToInt;
use std::fmt;
use std::option::Option;

pub const NUM_SQUARES: usize = 64;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Board {
    // piece bitboard, an entry for each piece type (enum Piece)
    piece_bb: [u64; piece::NUM_PIECE_TYPES],
    // bitboard for each Colour
    colour_bb: [u64; colour::NUM_COLOURS],
    // material value
    material: Material,
    // pieces on squares
    pieces: [Option<&'static Piece>; NUM_SQUARES],
    // king squares
    king_squares: [Square; colour::NUM_COLOURS],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            piece_bb: [0; piece::NUM_PIECE_TYPES],
            colour_bb: [0; colour::NUM_COLOURS],
            material: Material::default(),
            pieces: [None; NUM_SQUARES],
            king_squares: [Square::default(); colour::NUM_COLOURS],
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
                let sq = Square::from_rank_file(*r, *f);

                let pce = &mut None;
                self.get_piece_on_square(sq, pce);
                match pce {
                    Some(pce) => {
                        debug_str.push_str(&pce.label().to_string());
                        debug_str.push('\t');
                    }
                    _ => debug_str.push_str(".\t"),
                }
            }

            debug_str.push('\n');
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

    pub fn add_piece(&mut self, piece: &'static Piece, sq: Square) {
        debug_assert!(
            self.is_sq_empty(sq),
            "add_piece, square not empty. {:?}",
            sq
        );

        self.set_bitboards(piece, sq);
        let new_material = self.material.get_material_for_colour(piece.colour()) + piece.value();
        self.material
            .set_material_for_colour(piece.colour(), new_material);
        self.pieces[sq.to_usize()] = Some(piece);
        if piece.is_king() {
            self.king_squares[piece.colour().to_usize()] = sq;
        }
    }

    pub fn remove_piece(&mut self, piece: &'static Piece, sq: Square) {
        debug_assert!(
            !self.is_sq_empty(sq),
            "remove_piece, square is empty. {:?}",
            sq
        );

        self.clear_bitboards(piece, sq);
        let new_material = self.material.get_material_for_colour(piece.colour()) - piece.value();
        self.material
            .set_material_for_colour(piece.colour(), new_material);
        self.pieces[sq.to_usize()] = None;
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
        self.colour_bb[colour.to_usize()]
    }

    pub fn get_material(&self) -> Material {
        self.material
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
        self.pieces[from_sq.to_usize()] = None;

        self.set_bitboards(piece, to_sq);
        self.pieces[to_sq.to_usize()] = Some(piece);
        if piece.is_king() {
            self.king_squares[piece.colour().to_usize()] = to_sq;
        }
    }

    pub fn get_piece_on_square(&self, sq: Square, pce: &mut Option<&'static Piece>) {
        *pce = self.pieces[sq.to_usize()];
    }

    pub fn is_sq_empty(&self, sq: Square) -> bool {
        let board_bb = self.get_bitboard();
        bitboard::is_clear(board_bb, sq)
    }

    pub fn get_piece_bitboard(&self, piece: &Piece) -> u64 {
        self.piece_bb[piece.to_usize()]
    }

    pub fn get_white_rook_queen_bitboard(&self) -> u64 {
        let wr_off = piece::WHITE_ROOK.to_usize();
        let wq_off = piece::WHITE_QUEEN.to_usize();
        self.piece_bb[wr_off] | self.piece_bb[wq_off]
    }

    pub fn get_black_rook_queen_bitboard(&self) -> u64 {
        let br_off = piece::BLACK_ROOK.to_usize();
        let bq_off = piece::BLACK_QUEEN.to_usize();

        self.piece_bb[br_off] | self.piece_bb[bq_off]
    }

    pub fn get_white_bishop_queen_bitboard(&self) -> u64 {
        let wb_off = piece::WHITE_BISHOP.to_usize();
        let wq_off = piece::WHITE_QUEEN.to_usize();

        self.piece_bb[wb_off] | self.piece_bb[wq_off]
    }

    pub fn get_black_bishop_queen_bitboard(&self) -> u64 {
        let bb_off = piece::BLACK_BISHOP.to_usize();
        let bq_off = piece::BLACK_QUEEN.to_usize();

        self.piece_bb[bb_off] | self.piece_bb[bq_off]
    }

    pub fn get_bitboard(&self) -> u64 {
        self.get_colour_bb(Colour::White) | self.get_colour_bb(Colour::Black)
    }

    pub fn get_king_sq(&self, colour: Colour) -> Square {
        self.king_squares[colour.to_usize()]
    }

    fn clear_bitboards(&mut self, piece: &'static Piece, sq: Square) {
        let pce_off = piece.to_usize();
        let col_off = piece.colour().to_usize();

        self.piece_bb[pce_off] = bitboard::clear_bit(self.piece_bb[pce_off], sq);
        self.colour_bb[col_off] = bitboard::clear_bit(self.colour_bb[col_off], sq);
    }

    fn set_bitboards(&mut self, piece: &'static Piece, sq: Square) {
        let pce_off = piece.to_usize();
        let col_off = piece.colour().to_usize();

        self.piece_bb[pce_off] = bitboard::set_bit(self.piece_bb[pce_off], sq);
        self.colour_bb[col_off] = bitboard::set_bit(self.colour_bb[col_off], sq);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::bitboard;
    use crate::board::game_board::Board;
    use crate::board::piece;
    use crate::board::square;
    use crate::board::types::ToInt;
    use crate::io::fen;

    #[test]
    pub fn add_piece_king_square_as_expected() {
        let kings = [&piece::WHITE_KING, &piece::BLACK_KING];

        for pce in kings.iter() {
            let mut board = Board::new();
            for sq in square::SQUARES {
                let king = pce;
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
        let pce = &piece::WHITE_KNIGHT;
        let mut board = Board::new();

        let map = square::SQUARES.iter();
        for square in map {
            assert!(board.is_sq_empty(*square));

            board.add_piece(pce, *square);
            assert!(!board.is_sq_empty(*square));

            board.remove_piece(pce, *square);
            assert!(board.is_sq_empty(*square));
        }
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

                assert!(board.is_sq_empty(*from_sq));
                assert!(board.is_sq_empty(*to_sq));
                assert!(board.pieces[from_sq.to_usize()] == None);
                assert!(board.pieces[to_sq.to_usize()] == None);

                board.add_piece(pce, *from_sq);
                assert!(!board.is_sq_empty(*from_sq));
                assert!(board.is_sq_empty(*to_sq));
                assert!(board.pieces[from_sq.to_usize()] == Some(pce));
                assert!(board.pieces[to_sq.to_usize()] == None);

                board.move_piece(*from_sq, *to_sq, pce);
                assert!(board.is_sq_empty(*from_sq));
                assert!(!board.is_sq_empty(*to_sq));
                assert!(board.pieces[to_sq.to_usize()] == Some(pce));
                assert!(board.pieces[from_sq.to_usize()] == None);

                // clean up
                board.remove_piece(pce, *to_sq);
            }
        }
    }

    #[test]
    pub fn get_piece_on_square_as_expected() {
        let pce = &piece::WHITE_KNIGHT;
        let mut board = Board::new();

        for square in square::SQUARES {
            assert!(board.is_sq_empty(*square));

            board.add_piece(pce, *square);
            assert!(!board.is_sq_empty(*square));

            let retr_pce = &mut None;
            board.get_piece_on_square(*square, retr_pce);

            assert!(retr_pce.is_some());
            assert_eq!(retr_pce.unwrap(), pce);

            // clean up
            board.remove_piece(pce, *square);
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
    pub fn board_equality_as_expected() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";

        let (board_1, _, _, _, _) = fen::decompose_fen(fen);
        let (board_2, _, _, _, _) = fen::decompose_fen(fen);

        assert_eq!(board_1, board_2);
    }
}
