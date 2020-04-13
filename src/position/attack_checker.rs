use board::bitboard;
use board::board::Board;
use board::occupancy_masks;
use board::piece::Colour;
use board::piece::Piece;
use board::square::Square;
use std::ops::Shl;

#[derive(Default)]
struct CachedBitboards {
    pawn: u64,
    bishop: u64,
    knight: u64,
    rook: u64,
    queen: u64,
    king: u64,
    all_bb: u64,
}

pub fn is_king_sq_attacked(board: &Board, sq: Square, attacking_side: Colour) -> bool {
    let mut cb = CachedBitboards::default();

    match attacking_side {
        Colour::White => {
            populate_white_bitboards(board, &mut cb);

            if cb.pawn != 0 {
                if is_attacked_by_pawn_white(cb.pawn, sq) {
                    return true;
                }
            }
            if check_non_pawn_pieces_attacking(&cb, sq) {
                return true;
            }
        }
        Colour::Black => {
            populate_black_bitboards(board, &mut cb);

            if cb.pawn != 0 {
                if is_attacked_by_pawn_black(cb.pawn, sq) {
                    return true;
                }
            }
            if check_non_pawn_pieces_attacking(&cb, sq) {
                return true;
            }
        }
    }
    return false;
}

pub fn is_castle_squares_attacked(
    board: &Board,
    sq_array: &[Square],
    attacking_side: Colour,
) -> bool {
    let mut cb = CachedBitboards::default();

    match attacking_side {
        Colour::White => {
            populate_white_bitboards(board, &mut cb);

            for sq in sq_array.into_iter() {
                if cb.pawn != 0 {
                    if is_attacked_by_pawn_white(cb.pawn, *sq) {
                        return true;
                    }
                }
                if check_non_pawn_pieces_attacking(&cb, *sq) {
                    return true;
                }
            }
        }
        Colour::Black => {
            populate_black_bitboards(board, &mut cb);

            for sq in sq_array.into_iter() {
                if cb.pawn != 0 {
                    if is_attacked_by_pawn_black(cb.pawn, *sq) {
                        return true;
                    }
                }
                if check_non_pawn_pieces_attacking(&cb, *sq) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn populate_white_bitboards(board: &Board, cache: &mut CachedBitboards) {
    cache.pawn = board.get_piece_bitboard(Piece::WhitePawn);
    cache.knight = board.get_piece_bitboard(Piece::WhiteKnight);
    cache.bishop = board.get_piece_bitboard(Piece::WhiteBishop);
    cache.rook = board.get_piece_bitboard(Piece::WhiteRook);
    cache.queen = board.get_piece_bitboard(Piece::WhiteQueen);
    cache.king = board.get_piece_bitboard(Piece::WhiteKing);
    cache.all_bb = board.get_bitboard();
}

fn populate_black_bitboards(board: &Board, cache: &mut CachedBitboards) {
    cache.pawn = board.get_piece_bitboard(Piece::BlackPawn);
    cache.knight = board.get_piece_bitboard(Piece::BlackKnight);
    cache.bishop = board.get_piece_bitboard(Piece::BlackBishop);
    cache.rook = board.get_piece_bitboard(Piece::BlackRook);
    cache.queen = board.get_piece_bitboard(Piece::BlackQueen);
    cache.king = board.get_piece_bitboard(Piece::BlackKing);
    cache.all_bb = board.get_bitboard();
}

fn check_non_pawn_pieces_attacking(cached_bb: &CachedBitboards, sq: Square) -> bool {
    if cached_bb.knight != 0 {
        if is_knight_attacking(cached_bb.knight, sq) {
            return true;
        }
    }

    // combine piece bitboards
    let horiz_vert_bb = cached_bb.rook | cached_bb.queen;
    if horiz_vert_bb != 0 {
        if is_horizontal_or_vertical_attacking(cached_bb.all_bb, horiz_vert_bb, sq) {
            return true;
        }
    }

    let diag_bb = cached_bb.bishop | cached_bb.queen;
    if diag_bb != 0 {
        if is_diagonally_attacked(sq, diag_bb, cached_bb.all_bb) {
            return true;
        }
    }

    if is_attacked_by_king(cached_bb.king, sq) {
        return true;
    }

    return false;
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
    return false;
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
    return false;
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

    return false;
}

fn is_attacked_by_king(king_bb: u64, attacked_sq: Square) -> bool {
    let mut bb = king_bb;
    let attacking_king_sq = bitboard::pop_1st_bit(&mut bb);
    let king_occ_mask = occupancy_masks::get_occupancy_mask_king(attacking_king_sq);
    return bitboard::is_set(king_occ_mask, attacked_sq);
}

fn is_attacked_by_pawn_white(pawn_bb: u64, attacked_sq: Square) -> bool {
    // -1 Rank and +/- 1 File
    let mut pawn_sq = Square::derive_relative_square(attacked_sq, -1, 1);
    match pawn_sq {
        Some(_) => {
            if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
                return true;
            }
        }
        None => {}
    }

    pawn_sq = Square::derive_relative_square(attacked_sq, -1, -1);
    match pawn_sq {
        Some(_) => {
            if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
                return true;
            }
        }
        None => {}
    }

    return false;
}

fn is_attacked_by_pawn_black(pawn_bb: u64, attacked_sq: Square) -> bool {
    // +1 Rank and +/- 1 File
    let mut pawn_sq = Square::derive_relative_square(attacked_sq, 1, 1);
    match pawn_sq {
        Some(_) => {
            if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
                return true;
            }
        }
        None => {}
    }

    pawn_sq = Square::derive_relative_square(attacked_sq, 1, -1);
    match pawn_sq {
        Some(_) => {
            if bitboard::is_set(pawn_bb, pawn_sq.unwrap()) {
                return true;
            }
        }
        None => {}
    }
    return false;
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
#[inline(always)]
fn get_intervening_bitboard(sq1: Square, sq2: Square) -> u64 {
    const M1: u64 = 0xffffffffffffffff;
    const A2A7: u64 = 0x0001010101010100;
    const B2G7: u64 = 0x0040201008040200;
    const H1B7: u64 = 0x0002040810204080;

    let btwn = (M1.shl(sq1 as u8)) ^ (M1.shl(sq2 as u8));
    let file = (sq2 as u64 & 7).wrapping_sub(sq1 as u64 & 7);
    let rank = ((sq2 as u64 | 7).wrapping_sub(sq1 as u64)) >> 3;
    let mut line = ((file & 7).wrapping_sub(1)) & A2A7; /* a2a7 if same file */
    line = line.wrapping_add((((rank & 7).wrapping_sub(1)) >> 58).wrapping_mul(2)); /* b1g1 if same rank */
    line = line.wrapping_add((((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & B2G7); /* b2g7 if same diagonal */
    line = line.wrapping_add((((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & H1B7); /* h1b7 if same antidiag */
    line = line.wrapping_mul(btwn & (btwn.wrapping_neg())); /* mul acts like shift by smaller square */
    return line & btwn; /* return the bits on that line in-between */
}
