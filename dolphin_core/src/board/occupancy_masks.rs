use crate::board::bitboard;
use crate::board::file::File;
use crate::board::game_board;
use crate::board::rank::Rank;
use crate::board::square;
use crate::board::square::*;

use std::ops::Shl;

use super::types::ToInt;

// Bitboards representing commonly used ranks
pub const RANK_2_BB: u64 = 0x0000_0000_0000_FF00;
pub const RANK_7_BB: u64 = 0x00FF_0000_0000_0000;

const RANK_MASK: u64 = 0x0000_0000_0000_00ff;
const FILE_MASK: u64 = 0x0101_0101_0101_0101;

// bitboards for squares between castle squares (eg White King side = f1 and g1)
pub const CASTLE_MASK_WK: u64 = 0x0000_0000_0000_0060;
pub const CASTLE_MASK_WQ: u64 = 0x0000_0000_0000_000E;
pub const CASTLE_MASK_BK: u64 = 0x6000_0000_0000_0000;
pub const CASTLE_MASK_BQ: u64 = 0x0E00_0000_0000_0000;

const FILE_A_BB: u64 = FILE_MASK;
const FILE_H_BB: u64 = FILE_A_BB << 7;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct DiagonalAntidiagonal {
    diag_mask: u64,
    anti_diag_mask: u64,
}
impl DiagonalAntidiagonal {
    pub fn get_diag_mask(&self) -> u64 {
        self.diag_mask
    }
    pub fn get_anti_diag_mask(&self) -> u64 {
        self.anti_diag_mask
    }
}

impl Default for OccupancyMasks {
    fn default() -> Self {
        OccupancyMasks {
            knight: [0; game_board::NUM_SQUARES],
            diagonal: [DiagonalAntidiagonal::default(); game_board::NUM_SQUARES],
            bishop: [0; game_board::NUM_SQUARES],
            queen: [0; game_board::NUM_SQUARES],
            king: [0; game_board::NUM_SQUARES],
            in_between: [[0; game_board::NUM_SQUARES]; game_board::NUM_SQUARES],
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct OccupancyMasks {
    knight: [u64; game_board::NUM_SQUARES],
    diagonal: [DiagonalAntidiagonal; game_board::NUM_SQUARES],
    bishop: [u64; game_board::NUM_SQUARES],
    queen: [u64; game_board::NUM_SQUARES],
    king: [u64; game_board::NUM_SQUARES],
    in_between: [[u64; game_board::NUM_SQUARES]; game_board::NUM_SQUARES],
}

impl OccupancyMasks {
    pub fn new() -> Box<OccupancyMasks> {
        let knight = populate_knight_occupancy_mask_array();
        let diagonal = populate_diagonal_mask_array();
        let bishop = populate_bishop_mask_array(&diagonal);
        let queen = populate_queen_mask_array(&diagonal);
        let king = populate_king_mask_array();
        let in_between = populate_intervening_bitboard_array();

        Box::new(OccupancyMasks {
            knight,
            diagonal,
            bishop,
            queen,
            king,
            in_between,
        })
    }

    pub fn get_occupancy_mask_bishop(&self, sq: Square) -> u64 {
        *self.bishop.get(sq.to_usize()).unwrap()
    }

    pub fn get_occupancy_mask_knight(&self, sq: Square) -> u64 {
        *self.knight.get(sq.to_usize()).unwrap()
    }

    pub fn get_occupancy_mask_rook(&self, sq: Square) -> u64 {
        get_horizontal_move_mask(sq) | get_vertical_move_mask(sq)
    }

    pub fn get_occupancy_mask_queen(&self, sq: Square) -> u64 {
        *self.queen.get(sq.to_usize()).unwrap()
    }

    pub fn get_occupancy_mask_king(&self, sq: Square) -> u64 {
        *self.king.get(sq.to_usize()).unwrap()
    }

    pub fn get_inbetween_squares(&self, sq1: Square, sq2: Square) -> u64 {
        self.in_between[sq1.to_usize()][sq2.to_usize()]
    }

    pub fn get_horizontal_mask(&self, sq: Square) -> u64 {
        get_horizontal_move_mask(sq)
    }

    pub fn get_vertical_mask(&self, sq: Square) -> u64 {
        get_vertical_move_mask(sq)
    }

    pub fn get_diag_antidiag_mask(&self, sq: Square) -> &DiagonalAntidiagonal {
        self.diagonal.get(sq.to_usize()).unwrap()
    }

    pub fn get_occ_mask_white_pawns_double_move_mask(&self, sq: Square) -> u64 {
        let mut bb = sq.get_square_as_bb();
        bb = self.north(bb);
        bb |= self.north(bb);
        bb
    }
    pub fn get_occ_mask_black_pawns_double_move_mask(&self, sq: Square) -> u64 {
        let mut bb = sq.get_square_as_bb();
        bb = self.south(bb);
        bb |= self.south(bb);
        bb
    }

    pub fn get_occ_mask_white_pawns_attacking_sq(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
    pub fn get_occ_mask_black_pawns_attacking_sq(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    pub fn get_occ_mask_white_pawn_capture_non_first_double_move(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    pub fn get_occ_mask_black_pawn_capture_non_first_double_move(&self, sq: Square) -> u64 {
        let bb = sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
    pub fn get_occ_mask_white_pawn_attack_squares(&self, pawn_sq: Square) -> u64 {
        let bb = pawn_sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    pub fn get_occ_mask_black_pawn_attack_squares(&self, pawn_sq: Square) -> u64 {
        let bb = pawn_sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }

    const fn north_east(&self, bb: u64) -> u64 {
        (bb & !FILE_H_BB) << 9
    }

    const fn south_east(&self, bb: u64) -> u64 {
        (bb & !FILE_H_BB) >> 7
    }

    const fn south(&self, bb: u64) -> u64 {
        bb >> 8
    }

    const fn north(&self, bb: u64) -> u64 {
        bb << 8
    }

    const fn north_west(&self, bb: u64) -> u64 {
        (bb & !FILE_A_BB) << 7
    }

    const fn south_west(&self, bb: u64) -> u64 {
        (bb & !FILE_A_BB) >> 9
    }
}

fn get_vertical_move_mask(sq: Square) -> u64 {
    let file = sq.file();
    FILE_MASK << file.to_usize()
}

fn get_horizontal_move_mask(sq: Square) -> u64 {
    let rank = sq.rank();
    RANK_MASK << (rank.to_usize() << 3)
}

fn populate_knight_occupancy_mask_array() -> [u64; game_board::NUM_SQUARES] {
    let mut retval: [u64; game_board::NUM_SQUARES] = [0; game_board::NUM_SQUARES];

    let squares = square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        let rank = sq.rank();
        let file = sq.file();

        // rank + 2, file +/- 1
        let mut r = rank.add_two();
        let mut f = file.add_one();

        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.subtract_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        // rank + 1, file +/- 2
        r = rank.add_one();
        f = file.add_two();
        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.subtract_two();
        set_bb_if_sq_valid(r, f, &mut bb);

        // rank - 1, file +/- 2
        r = rank.subtract_one();
        f = file.add_two();
        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.subtract_two();
        set_bb_if_sq_valid(r, f, &mut bb);

        // rank - 2, file +/- 1
        r = rank.subtract_two();
        f = file.add_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.subtract_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        retval[sq.to_usize()] = bb;
    }
    retval
}

fn set_bb_if_sq_valid(rank: Option<Rank>, file: Option<File>, bb: &mut u64) {
    if let Some(r) = rank {
        if let Some(f) = file {
            let derived_sq = Square::from_rank_file(r, f);
            *bb = bitboard::set_bit(*bb, derived_sq);
        }
    }
}

fn populate_king_mask_array() -> [u64; game_board::NUM_SQUARES] {
    let mut retval: [u64; game_board::NUM_SQUARES] = [0; game_board::NUM_SQUARES];

    let squares = square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        let rank = sq.rank();
        let file = sq.file();

        // rank+1, file -1/0/+1
        let mut r = rank.add_one();
        let mut f = file.subtract_one();
        set_bb_if_sq_valid(r, f, &mut bb);
        f = Some(file);
        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.add_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        // rank, file -1/+1
        r = Some(rank);
        f = file.subtract_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.add_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        // rank-1, file -1/0/+1
        r = rank.subtract_one();
        f = file.subtract_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        f = Some(file);
        set_bb_if_sq_valid(r, f, &mut bb);

        f = file.add_one();
        set_bb_if_sq_valid(r, f, &mut bb);

        retval[sq.to_usize()] = bb;
    }
    retval
}

fn is_valid_rank_and_file(rank: Option<Rank>, file: Option<File>) -> bool {
    rank.is_some() && file.is_some()
}

fn populate_diagonal_mask_array() -> [DiagonalAntidiagonal; game_board::NUM_SQUARES] {
    let mut retval: [DiagonalAntidiagonal; game_board::NUM_SQUARES] =
        [DiagonalAntidiagonal::default(); game_board::NUM_SQUARES];

    for sq in square::SQUARES.iter() {
        let mut bb: u64 = 0;
        let mut rank = sq.rank();
        let mut file = sq.file();

        // move SW
        loop {
            let r = rank.subtract_one();
            let f = file.subtract_one();

            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }

            // if r.is_some() && f.is_some() {
            //     let derived_sq = Square::get_square(r.unwrap(), f.unwrap());
            //     bb = bitboard::set_bit(bb, derived_sq);

            //     rank = r.unwrap();
            //     file = f.unwrap();
            // } else {
            //     break;
            // }
        }

        // move NE
        rank = sq.rank();
        file = sq.file();

        loop {
            let r = rank.add_one();
            let f = file.add_one();
            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }
        }

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_usize()].diag_mask = bb;
    }

    for sq in square::SQUARES.iter() {
        let mut bb: u64 = 0;

        let mut rank = sq.rank();
        let mut file = sq.file();

        // move NW
        loop {
            let r = rank.add_one();
            let f = file.subtract_one();
            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
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
            let r = rank.subtract_one();
            let f = file.add_one();

            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
                bb = bitboard::set_bit(bb, derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }
        }

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_usize()].anti_diag_mask = bb;
    }

    retval
}

fn populate_bishop_mask_array(
    diag_masks: &[DiagonalAntidiagonal; game_board::NUM_SQUARES],
) -> [u64; game_board::NUM_SQUARES] {
    let mut retval: [u64; game_board::NUM_SQUARES] = [0; game_board::NUM_SQUARES];
    let squares = square::SQUARES;

    for sq in squares {
        let mut bb = diag_masks[sq.to_usize()].diag_mask | diag_masks[sq.to_usize()].anti_diag_mask;

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_usize()] = bb;
    }
    retval
}

fn populate_queen_mask_array(
    diag_masks: &[DiagonalAntidiagonal; game_board::NUM_SQUARES],
) -> [u64; game_board::NUM_SQUARES] {
    let mut retval: [u64; game_board::NUM_SQUARES] = [0; game_board::NUM_SQUARES];
    let squares = square::SQUARES;

    for sq in squares {
        let mut bb = get_horizontal_move_mask(*sq)
            | get_vertical_move_mask(*sq)
            | diag_masks[sq.to_usize()].diag_mask
            | diag_masks[sq.to_usize()].anti_diag_mask;

        // remove current square
        bb = bitboard::clear_bit(bb, *sq);

        retval[sq.to_usize()] = bb;
    }
    retval
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
fn populate_intervening_bitboard_array() -> [[u64; game_board::NUM_SQUARES]; game_board::NUM_SQUARES]
{
    const M1: u64 = 0xffff_ffff_ffff_ffff;
    const A2A7: u64 = 0x0001_0101_0101_0100;
    const B2G7: u64 = 0x0040_2010_0804_0200;
    const H1B7: u64 = 0x0002_0408_1020_4080;

    let mut retval = [[0; game_board::NUM_SQUARES]; game_board::NUM_SQUARES];

    let squares = square::SQUARES;

    for sq1 in squares {
        for sq2 in squares {
            let btwn = (M1.shl(sq1.to_usize() as u8)) ^ (M1.shl(sq2.to_usize() as u8));
            let file = (sq2.to_usize() as u64 & 7).wrapping_sub(sq1.to_usize() as u64 & 7);
            let rank = ((sq2.to_usize() as u64 | 7).wrapping_sub(sq1.to_usize() as u64)) >> 3;
            let mut line = ((file & 7).wrapping_sub(1)) & A2A7; /* a2a7 if same file */
            line = line.wrapping_add((((rank & 7).wrapping_sub(1)) >> 58).wrapping_mul(2)); /* b1g1 if same rank */
            line = line.wrapping_add((((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & B2G7); /* b2g7 if same diagonal */
            line = line.wrapping_add((((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & H1B7); /* h1b7 if same antidiag */
            line = line.wrapping_mul(btwn & (btwn.wrapping_neg())); /* mul acts like shift by smaller square */
            let val = line & btwn; /* return the bits on that line in-between */

            retval[sq1.to_usize()][sq2.to_usize()] = val;
        }
    }

    retval
}

#[cfg(test)]
pub mod tests {
    use super::OccupancyMasks;
    use crate::board::bitboard;
    use crate::board::square::*;

    #[test]
    pub fn white_double_first_move_mask() {
        let masks = OccupancyMasks::new();

        let mut bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_A2);
        assert!(bitboard::is_set(bb, SQUARE_A3));
        assert!(bitboard::is_set(bb, SQUARE_A4));
        assert!(bitboard::is_clear(bb, SQUARE_A2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_B2);
        assert!(bitboard::is_set(bb, SQUARE_B3));
        assert!(bitboard::is_set(bb, SQUARE_B4));
        assert!(bitboard::is_clear(bb, SQUARE_B2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_C2);
        assert!(bitboard::is_set(bb, SQUARE_C3));
        assert!(bitboard::is_set(bb, SQUARE_C4));
        assert!(bitboard::is_clear(bb, SQUARE_C2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_D2);
        assert!(bitboard::is_set(bb, SQUARE_D3));
        assert!(bitboard::is_set(bb, SQUARE_D4));
        assert!(bitboard::is_clear(bb, SQUARE_D2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_E2);
        assert!(bitboard::is_set(bb, SQUARE_E3));
        assert!(bitboard::is_set(bb, SQUARE_E4));
        assert!(bitboard::is_clear(bb, SQUARE_E2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_F2);
        assert!(bitboard::is_set(bb, SQUARE_F3));
        assert!(bitboard::is_set(bb, SQUARE_F4));
        assert!(bitboard::is_clear(bb, SQUARE_F2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_G2);
        assert!(bitboard::is_set(bb, SQUARE_G3));
        assert!(bitboard::is_set(bb, SQUARE_G4));
        assert!(bitboard::is_clear(bb, SQUARE_G2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_H2);
        assert!(bitboard::is_set(bb, SQUARE_H3));
        assert!(bitboard::is_set(bb, SQUARE_H4));
        assert!(bitboard::is_clear(bb, SQUARE_H2));
    }

    #[test]
    pub fn black_double_first_move_mask() {
        let masks = OccupancyMasks::new();

        let mut bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_A7);
        assert!(bitboard::is_set(bb, SQUARE_A6));
        assert!(bitboard::is_set(bb, SQUARE_A5));
        assert!(bitboard::is_clear(bb, SQUARE_A7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_B7);
        assert!(bitboard::is_set(bb, SQUARE_B6));
        assert!(bitboard::is_set(bb, SQUARE_B5));
        assert!(bitboard::is_clear(bb, SQUARE_B7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_C7);
        assert!(bitboard::is_set(bb, SQUARE_C6));
        assert!(bitboard::is_set(bb, SQUARE_C5));
        assert!(bitboard::is_clear(bb, SQUARE_C7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_D7);
        assert!(bitboard::is_set(bb, SQUARE_D6));
        assert!(bitboard::is_set(bb, SQUARE_D5));
        assert!(bitboard::is_clear(bb, SQUARE_D7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_E7);
        assert!(bitboard::is_set(bb, SQUARE_E6));
        assert!(bitboard::is_set(bb, SQUARE_E5));
        assert!(bitboard::is_clear(bb, SQUARE_E7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_F7);
        assert!(bitboard::is_set(bb, SQUARE_F6));
        assert!(bitboard::is_set(bb, SQUARE_F5));
        assert!(bitboard::is_clear(bb, SQUARE_F7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_G7);
        assert!(bitboard::is_set(bb, SQUARE_G6));
        assert!(bitboard::is_set(bb, SQUARE_G5));
        assert!(bitboard::is_clear(bb, SQUARE_G7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_H7);
        assert!(bitboard::is_set(bb, SQUARE_H6));
        assert!(bitboard::is_set(bb, SQUARE_H5));
        assert!(bitboard::is_clear(bb, SQUARE_H7));
    }
}
