use crate::bitboard;
use crate::board;
use crate::square;
use crate::square::{File, Rank, Square};
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

pub struct OccupancyMasks {
    knight: [u64; board::NUM_SQUARES],
    diagonal: [u64; board::NUM_SQUARES],
    anti_diagonal: [u64; board::NUM_SQUARES],
    bishop: [u64; board::NUM_SQUARES],
    rook: [u64; board::NUM_SQUARES],
    queen: [u64; board::NUM_SQUARES],
    king: [u64; board::NUM_SQUARES],
    in_between: [[u64; board::NUM_SQUARES]; board::NUM_SQUARES],
}

impl OccupancyMasks {
    pub fn new() -> OccupancyMasks {
        let knight = populate_knight_occupancy_mask_array();
        let diagonal = populate_diagonal_mask_array();
        let anti_diagonal = populate_antidiagonal_mask_array();
        let bishop = populate_bishop_mask_array();
        let rook = populate_rook_mask_array();
        let queen = populate_queen_mask_array();
        let king = populate_king_mask_array();
        let in_between = populate_intervening_bitboard_array();

        OccupancyMasks {
            knight,
            diagonal,
            anti_diagonal,
            bishop,
            rook,
            queen,
            king,
            in_between,
        }
    }

    pub fn get_occupancy_mask_bishop(&self, sq: Square) -> u64 {
        *self.bishop.get(sq.to_offset()).unwrap()
    }

    pub fn get_occupancy_mask_knight(&self, sq: Square) -> u64 {
        *self.knight.get(sq.to_offset()).unwrap()
    }

    pub fn get_occupancy_mask_rook(&self, sq: Square) -> u64 {
        *self.rook.get(sq.to_offset()).unwrap()
    }

    pub fn get_occupancy_mask_queen(&self, sq: Square) -> u64 {
        *self.queen.get(sq.to_offset()).unwrap()
    }

    pub fn get_occupancy_mask_king(&self, sq: Square) -> u64 {
        *self.king.get(sq.to_offset()).unwrap()
    }

    pub fn get_inbetween_squares(&self, sq1: Square, sq2: Square) -> u64 {
        self.in_between[sq1.to_offset()][sq2.to_offset()]
    }

    pub fn get_vertical_move_mask(&self, sq: Square) -> u64 {
        return get_vertical_move_mask(sq);
    }

    pub fn get_horizontal_move_mask(&self, sq: Square) -> u64 {
        return get_horizontal_move_mask(sq);
    }

    pub fn get_diagonal_move_mask(&self, sq: Square) -> u64 {
        *self.diagonal.get(sq.to_offset()).unwrap()
    }
    pub fn get_anti_diagonal_move_mask(&self, sq: Square) -> u64 {
        *self.anti_diagonal.get(sq.to_offset()).unwrap()
    }

    #[inline(always)]
    const fn north_east(&self, bb: u64) -> u64 {
        (bb & !FILE_H_BB) << 9
    }

    #[inline(always)]
    const fn south_east(&self, bb: u64) -> u64 {
        (bb & !FILE_H_BB) >> 7
    }

    #[inline(always)]
    const fn north_west(&self, bb: u64) -> u64 {
        (bb & !FILE_A_BB) << 7
    }

    #[inline(always)]
    const fn south_west(&self, bb: u64) -> u64 {
        (bb & !FILE_A_BB) >> 9
    }

    #[inline(always)]
    pub fn get_occ_mask_white_pawns_attacking_sq(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
    #[inline(always)]
    pub fn get_occ_mask_black_pawns_attacking_sq(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    #[inline(always)]
    pub fn get_occ_mask_white_pawn_capture_non_first_double_move(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    #[inline(always)]
    pub fn get_occ_mask_black_pawn_capture_non_first_double_move(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
    #[inline(always)]
    pub fn get_occ_mask_white_pawn_attack_squares(&self, pawn_sq: Square) -> u64 {
        let bb = pawn_sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    #[inline(always)]
    pub fn get_occ_mask_black_pawn_attack_squares(&self, pawn_sq: Square) -> u64 {
        let bb = pawn_sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
}

fn get_vertical_move_mask(sq: Square) -> u64 {
    let file = sq.file();
    return FILE_MASK << (file as u8);
}

fn get_horizontal_move_mask(sq: Square) -> u64 {
    let rank = sq.rank();
    return RANK_MASK << ((rank as u8) << 3);
}

fn populate_knight_occupancy_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    let squares = square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        let rank = sq.rank();
        let file = sq.file();

        // rank + 2, file +/- 1
        let mut r = Rank::add_two(rank);
        let mut f = File::add_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::subtract_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        // rank + 1, file +/- 2
        r = Rank::add_one(rank);
        f = File::add_two(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::subtract_two(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        // rank - 1, file +/- 2
        r = Rank::subtract_one(rank);
        f = File::add_two(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::subtract_two(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        // rank - 2, file +/- 1
        r = Rank::subtract_two(rank);
        f = File::add_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::subtract_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_king_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    let squares = square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        let rank = sq.rank();
        let file = sq.file();

        // rank+1, file -1/0/+1
        let mut r = Rank::add_one(rank);
        let mut f = File::subtract_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = Some(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::add_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        // rank, file -1/+1
        r = Some(rank);
        f = File::subtract_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::add_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        // rank-1, file -1/0/+1
        r = Rank::subtract_one(rank);
        f = File::subtract_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = Some(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }
        f = File::add_one(file);
        if r.is_some() && f.is_some() {
            let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            bb = bitboard::set_bit(bb, derived_sq);
        }

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_diagonal_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];

    for sq in square::SQUARES.iter() {
        let mut bb: u64 = 0;
        let mut rank = sq.rank();
        let mut file = sq.file();

        // move SW
        loop {
            let r = Rank::subtract_one(rank);
            let f = File::subtract_one(file);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }
        }

        // move NE
        rank = sq.rank();
        file = sq.file();

        loop {
            let r = Rank::add_one(rank);
            let f = File::add_one(file);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
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

    let squares = square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        let mut rank = sq.rank();
        let mut file = sq.file();

        // move NW
        loop {
            let r = Rank::add_one(rank);
            let f = File::subtract_one(file);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);
                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }
        }

        // move SE
        rank = sq.rank();
        file = sq.file();

        loop {
            let r = Rank::subtract_one(rank);
            let f = File::add_one(file);
            if r.is_some() && f.is_some() {
                let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);
                rank = r.unwrap();
                file = f.unwrap();
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
    let squares = square::SQUARES;

    let diag_masks = populate_diagonal_mask_array();
    let anti_diag_masks = populate_antidiagonal_mask_array();
    for sq in squares {
        let mut bb = diag_masks[*sq as usize] | anti_diag_masks[*sq as usize];

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_offset()] = bb;
    }
    retval
}

fn populate_rook_mask_array() -> [u64; board::NUM_SQUARES] {
    let mut retval: [u64; board::NUM_SQUARES] = [0; board::NUM_SQUARES];
    let squares = square::SQUARES;

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
    let squares = square::SQUARES;

    let diag_masks = populate_diagonal_mask_array();
    let anti_diag_masks = populate_antidiagonal_mask_array();

    for sq in squares {
        let mut bb = get_horizontal_move_mask(*sq)
            | get_vertical_move_mask(*sq)
            | diag_masks.get(*sq as usize).unwrap()
            | anti_diag_masks.get(*sq as usize).unwrap();

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

    let squares = square::SQUARES;

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

#[cfg(test)]
pub mod tests {
    use super::OccupancyMasks;
    use crate::bitboard;

    use crate::square::Square;

    #[test]
    pub fn diagonal_occupancy_masks() {
        let masks = OccupancyMasks::new();

        let bb = masks.get_diagonal_move_mask(Square::c1);

        assert!(bitboard::is_set(bb, Square::d2));
        assert!(bitboard::is_set(bb, Square::e3));
        assert!(bitboard::is_set(bb, Square::f4));
        assert!(bitboard::is_set(bb, Square::g5));
        assert!(bitboard::is_set(bb, Square::h6));
    }
}
