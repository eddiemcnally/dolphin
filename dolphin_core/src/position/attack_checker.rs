use crate::board::bitboard::Bitboard;
use crate::board::colour::Colour;
use crate::board::game_board::Board;
use crate::board::occupancy_masks::OccupancyMasks;
use crate::board::piece::Piece;
use crate::board::square::Square;

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub struct AttackChecker {}

impl AttackChecker {
    pub fn new() -> AttackChecker {
        AttackChecker::default()
    }

    pub fn is_sq_attacked(
        &self,
        occ_masks: &OccupancyMasks,
        board: &Board,
        sq: &Square,
        attacking_side: &Colour,
    ) -> bool {
        match attacking_side {
            Colour::White => {
                let pawn_bb = board.get_piece_bitboard(&Piece::Pawn, &Colour::White);
                let wp_attacking_square =
                    occ_masks.get_occ_mask_pawns_attacking_sq(&Colour::White, &sq);
                if !(pawn_bb & wp_attacking_square).is_empty() {
                    return true;
                }

                let knight_bb = board.get_piece_bitboard(&Piece::Knight, &Colour::White);
                for from_sq in knight_bb.iterator() {
                    if occ_masks.get_occupancy_mask_knight(&from_sq).is_set(&sq) {
                        return true;
                    }
                }

                let horiz_vert_bb = board.get_piece_bitboard(&Piece::Rook, &Colour::White)
                    | board.get_piece_bitboard(&Piece::Queen, &Colour::White);

                let all_pce_bb = board.get_bitboard();
                // check to see if the sqaure being attacked shares a rank or file
                // with any of the rooks or queens before doing a detailed analysis
                // of potential blocking pieces
                let horiz_vert_sq_mask =
                    occ_masks.get_vertical_mask(&sq) | occ_masks.get_horizontal_mask(&sq);
                let shares_rank_or_file = !(horiz_vert_bb & horiz_vert_sq_mask).is_empty();
                if shares_rank_or_file
                    && self.is_horizontal_or_vertical_attacking(
                        occ_masks,
                        &all_pce_bb,
                        &horiz_vert_bb,
                        sq,
                    )
                {
                    return true;
                }

                let diag_bb = board.get_piece_bitboard(&Piece::Bishop, &Colour::White)
                    | board.get_piece_bitboard(&Piece::Queen, &Colour::White);
                // check to see if the sqaure being attacked shares a diagonal
                // with any of the bishops or queens before doing a detailed analysis
                // of potential blocking pieces
                let sq_mask =
                    occ_masks.get_diagonal_mask(&sq) | occ_masks.get_antidiagonal_mask(&sq);
                if !(sq_mask & diag_bb).is_empty() {
                    // possible attack, check for blocking pieces
                    if self.is_diagonally_attacked(occ_masks, sq, &diag_bb, &all_pce_bb) {
                        return true;
                    }
                }

                let king_sq = board.get_king_sq(&Colour::White);
                if occ_masks.get_occupancy_mask_king(&king_sq).is_set(&sq) {
                    return true;
                }
            }
            Colour::Black => {
                let pawn_bb = board.get_piece_bitboard(&Piece::Pawn, &Colour::Black);
                let bp_attacking_square =
                    occ_masks.get_occ_mask_pawns_attacking_sq(&Colour::Black, &sq);
                if !(pawn_bb & bp_attacking_square).is_empty() {
                    return true;
                }

                let knight_bb = board.get_piece_bitboard(&Piece::Knight, &Colour::Black);
                for from_sq in knight_bb.iterator() {
                    if occ_masks.get_occupancy_mask_knight(&from_sq).is_set(&sq) {
                        return true;
                    }
                }

                let horiz_vert_bb = board.get_piece_bitboard(&Piece::Rook, &Colour::Black)
                    | board.get_piece_bitboard(&Piece::Queen, &Colour::Black);

                let all_pce_bb = board.get_bitboard();
                // check to see if the sqaure being attacked shares a rank or file
                // with any of the rooks or queens before doing a detailed analysis
                // of potential blocking pieces
                let horiz_vert_sq_mask =
                    occ_masks.get_vertical_mask(&sq) | occ_masks.get_horizontal_mask(&sq);
                let shares_rank_or_file = !(horiz_vert_bb & horiz_vert_sq_mask).is_empty();
                if shares_rank_or_file
                    && self.is_horizontal_or_vertical_attacking(
                        occ_masks,
                        &all_pce_bb,
                        &horiz_vert_bb,
                        sq,
                    )
                {
                    return true;
                }

                let diag_bb = board.get_piece_bitboard(&Piece::Bishop, &Colour::Black)
                    | board.get_piece_bitboard(&Piece::Queen, &Colour::Black);
                // check to see if the sqaure being attacked shares a diagonal
                // with any of the bishops or queens before doing a detailed analysis
                // of potential blocking pieces
                let sq_mask =
                    occ_masks.get_diagonal_mask(&sq) | occ_masks.get_antidiagonal_mask(&sq);
                if !(sq_mask & diag_bb).is_empty() {
                    // possible attack, check for blocking pieces
                    if self.is_diagonally_attacked(occ_masks, sq, &diag_bb, &all_pce_bb) {
                        return true;
                    }
                }

                let king_sq = board.get_king_sq(&Colour::Black);
                if occ_masks.get_occupancy_mask_king(&king_sq).is_set(&sq) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_castle_squares_attacked(
        &self,
        occ_masks: &OccupancyMasks,
        board: &Board,
        sq_array: &[Square],
        attacking_side: &Colour,
    ) -> bool {
        for sq in sq_array {
            if self.is_sq_attacked(occ_masks, board, sq, &attacking_side) {
                return true;
            }
        }

        false
    }

    fn is_horizontal_or_vertical_attacking(
        &self,
        occ_masks: &OccupancyMasks,
        all_piece_bb: &Bitboard,
        attack_pce_bb: &Bitboard,
        attack_sq: &Square,
    ) -> bool {
        for pce_sq in attack_pce_bb.iterator() {
            if pce_sq.same_rank(attack_sq) || pce_sq.same_file(attack_sq) {
                // potentially attacking
                let blocking_pces = occ_masks.get_inbetween_squares(&pce_sq, &attack_sq);
                if (blocking_pces & *all_piece_bb).is_empty() {
                    // no blocking pieces, attacked
                    return true;
                }
            }
        }
        false
    }

    fn is_diagonally_attacked(
        &self,
        occ_masks: &OccupancyMasks,
        attack_sq: &Square,
        diag_bb: &Bitboard,
        all_pce_bb: &Bitboard,
    ) -> bool {
        for pce_sq in diag_bb.iterator() {
            // diagonal mask will also work for queen
            if occ_masks
                .get_occupancy_mask_bishop(&pce_sq)
                .is_set(&attack_sq)
            {
                // potentially attacking....ie, sharing a diagonal
                let blocking_pces = occ_masks.get_inbetween_squares(&pce_sq, &attack_sq);
                if (blocking_pces & *all_pce_bb).is_empty() {
                    // no blocking pieces, attacked
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::colour::Colour;
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::board::square::*;
    use crate::io::fen;
    use crate::position::attack_checker::AttackChecker;
    use crate::position::game_position::Position;
    use crate::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn is_attacked_by_white_pawn() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::G5,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_pawn() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::H4,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_bishop() {
        let fen = "8/2B5/8/1p2kPp1/7P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E5,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_bishop() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/2b5 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E3,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_knight() {
        let fen = "8/8/8/1p2kPp1/2N4P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E5,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_knight() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/2n5/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E3,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_rook() {
        let fen = "4R3/8/8/1p2kPp1/7P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E5,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_rook() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/4r3 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E3,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_queen() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/Q7 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E5,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_queen() {
        let fen = "8/8/8/1p2kPp1/7P/q3K3/8/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::E3,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_king() {
        let fen = "8/8/8/1p2kPp1/1K5P/8/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::B5,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_king() {
        let fen = "8/8/8/1p2kPp1/7P/3K4/8/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_sq_attacked(
            &occ_masks,
            pos.board(),
            &Square::F5,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_kingside_castle_sq_e1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::E1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/4q3/2P5/PP1P2PP/RNBQK2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_kingside_castle_sq_f1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::F1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/2q5/2P5/PP1P2PP/RNBQK2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_kingside_castle_sq_g1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::G1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3q4/2P5/PP1P2PP/RNBQK2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_e1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::E1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3P3q/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_d1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::D1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3P2q1/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_c1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::C1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3P1q2/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_b1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::B1];

        let fen = "rnq1kbnr/pp1p1ppp/8/2p5/3Pb3/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::Black
        ));
    }

    #[test]
    pub fn is_black_kingside_castle_sq_e8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::E8];

        let fen = "r3k2r/pp4pp/2p5/7B/8/2P5/PP1P2PP/RNB1K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_black_kingside_castle_sq_f8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::F8];

        let fen = "r3k2r/pp4pp/8/2B5/8/2P5/PP1P2PP/RNB1K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_black_kingside_castle_sq_g8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::G8];

        let fen = "r3k2r/pp4pp/8/3B4/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_e8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::E8];

        let fen = "r3k2r/pp4pp/8/7B/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_d8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::D8];

        let fen = "r3k2r/pp4pp/8/6B1/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_c8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::C8];

        let fen = "r3k2r/pp4pp/8/5B2/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_b8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [Square::B8];

        let fen = "r3k2r/pp4pp/8/4B3/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(attack_checker.is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            &Colour::White
        ));
    }
}
