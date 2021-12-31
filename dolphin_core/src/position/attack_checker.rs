use crate::board::bitboard;
use crate::board::bitboard::SquareIterator;
use crate::board::colour::Colour;
use crate::board::game_board::Board;
use crate::board::occupancy_masks::OccupancyMasks;
use crate::board::piece;
use crate::board::square::Square;

pub fn is_sq_attacked(
    occ_masks: &OccupancyMasks,
    board: &Board,
    sq: Square,
    attacking_side: Colour,
) -> bool {
    match attacking_side {
        Colour::White => {
            let pawn_bb = board.get_piece_bitboard(&piece::WHITE_PAWN);
            if pawn_bb != 0 && is_attacked_by_pawn_white(occ_masks, pawn_bb, sq) {
                return true;
            }
            if check_non_pawn_pieces_attacking(occ_masks, Colour::White, board, sq) {
                return true;
            }
        }
        Colour::Black => {
            let pawn_bb = board.get_piece_bitboard(&piece::BLACK_PAWN);
            if pawn_bb != 0 && is_attacked_by_pawn_black(occ_masks, pawn_bb, sq) {
                return true;
            }
            if check_non_pawn_pieces_attacking(occ_masks, Colour::Black, board, sq) {
                return true;
            }
        }
    }
    false
}

pub fn is_castle_squares_attacked(
    occ_masks: &OccupancyMasks,
    board: &Board,
    sq_array: &[Square],
    attacking_side: Colour,
) -> bool {
    match attacking_side {
        Colour::White => {
            let pawn_bb = board.get_piece_bitboard(&piece::WHITE_PAWN);
            for sq in sq_array.iter() {
                if check_non_pawn_pieces_attacking(occ_masks, Colour::White, board, *sq) {
                    return true;
                }
                if pawn_bb != 0 && is_attacked_by_pawn_white(occ_masks, pawn_bb, *sq) {
                    return true;
                }
            }
        }
        Colour::Black => {
            let pawn_bb = board.get_piece_bitboard(&piece::BLACK_PAWN);
            for sq in sq_array.iter() {
                if check_non_pawn_pieces_attacking(occ_masks, Colour::Black, board, *sq) {
                    return true;
                }
                if pawn_bb != 0 && is_attacked_by_pawn_black(occ_masks, pawn_bb, *sq) {
                    return true;
                }
            }
        }
    }

    false
}

fn check_non_pawn_pieces_attacking(
    occ_masks: &OccupancyMasks,
    side: Colour,
    board: &Board,
    sq: Square,
) -> bool {
    if side == Colour::White {
        let knight_bb = board.get_piece_bitboard(&piece::WHITE_KNIGHT);
        if knight_bb != 0 && is_knight_attacking(occ_masks, knight_bb, sq) {
            return true;
        }

        let horiz_vert_bb = board.get_white_rook_queen_bitboard();
        let all_pce_bb = board.get_bitboard();
        if horiz_vert_bb != 0
            && is_horizontal_or_vertical_attacking(occ_masks, all_pce_bb, horiz_vert_bb, sq)
        {
            return true;
        }

        let diag_bb = board.get_white_bishop_queen_bitboard();
        if diag_bb != 0 && is_diagonally_attacked(occ_masks, sq, diag_bb, all_pce_bb) {
            return true;
        }

        let king_sq = board.get_king_sq(Colour::White);
        if is_attacked_by_king(occ_masks, king_sq, sq) {
            return true;
        }
    } else {
        let knight_bb = board.get_piece_bitboard(&piece::BLACK_KNIGHT);
        if knight_bb != 0 && is_knight_attacking(occ_masks, knight_bb, sq) {
            return true;
        }

        let horiz_vert_bb = board.get_black_rook_queen_bitboard();
        let all_pce_bb = board.get_bitboard();
        if horiz_vert_bb != 0
            && is_horizontal_or_vertical_attacking(occ_masks, all_pce_bb, horiz_vert_bb, sq)
        {
            return true;
        }

        let diag_bb = board.get_black_bishop_queen_bitboard();
        if diag_bb != 0 && is_diagonally_attacked(occ_masks, sq, diag_bb, all_pce_bb) {
            return true;
        }

        let king_sq = board.get_king_sq(Colour::Black);
        if is_attacked_by_king(occ_masks, king_sq, sq) {
            return true;
        }
    }

    false
}

fn is_knight_attacking(occ_masks: &OccupancyMasks, pce_bitboard: u64, attack_sq: Square) -> bool {
    let iter = SquareIterator::new(pce_bitboard);
    for sq in iter {
        let occ_mask = occ_masks.get_occupancy_mask_knight(sq);
        if bitboard::is_set(occ_mask, attack_sq) {
            return true;
        }
    }
    false
}

fn is_horizontal_or_vertical_attacking(
    occ_masks: &OccupancyMasks,
    all_piece_bb: u64,
    attack_pce_bb: u64,
    attack_sq: Square,
) -> bool {
    // do a quick check to see if any piece is sharing a rank and file
    // with the square
    let vert_occ_masks = occ_masks.get_vertical_mask(attack_sq);
    let horiz_occ_masks = occ_masks.get_horizontal_mask(attack_sq);
    let horiz_vert_sq_mask = vert_occ_masks | horiz_occ_masks;
    if attack_pce_bb & horiz_vert_sq_mask == 0 {
        // no diagonals shared
        return false;
    }

    let iter = SquareIterator::new(attack_pce_bb);
    for pce_sq in iter {
        if pce_sq.same_rank(attack_sq) || pce_sq.same_file(attack_sq) {
            // potentially attacking
            let blocking_pces = occ_masks.get_inbetween_squares(pce_sq, attack_sq);
            if blocking_pces & all_piece_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }
    false
}

fn is_diagonally_attacked(
    occ_masks: &OccupancyMasks,
    attack_sq: Square,
    diag_bb: u64,
    all_pce_bb: u64,
) -> bool {
    // do a quick check to see if any piece is sharing a diagonal with
    // the square
    let diag_occ_masks = occ_masks.get_diag_antidiag_mask(attack_sq);
    let sq_mask = diag_occ_masks.get_anti_diag_mask() | diag_occ_masks.get_diag_mask();
    if sq_mask & diag_bb == 0 {
        // no diagonals shared
        return false;
    }

    let iter = SquareIterator::new(diag_bb);
    for pce_sq in iter {
        // diagonal mask will also work for queen
        let diagonal_bb = occ_masks.get_occupancy_mask_bishop(pce_sq);
        if bitboard::is_set(diagonal_bb, attack_sq) {
            // potentially attacking....ie, sharing a diagonal
            let blocking_pces = occ_masks.get_inbetween_squares(pce_sq, attack_sq);
            if blocking_pces & all_pce_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }

    false
}

#[inline(always)]
fn is_attacked_by_king(occ_masks: &OccupancyMasks, king_sq: Square, attacked_sq: Square) -> bool {
    let king_occ_mask = occ_masks.get_occupancy_mask_king(king_sq);
    bitboard::is_set(king_occ_mask, attacked_sq)
}
#[inline(always)]
fn is_attacked_by_pawn_white(
    occ_masks: &OccupancyMasks,
    pawn_bb: u64,
    attacked_sq: Square,
) -> bool {
    let wp_attacking_square = occ_masks.get_occ_mask_white_pawns_attacking_sq(attacked_sq);
    (pawn_bb & wp_attacking_square) != 0
}
#[inline(always)]
fn is_attacked_by_pawn_black(
    occ_masks: &OccupancyMasks,
    pawn_bb: u64,
    attacked_sq: Square,
) -> bool {
    let bp_attacking_square = occ_masks.get_occ_mask_black_pawns_attacking_sq(attacked_sq);
    (pawn_bb & bp_attacking_square) != 0
}

#[cfg(test)]
pub mod tests {
    use crate::board::colour::Colour;
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::board::square::*;
    use crate::io::fen;
    use crate::position::attack_checker;
    use crate::position::game_position::Position;
    use crate::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn is_attacked_by_white_pawn() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_G5,
            Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_pawn() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_H4,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_bishop() {
        let fen = "8/2B5/8/1p2kPp1/7P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E5,
            Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_bishop() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/2b5 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E3,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_knight() {
        let fen = "8/8/8/1p2kPp1/2N4P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E5,
            Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_knight() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/2n5/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E3,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_rook() {
        let fen = "4R3/8/8/1p2kPp1/7P/4K3/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E5,
            Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_rook() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/4r3 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E3,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_queen() {
        let fen = "8/8/8/1p2kPp1/7P/4K3/8/Q7 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E5,
            Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_queen() {
        let fen = "8/8/8/1p2kPp1/7P/q3K3/8/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_E3,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_attacked_by_white_king() {
        let fen = "8/8/8/1p2kPp1/1K5P/8/8/8 w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_B5,
            Colour::White
        ));
    }

    #[test]
    pub fn is_attacked_by_black_king() {
        let fen = "8/8/8/1p2kPp1/7P/3K4/8/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_sq_attacked(
            &occ_masks,
            pos.board(),
            SQUARE_F5,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_kingside_castle_sq_e1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_E1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/4q3/2P5/PP1P2PP/RNBQK2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_kingside_castle_sq_f1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_F1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/2q5/2P5/PP1P2PP/RNBQK2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_kingside_castle_sq_g1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_G1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3q4/2P5/PP1P2PP/RNBQK2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_e1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_E1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3P3q/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_d1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_D1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3P2q1/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_c1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_C1];

        let fen = "rn2kbnr/pp1p1ppp/8/2p5/3P1q2/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_white_queenside_castle_sq_b1_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_B1];

        let fen = "rnq1kbnr/pp1p1ppp/8/2p5/3Pb3/2P5/PP4PP/R3K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::Black
        ));
    }

    #[test]
    pub fn is_black_kingside_castle_sq_e8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_E8];

        let fen = "r3k2r/pp4pp/2p5/7B/8/2P5/PP1P2PP/RNB1K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }

    #[test]
    pub fn is_black_kingside_castle_sq_f8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_F8];

        let fen = "r3k2r/pp4pp/8/2B5/8/2P5/PP1P2PP/RNB1K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }

    #[test]
    pub fn is_black_kingside_castle_sq_g8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_G8];

        let fen = "r3k2r/pp4pp/8/3B4/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_e8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_E8];

        let fen = "r3k2r/pp4pp/8/7B/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_d8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_D8];

        let fen = "r3k2r/pp4pp/8/6B1/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_c8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_C8];

        let fen = "r3k2r/pp4pp/8/5B2/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }

    #[test]
    pub fn is_black_queenside_castle_sq_b8_attacked_() {
        const SQUARE_TO_CHECK: [Square; 1] = [SQUARE_B8];

        let fen = "r3k2r/pp4pp/8/4B3/8/2P5/PP1P2PP/RN2K2R b KQkq - 0 2";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(attack_checker::is_castle_squares_attacked(
            &occ_masks,
            pos.board(),
            &SQUARE_TO_CHECK,
            Colour::White
        ));
    }
}
