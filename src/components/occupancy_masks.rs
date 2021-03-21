use components;
use components::bitboard;
use components::board;
use components::square::File;
use components::square::Rank;
use components::square::Square;
use enum_primitive::FromPrimitive;
use std::ops::Shl;

// Bitboards representing commonly used ranks
pub const RANK_2_BB: u64 = 0x0000_0000_0000_FF00;
pub const RANK_7_BB: u64 = 0x00FF_0000_0000_0000;

const RANK_MASK: u64 = 0x0000_0000_0000_00ff;
const FILE_MASK: u64 = 0x0101_0101_0101_0101;

// bitboards for squares between castle squares
pub const CASTLE_MASK_WK: u64 = 0x0000_0000_0000_0060;
pub const CASTLE_MASK_WQ: u64 = 0x0000_0000_0000_000E;
pub const CASTLE_MASK_BK: u64 = 0x6000_0000_0000_0000;
pub const CASTLE_MASK_BQ: u64 = 0x0E00_0000_0000_0000;

const FILE_A_BB: u64 = FILE_MASK;
const FILE_H_BB: u64 = FILE_A_BB << 7;

pub fn get_occupancy_mask_bishop(sq: Square) -> u64 {
    *BISHOP_OCCUPANCY_MASKS.get(sq.to_offset()).unwrap()
}

pub fn get_occupancy_mask_knight(sq: Square) -> u64 {
    *KNIGHT_OCCUPANCY_MASKS.get(sq.to_offset()).unwrap()
}

pub fn get_occupancy_mask_rook(sq: Square) -> u64 {
    *ROOK_OCCUPANCY_MASKS.get(sq.to_offset()).unwrap()
}

pub fn get_occupancy_mask_queen(sq: Square) -> u64 {
    *QUEEN_OCCUPANCY_MASKS.get(sq.to_offset()).unwrap()
}

pub fn get_occupancy_mask_king(sq: Square) -> u64 {
    *KING_OCCUPANCY_MASKS.get(sq.to_offset()).unwrap()
}

pub fn get_inbetween_squares(sq1: Square, sq2: Square) -> u64 {
    INBETWEEN_SQUARE_MASK[sq1.to_offset()][sq2.to_offset()]
}

pub fn get_vertical_move_mask(sq: Square) -> u64 {
    let file = sq.file();
    return FILE_MASK << (file as u8);
}

pub fn get_horizontal_move_mask(sq: Square) -> u64 {
    let rank = sq.rank();
    return RANK_MASK << ((rank as u8) << 3);
}

pub fn get_diagonal_move_mask(sq: Square) -> u64 {
    *DIAGONAL_MOVE_MASKS.get(sq.to_offset()).unwrap()
}
pub fn get_anti_diagonal_move_mask(sq: Square) -> u64 {
    *ANTI_DIAGONAL_MOVE_MASKS.get(sq.to_offset()).unwrap()
}

#[inline(always)]
const fn north_east(bb: u64) -> u64 {
    (bb & !FILE_H_BB) << 9
}

#[inline(always)]
const fn south_east(bb: u64) -> u64 {
    (bb & !FILE_H_BB) >> 7
}

#[inline(always)]
const fn north_west(bb: u64) -> u64 {
    (bb & !FILE_A_BB) << 7
}

#[inline(always)]
const fn south_west(bb: u64) -> u64 {
    (bb & !FILE_A_BB) >> 9
}

#[inline(always)]
pub fn get_occ_mask_white_pawns_attacking_sq(sq: Square) -> u64 {
    let bb = sq.get_square_as_bb();
    south_east(bb) | south_west(bb)
}
#[inline(always)]
pub fn get_occ_mask_black_pawns_attacking_sq(sq: Square) -> u64 {
    let bb = sq.get_square_as_bb();
    north_east(bb) | north_west(bb)
}
#[inline(always)]
pub fn get_occ_mask_white_pawn_capture_non_first_double_move(sq: Square) -> u64 {
    let bb = sq.get_square_as_bb();
    north_east(bb) | north_west(bb)
}
#[inline(always)]
pub fn get_occ_mask_black_pawn_capture_non_first_double_move(sq: Square) -> u64 {
    let bb = sq.get_square_as_bb();
    south_east(bb) | south_west(bb)
}
#[inline(always)]
pub fn get_occ_mask_white_pawn_attack_squares(pawn_sq: Square) -> u64 {
    let bb = pawn_sq.get_square_as_bb();
    north_east(bb) | north_west(bb)
}
#[inline(always)]
pub fn get_occ_mask_black_pawn_attack_squares(pawn_sq: Square) -> u64 {
    let bb = pawn_sq.get_square_as_bb();
    south_east(bb) | south_west(bb)
}

lazy_static! {
    // the order of these is important....bishop uses the diag and anti-diag masks, the queen uses both rook and bishop masks
    static ref KNIGHT_OCCUPANCY_MASKS: [u64; board::NUM_SQUARES] = populate_knight_occupancy_mask_array();
    static ref DIAGONAL_MOVE_MASKS: [u64; board::NUM_SQUARES] = populate_diagonal_mask_array();
    static ref ANTI_DIAGONAL_MOVE_MASKS: [u64; board::NUM_SQUARES] = populate_antidiagonal_mask_array();
    static ref BISHOP_OCCUPANCY_MASKS: [u64; board::NUM_SQUARES] = populate_bishop_mask_array();
    static ref ROOK_OCCUPANCY_MASKS: [u64; board::NUM_SQUARES] = populate_rook_mask_array();
    static ref QUEEN_OCCUPANCY_MASKS: [u64; board::NUM_SQUARES] = populate_queen_mask_array();
    static ref KING_OCCUPANCY_MASKS: [u64; board::NUM_SQUARES] = populate_king_mask_array();
    static ref INBETWEEN_SQUARE_MASK:  [[u64;board::NUM_SQUARES];board::NUM_SQUARES] = populate_intervening_bitboard_array();
}

fn populate_knight_occupancy_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        // rank + 2, file +/- 1
        set_dest_sq_if_valid(&mut bb, *sq, 2, 1);
        set_dest_sq_if_valid(&mut bb, *sq, 2, -1);

        // rank + 1, file +/- 2
        set_dest_sq_if_valid(&mut bb, *sq, 1, 2);
        set_dest_sq_if_valid(&mut bb, *sq, 1, -2);

        // rank - 1, file +/- 2
        set_dest_sq_if_valid(&mut bb, *sq, -1, 2);
        set_dest_sq_if_valid(&mut bb, *sq, -1, -2);

        // rank - 2, file +/- 1
        set_dest_sq_if_valid(&mut bb, *sq, -2, 1);
        set_dest_sq_if_valid(&mut bb, *sq, -2, -1);

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_king_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        // rank+1, file -1/0/+1
        set_dest_sq_if_valid(&mut bb, *sq, 1, -1);
        set_dest_sq_if_valid(&mut bb, *sq, 1, 0);
        set_dest_sq_if_valid(&mut bb, *sq, 1, 1);

        // rank, file -1/+1
        set_dest_sq_if_valid(&mut bb, *sq, 0, -1);
        set_dest_sq_if_valid(&mut bb, *sq, 0, 1);

        // rank-1, file -1/0/+1
        set_dest_sq_if_valid(&mut bb, *sq, -1, -1);
        set_dest_sq_if_valid(&mut bb, *sq, -1, 0);
        set_dest_sq_if_valid(&mut bb, *sq, -1, 1);

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_diagonal_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        // move SW
        let mut rank_offset = sq.rank() as i8;
        let mut file_offset = sq.file() as i8;
        loop {
            rank_offset -= 1;
            file_offset -= 1;

            let r = Rank::from_i8(rank_offset);
            let f = File::from_i8(file_offset);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);
            //println!("Square {}, derived {}", sq, derived_sq);
            } else {
                break;
            }
        }

        // move NE
        rank_offset = sq.rank() as i8;
        file_offset = sq.file() as i8;

        loop {
            rank_offset += 1;
            file_offset += 1;

            let r = Rank::from_i8(rank_offset);
            let f = File::from_i8(file_offset);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);
            //println!("Square {}, derived {}", sq, derived_sq);
            } else {
                break;
            }
        }

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_offset()] = bb;
    }

    retval
}

fn populate_antidiagonal_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        // move NW
        let mut rank_offset = sq.rank() as i8;
        let mut file_offset = sq.file() as i8;
        loop {
            rank_offset += 1;
            file_offset -= 1;

            let r = Rank::from_i8(rank_offset);
            let f = File::from_i8(file_offset);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);
            //println!("Square {}, derived {}", sq, derived_sq);
            } else {
                break;
            }
        }

        // move SE
        rank_offset = sq.rank() as i8;
        file_offset = sq.file() as i8;

        loop {
            rank_offset -= 1;
            file_offset += 1;

            let r = Rank::from_i8(rank_offset);
            let f = File::from_i8(file_offset);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);
            //println!("Square {}, derived {}", sq, derived_sq);
            } else {
                break;
            }
        }

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_bishop_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];
    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb = get_diagonal_move_mask(*sq) | get_anti_diagonal_move_mask(*sq);

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_rook_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];
    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb = get_horizontal_move_mask(*sq) | get_vertical_move_mask(*sq);

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_queen_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];
    let squares = components::square::SQUARES;

    for sq in squares {
        let mut bb = get_horizontal_move_mask(*sq)
            | get_vertical_move_mask(*sq)
            | get_occupancy_mask_bishop(*sq);

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_offset()] = bb;
    }
    retval
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
fn populate_intervening_bitboard_array() -> [[u64; board::NUM_SQUARES]; board::NUM_SQUARES] {
    const M1: u64 = 0xffff_ffff_ffff_ffff;
    const A2A7: u64 = 0x0001_0101_0101_0100;
    const B2G7: u64 = 0x0040_2010_0804_0200;
    const H1B7: u64 = 0x0002_0408_1020_4080;

    let mut retval = [[0; board::NUM_SQUARES]; board::NUM_SQUARES];

    let squares = components::square::SQUARES;

    for sq1 in squares {
        for sq2 in squares {
            let btwn = (M1.shl(*sq1 as u8)) ^ (M1.shl(*sq2 as u8));
            let file = (*sq2 as u64 & 7).wrapping_sub(*sq1 as u64 & 7);
            let rank = ((*sq2 as u64 | 7).wrapping_sub(*sq1 as u64)) >> 3;
            let mut line = ((file & 7).wrapping_sub(1)) & A2A7; /* a2a7 if same file */
            line = line.wrapping_add((((rank & 7).wrapping_sub(1)) >> 58).wrapping_mul(2)); /* b1g1 if same rank */
            line = line.wrapping_add((((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & B2G7); /* b2g7 if same diagonal */
            line = line.wrapping_add((((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & H1B7); /* h1b7 if same antidiag */
            line = line.wrapping_mul(btwn & (btwn.wrapping_neg())); /* mul acts like shift by smaller square */
            let val = line & btwn; /* return the bits on that line in-between */

            retval[sq1.to_offset()][sq2.to_offset()] = val;
        }
    }

    return retval;
}

fn set_dest_sq_if_valid(bb: &mut u64, sq: Square, rank_offset: i8, file_offset: i8) {
    let dest_rank: i8 = sq.rank() as i8 + rank_offset;
    let dest_file: i8 = sq.file() as i8 + file_offset;

    if is_valid_rank(dest_rank) && is_valid_file(dest_file) {
        let new_sq = Square::derive_relative_square(sq, rank_offset, file_offset);
        if let Some(_) = new_sq {
            *bb = bitboard::set_bit(*bb, new_sq.unwrap());
        }
    }
}

fn is_valid_rank(r: i8) -> bool {
    r >= Rank::Rank1 as i8 && r <= Rank::Rank8 as i8
}

fn is_valid_file(f: i8) -> bool {
    f >= File::FileA as i8 && f <= File::FileH as i8
}

#[cfg(test)]
pub mod tests {
    use components::bitboard;
    use components::occupancy_masks;
    use components::square::Square;

    #[test]
    pub fn diagonal_occupancy_masks() {
        let bb = occupancy_masks::get_diagonal_move_mask(Square::c1);

        assert!(bitboard::is_set(bb, Square::d2));
        assert!(bitboard::is_set(bb, Square::e3));
        assert!(bitboard::is_set(bb, Square::f4));
        assert!(bitboard::is_set(bb, Square::g5));
        assert!(bitboard::is_set(bb, Square::h6));
    }
}
