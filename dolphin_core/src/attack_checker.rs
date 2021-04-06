use crate::bitboard;
use crate::board::Board;
use crate::occupancy_masks::OccupancyMasks;
use crate::piece::Colour;
use crate::piece::Piece;
use crate::square::Square;

pub fn is_king_sq_attacked(
    occ_masks: &OccupancyMasks,
    board: &Board,
    sq: Square,
    attacking_side: Colour,
) -> bool {
    match attacking_side {
        Colour::White => {
            let pawn_bb = board.get_piece_bitboard(Piece::WhitePawn);
            if pawn_bb != 0 && is_attacked_by_pawn_white(occ_masks, pawn_bb, sq) {
                return true;
            }
            if check_non_pawn_pieces_attacking(occ_masks, Colour::White, board, sq) {
                return true;
            }
        }
        Colour::Black => {
            let pawn_bb = board.get_piece_bitboard(Piece::BlackPawn);
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
            let pawn_bb = board.get_piece_bitboard(Piece::WhitePawn);
            for sq in sq_array.iter() {
                if pawn_bb != 0 && is_attacked_by_pawn_white(occ_masks, pawn_bb, *sq) {
                    return true;
                }
                if check_non_pawn_pieces_attacking(occ_masks, Colour::White, board, *sq) {
                    return true;
                }
            }
        }
        Colour::Black => {
            let pawn_bb = board.get_piece_bitboard(Piece::BlackPawn);
            for sq in sq_array.iter() {
                if pawn_bb != 0 && is_attacked_by_pawn_black(occ_masks, pawn_bb, *sq) {
                    return true;
                }
                if check_non_pawn_pieces_attacking(occ_masks, Colour::Black, board, *sq) {
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
        let knight_bb = board.get_piece_bitboard(Piece::WhiteKnight);
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

        let king_bb = board.get_piece_bitboard(Piece::WhiteKing);
        if is_attacked_by_king(occ_masks, king_bb, sq) {
            return true;
        }
    } else {
        let knight_bb = board.get_piece_bitboard(Piece::BlackKnight);
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

        let king_bb = board.get_piece_bitboard(Piece::BlackKing);
        if is_attacked_by_king(occ_masks, king_bb, sq) {
            return true;
        }
    }

    false
}

fn is_knight_attacking(occ_masks: &OccupancyMasks, pce_bitboard: u64, attack_sq: Square) -> bool {
    let mut pce_bb = pce_bitboard;

    while pce_bb != 0 {
        let sq = bitboard::pop_1st_bit(&mut pce_bb);
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
    let mut pce_bb = attack_pce_bb;

    while pce_bb != 0 {
        let pce_sq = bitboard::pop_1st_bit(&mut pce_bb);

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
    let mut attack_pce_bb = diag_bb;

    while attack_pce_bb != 0 {
        let pce_sq = bitboard::pop_1st_bit(&mut attack_pce_bb);

        let diagonal_bb = occ_masks.get_occupancy_mask_bishop(pce_sq);
        if bitboard::is_set(diagonal_bb, attack_sq) {
            // potentially attacking, sharing a diagonal
            let blocking_pces = occ_masks.get_inbetween_squares(pce_sq, attack_sq);
            if blocking_pces & all_pce_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }

    false
}

fn is_attacked_by_king(occ_masks: &OccupancyMasks, king_bb: u64, attacked_sq: Square) -> bool {
    let mut bb = king_bb;
    let attacking_king_sq = bitboard::pop_1st_bit(&mut bb);
    let king_occ_mask = occ_masks.get_occupancy_mask_king(attacking_king_sq);
    bitboard::is_set(king_occ_mask, attacked_sq)
}

fn is_attacked_by_pawn_white(
    occ_masks: &OccupancyMasks,
    pawn_bb: u64,
    attacked_sq: Square,
) -> bool {
    let wp_attacking_square = occ_masks.get_occ_mask_white_pawns_attacking_sq(attacked_sq);
    (pawn_bb & wp_attacking_square) != 0
}

fn is_attacked_by_pawn_black(
    occ_masks: &OccupancyMasks,
    pawn_bb: u64,
    attacked_sq: Square,
) -> bool {
    let bp_attacking_square = occ_masks.get_occ_mask_black_pawns_attacking_sq(attacked_sq);
    (pawn_bb & bp_attacking_square) != 0
}
