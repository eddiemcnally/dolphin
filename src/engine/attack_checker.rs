use components::bitboard;
use components::board::Board;
use components::occupancy_masks;
use components::piece::Colour;
use components::piece::Piece;
use components::square::Square;
use std::ops::Shl;

pub fn is_king_sq_attacked(board: &Board, sq: Square, attacking_side: Colour) -> bool {
    match attacking_side {
        Colour::White => {
            let pawn_bb = board.get_piece_bitboard(&Piece::WHITE_PAWN);
            if pawn_bb != 0 && is_attacked_by_pawn_white(pawn_bb, sq) {
                return true;
            }
            if check_non_pawn_pieces_attacking(Colour::White, board, sq) {
                return true;
            }
        }
        Colour::Black => {
            let pawn_bb = board.get_piece_bitboard(&Piece::BLACK_PAWN);
            if pawn_bb != 0 && is_attacked_by_pawn_black(pawn_bb, sq) {
                return true;
            }
            if check_non_pawn_pieces_attacking(Colour::Black, board, sq) {
                return true;
            }
        }
    }
    false
}

pub fn is_castle_squares_attacked(
    board: &Board,
    sq_array: &[Square],
    attacking_side: Colour,
) -> bool {
    match attacking_side {
        Colour::White => {
            let pawn_bb = board.get_piece_bitboard(&Piece::WHITE_PAWN);
            for sq in sq_array.iter() {
                if pawn_bb != 0 && is_attacked_by_pawn_white(pawn_bb, *sq) {
                    return true;
                }
                if check_non_pawn_pieces_attacking(Colour::White, board, *sq) {
                    return true;
                }
            }
        }
        Colour::Black => {
            let pawn_bb = board.get_piece_bitboard(&Piece::BLACK_PAWN);
            for sq in sq_array.iter() {
                if pawn_bb != 0 && is_attacked_by_pawn_black(pawn_bb, *sq) {
                    return true;
                }
                if check_non_pawn_pieces_attacking(Colour::Black, board, *sq) {
                    return true;
                }
            }
        }
    }

    false
}

fn check_non_pawn_pieces_attacking(side: Colour, board: &Board, sq: Square) -> bool {
    if side == Colour::White {
        let knight_bb = board.get_piece_bitboard(&Piece::WHITE_KNIGHT);
        if knight_bb != 0 && is_knight_attacking(knight_bb, sq) {
            return true;
        }

        let horiz_vert_bb = board.get_white_rook_queen_bitboard();
        let all_pce_bb = board.get_bitboard();
        if horiz_vert_bb != 0 && is_horizontal_or_vertical_attacking(all_pce_bb, horiz_vert_bb, sq)
        {
            return true;
        }

        let diag_bb = board.get_white_bishop_queen_bitboard();
        if diag_bb != 0 && is_diagonally_attacked(sq, diag_bb, all_pce_bb) {
            return true;
        }

        let king_bb = board.get_piece_bitboard(&Piece::WHITE_KING);
        if is_attacked_by_king(king_bb, sq) {
            return true;
        }
    } else {
        let knight_bb = board.get_piece_bitboard(&Piece::BLACK_KNIGHT);
        if knight_bb != 0 && is_knight_attacking(knight_bb, sq) {
            return true;
        }

        let horiz_vert_bb = board.get_black_rook_queen_bitboard();
        let all_pce_bb = board.get_bitboard();
        if horiz_vert_bb != 0 && is_horizontal_or_vertical_attacking(all_pce_bb, horiz_vert_bb, sq)
        {
            return true;
        }

        let diag_bb = board.get_black_bishop_queen_bitboard();
        if diag_bb != 0 && is_diagonally_attacked(sq, diag_bb, all_pce_bb) {
            return true;
        }

        let king_bb = board.get_piece_bitboard(&Piece::BLACK_KING);
        if is_attacked_by_king(king_bb, sq) {
            return true;
        }
    }

    false
}

fn is_knight_attacking(pce_bitboard: u64, attack_sq: Square) -> bool {
    let mut pce_bb = pce_bitboard;

    while pce_bb != 0 {
        let sq = bitboard::pop_1st_bit(&mut pce_bb);
        let occ_mask = occupancy_masks::get_occupancy_mask_knight(sq);
        if bitboard::is_set(occ_mask, attack_sq) {
            return true;
        }
    }
    false
}

fn is_horizontal_or_vertical_attacking(
    all_piece_bb: u64,
    attack_pce_bb: u64,
    attack_sq: Square,
) -> bool {
    let mut pce_bb = attack_pce_bb;

    while pce_bb != 0 {
        let pce_sq = bitboard::pop_1st_bit(&mut pce_bb);

        if pce_sq.same_rank(attack_sq) || pce_sq.same_file(attack_sq) {
            // potentially attacking
            let blocking_pces = get_intervening_bitboard(pce_sq, attack_sq);
            if blocking_pces & all_piece_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }
    false
}

fn is_diagonally_attacked(attack_sq: Square, diag_bb: u64, all_pce_bb: u64) -> bool {
    let mut attack_pce_bb = diag_bb;

    while attack_pce_bb != 0 {
        let pce_sq = bitboard::pop_1st_bit(&mut attack_pce_bb);

        let diagonal_bb = occupancy_masks::get_occupancy_mask_bishop(pce_sq);
        if bitboard::is_set(diagonal_bb, attack_sq) {
            // potentially attacking, sharing a diagonal
            let blocking_pces = get_intervening_bitboard(pce_sq, attack_sq);
            if blocking_pces & all_pce_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }

    false
}

fn is_attacked_by_king(king_bb: u64, attacked_sq: Square) -> bool {
    let mut bb = king_bb;
    let attacking_king_sq = bitboard::pop_1st_bit(&mut bb);
    let king_occ_mask = occupancy_masks::get_occupancy_mask_king(attacking_king_sq);
    bitboard::is_set(king_occ_mask, attacked_sq)
}

fn is_attacked_by_pawn_white(pawn_bb: u64, attacked_sq: Square) -> bool {
    // -1 Rank and +/- 1 File
    let mut pawn_sq = Square::derive_relative_square(attacked_sq, -1, 1);
    if let Some(_) = pawn_sq {
        if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
            return true;
        }
    }

    pawn_sq = Square::derive_relative_square(attacked_sq, -1, -1);
    if let Some(_) = pawn_sq {
        if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
            return true;
        }
    }

    false
}

fn is_attacked_by_pawn_black(pawn_bb: u64, attacked_sq: Square) -> bool {
    // +1 Rank and +/- 1 File
    let mut pawn_sq = Square::derive_relative_square(attacked_sq, 1, 1);

    if let Some(_) = pawn_sq {
        if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
            return true;
        }
    }

    pawn_sq = Square::derive_relative_square(attacked_sq, 1, -1);
    if let Some(_) = pawn_sq {
        if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
            return true;
        }
    }
    false
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
#[inline(always)]
fn get_intervening_bitboard(sq1: Square, sq2: Square) -> u64 {
    const M1: u64 = 0xffff_ffff_ffff_ffff;
    const A2A7: u64 = 0x0001_0101_0101_0100;
    const B2G7: u64 = 0x0040_2010_0804_0200;
    const H1B7: u64 = 0x0002_0408_1020_4080;

    let btwn = (M1.shl(sq1 as u8)) ^ (M1.shl(sq2 as u8));
    let file = (sq2 as u64 & 7).wrapping_sub(sq1 as u64 & 7);
    let rank = ((sq2 as u64 | 7).wrapping_sub(sq1 as u64)) >> 3;
    let mut line = ((file & 7).wrapping_sub(1)) & A2A7; /* a2a7 if same file */
    line = line.wrapping_add((((rank & 7).wrapping_sub(1)) >> 58).wrapping_mul(2)); /* b1g1 if same rank */
    line = line.wrapping_add((((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & B2G7); /* b2g7 if same diagonal */
    line = line.wrapping_add((((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & H1B7); /* h1b7 if same antidiag */
    line = line.wrapping_mul(btwn & (btwn.wrapping_neg())); /* mul acts like shift by smaller square */
    line & btwn /* return the bits on that line in-between */
}
