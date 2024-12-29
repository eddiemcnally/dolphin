use crate::board::bitboard::Bitboard;
use crate::board::colour::Colour;
use crate::board::file::File;
use crate::board::piece::Piece;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::moves::mov::Score;
use std::fmt;
use std::option::Option;

#[derive(Eq, PartialEq, Default, Copy, Clone)]
struct ColourInfo {
    piece_bb: [Bitboard; Piece::NUM_PIECE_TYPES],
    colour_bb: Bitboard,
    material: Score,
    king_sq: Square,
}

#[derive(Eq, PartialEq, Default, Copy, Clone)]
pub struct Material {
    white: Score,
    black: Score,
}

#[derive(Eq, PartialEq)]
pub struct Board {
    colour_info: [ColourInfo; Colour::NUM_COLOURS],
    pieces: [Option<Piece>; Board::NUM_SQUARES],
}

impl Board {
    pub const NUM_SQUARES: usize = 64;

    pub fn new() -> Board {
        Board::default()
    }

    pub fn add_piece(&mut self, piece: &Piece, colour: &Colour, sq: &Square) {
        self.flip_piece_bits(piece, colour, sq);

        self.colour_info[colour.as_index()].material += piece.value();
        self.pieces[sq.as_index()] = Some(*piece);
        match piece {
            Piece::King => self.colour_info[colour.as_index()].king_sq = *sq,
            _ => (),
        }
    }

    pub fn remove_piece(&mut self, piece: &Piece, colour: &Colour, sq: &Square) {
        self.flip_piece_bits(piece, colour, sq);

        self.colour_info[colour.as_index()].material -= piece.value();
        self.pieces[sq.as_index()] = None;
    }

    pub fn move_piece(&mut self, from_sq: &Square, to_sq: &Square, piece: &Piece, colour: &Colour) {
        self.flip_piece_bits(piece, colour, from_sq);
        self.flip_piece_bits(piece, colour, to_sq);

        self.pieces[from_sq.as_index()] = None;
        self.pieces[to_sq.as_index()] = Some(*piece);

        match piece {
            Piece::King => self.colour_info[colour.as_index()].king_sq = *to_sq,
            _ => (),
        }
    }

    #[inline(always)]
    fn flip_piece_bits(&mut self, piece: &Piece, colour: &Colour, sq: &Square) {
        let bb = Bitboard::from_square(sq);

        (&mut self.colour_info[colour.as_index()]).piece_bb[piece.as_index()] ^= bb;
        (&mut self.colour_info[colour.as_index()]).colour_bb ^= bb;
    }

    pub fn get_piece_and_colour_on_square(&self, sq: &Square) -> Option<(Piece, Colour)> {
        if let Some(pce) = self.get_piece_on_square(sq) {
            let colour = if self.colour_info[Colour::White.as_index()]
                .colour_bb
                .is_set(sq)
            {
                Colour::White
            } else {
                Colour::Black
            };

            return Some((pce, colour));
        }

        None
    }

    pub fn get_piece_on_square(&self, sq: &Square) -> Option<Piece> {
        self.pieces[sq.as_index()]
    }

    pub fn is_sq_empty(&self, sq: &Square) -> bool {
        self.get_bitboard().is_set(sq) == false
    }

    pub const fn get_piece_bitboard(&self, piece: &Piece, colour: &Colour) -> Bitboard {
        self.colour_info[colour.as_index()].piece_bb[piece.as_index()]
    }

    pub fn get_rook_and_queen_bb_for_colour(&self, colour: &Colour) -> Bitboard {
        self.colour_info[colour.as_index()].piece_bb[Piece::Rook.as_index()]
            | self.colour_info[colour.as_index()].piece_bb[Piece::Queen.as_index()]
    }
    pub fn get_bishop_and_queen_bb_for_colour(&self, colour: &Colour) -> Bitboard {
        self.colour_info[colour.as_index()].piece_bb[Piece::Bishop.as_index()]
            | self.colour_info[colour.as_index()].piece_bb[Piece::Queen.as_index()]
    }

    pub const fn get_knight_bb_for_colour(&self, colour: &Colour) -> Bitboard {
        self.colour_info[colour.as_index()].piece_bb[Piece::Knight.as_index()]
    }

    pub const fn get_colour_bb(&self, colour: &Colour) -> Bitboard {
        self.colour_info[colour.as_index()].colour_bb
    }

    pub fn get_material(&self) -> Material {
        Material {
            white: self.colour_info[Colour::White.as_index()].material,
            black: self.colour_info[Colour::Black.as_index()].material,
        }
    }

    pub fn get_net_material(&self) -> Score {
        self.colour_info[Colour::White.as_index()]
            .material
            .wrapping_sub(self.colour_info[Colour::Black.as_index()].material) as Score
    }

    pub fn get_bitboard(&self) -> Bitboard {
        self.get_colour_bb(&Colour::White) | self.get_colour_bb(&Colour::Black)
    }

    pub fn get_king_sq(&self, colour: &Colour) -> Square {
        self.colour_info[colour.as_index()].king_sq
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
                let sq = Square::from_rank_file(r, f);

                if let Some((piece, colour)) =
                    self.get_piece_and_colour_on_square(&sq.expect("Invalid square"))
                {
                    debug_str.push_str(&Piece::label(&piece, &colour).to_string());
                    debug_str.push('\t');
                } else {
                    debug_str.push_str(".\t");
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

impl Default for Board {
    fn default() -> Self {
        Board {
            colour_info: [ColourInfo::default(); Colour::NUM_COLOURS],
            pieces: [None; Board::NUM_SQUARES],
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::colour::Colour;
    use crate::board::game_board::Board;
    use crate::board::piece::Piece;
    use crate::board::square::Square;
    use crate::io::fen;

    #[test]
    pub fn add_piece_king_square_as_expected() {
        let colours = [Colour::White, Colour::Black];

        for col in colours.iter() {
            let mut board = Board::new();

            for sq in Square::iterator() {
                assert!(board.get_bitboard().is_empty());
                board.add_piece(&Piece::King, col, sq);
                assert!(!board.get_bitboard().is_empty());

                assert_eq!(board.get_king_sq(col), *sq);

                // remove so state is restored.
                board.remove_piece(&Piece::King, col, sq);
            }
        }
    }

    #[test]
    pub fn add_remove_piece_square_state_as_expected() {
        let pce = Piece::Bishop;
        let col = Colour::White;
        let mut board = Board::new();

        let map = Square::iterator();
        for square in map {
            assert!(board.is_sq_empty(square));

            board.add_piece(&pce, &col, square);
            assert!(!board.is_sq_empty(square));

            board.remove_piece(&pce, &col, square);
            assert!(board.is_sq_empty(square));
        }
    }

    #[test]
    pub fn move_piece_square_state_as_expected() {
        let pce = Piece::Knight;
        let col = Colour::Black;

        let mut board = Board::new();

        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                assert!(board.is_sq_empty(from_sq));
                assert!(board.is_sq_empty(to_sq));

                board.add_piece(&pce, &col, from_sq);
                assert!(!board.is_sq_empty(from_sq));
                assert!(board.is_sq_empty(to_sq));

                board.move_piece(from_sq, to_sq, &pce, &col);
                assert!(board.is_sq_empty(from_sq));
                assert!(!board.is_sq_empty(to_sq));

                // clean up
                board.remove_piece(&pce, &col, to_sq);
            }
        }
    }

    #[test]
    pub fn get_piece_on_square_as_expected() {
        let pce = Piece::Rook;
        let col = Colour::Black;
        let mut board = Board::new();

        for square in Square::iterator() {
            assert!(board.is_sq_empty(square));

            board.add_piece(&pce, &col, square);
            assert!(!board.is_sq_empty(square));

            if let Some((piece, colour)) = board.get_piece_and_colour_on_square(square) {
                assert_eq!((pce, col), (piece, colour));
            }

            // clean up
            board.remove_piece(&pce, &col, square);
        }
    }

    #[test]
    pub fn get_bitboard_value_as_expected() {
        let mut board = Board::new();

        let pieces = [
            Piece::Pawn,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];

        let colours = [Colour::White, Colour::Black];

        for pce in pieces.iter() {
            for col in colours.iter() {
                for square in Square::iterator() {
                    board.add_piece(pce, col, square);

                    assert!(board.get_piece_bitboard(pce, col).is_set(square));

                    // clean up
                    board.remove_piece(pce, col, square);
                }
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
