use super::types::ToInt;
use crate::board::bitboard::Bitboard;
use crate::board::file::File;
use crate::board::game_board;
use crate::board::rank::Rank;
use crate::board::square;
use crate::board::square::*;
use std::ops::Shl;

// Bitboards representing commonly used ranks
pub const RANK_2_BB: Bitboard = Bitboard::new(0x0000_0000_0000_FF00);
pub const RANK_7_BB: Bitboard = Bitboard::new(0x00FF_0000_0000_0000);

const RANK_MASK: Bitboard = Bitboard::new(0x0000_0000_0000_00ff);
const FILE_MASK: Bitboard = Bitboard::new(0x0101_0101_0101_0101);

// bitboards for squares between castle squares (eg White King side = f1 and g1)
pub const CASTLE_MASK_WK: Bitboard = Bitboard::new(0x0000_0000_0000_0060);
pub const CASTLE_MASK_WQ: Bitboard = Bitboard::new(0x0000_0000_0000_000E);
pub const CASTLE_MASK_BK: Bitboard = Bitboard::new(0x6000_0000_0000_0000);
pub const CASTLE_MASK_BQ: Bitboard = Bitboard::new(0x0E00_0000_0000_0000);

const FILE_A_BB: Bitboard = FILE_MASK;
const FILE_H_BB: Bitboard = Bitboard::new(0x8080_8080_8080_8080);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct DiagonalAntidiagonal {
    diag_mask: Bitboard,
    anti_diag_mask: Bitboard,
}
impl DiagonalAntidiagonal {
    pub fn get_diag_mask(&self) -> Bitboard {
        self.diag_mask
    }
    pub fn get_anti_diag_mask(&self) -> Bitboard {
        self.anti_diag_mask
    }
}

impl Default for OccupancyMasks {
    fn default() -> Self {
        OccupancyMasks {
            knight: [Bitboard::default(); game_board::NUM_SQUARES],
            diagonal: [DiagonalAntidiagonal::default(); game_board::NUM_SQUARES],
            bishop: [Bitboard::default(); game_board::NUM_SQUARES],
            queen: [Bitboard::default(); game_board::NUM_SQUARES],
            king: [Bitboard::default(); game_board::NUM_SQUARES],
            in_between: [[Bitboard::default(); game_board::NUM_SQUARES]; game_board::NUM_SQUARES],
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct OccupancyMasks {
    knight: [Bitboard; game_board::NUM_SQUARES],
    diagonal: [DiagonalAntidiagonal; game_board::NUM_SQUARES],
    bishop: [Bitboard; game_board::NUM_SQUARES],
    queen: [Bitboard; game_board::NUM_SQUARES],
    king: [Bitboard; game_board::NUM_SQUARES],
    in_between: [[Bitboard; game_board::NUM_SQUARES]; game_board::NUM_SQUARES],
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

    pub fn get_occupancy_mask_bishop(&self, sq: Square) -> Bitboard {
        *self.bishop.get(sq.to_usize()).unwrap()
    }

    pub fn get_occupancy_mask_knight(&self, sq: Square) -> Bitboard {
        *self.knight.get(sq.to_usize()).unwrap()
    }

    pub fn get_occupancy_mask_rook(&self, sq: Square) -> Bitboard {
        get_horizontal_move_mask(sq) | get_vertical_move_mask(sq)
    }

    pub fn get_occupancy_mask_queen(&self, sq: Square) -> Bitboard {
        *self.queen.get(sq.to_usize()).unwrap()
    }

    pub fn get_occupancy_mask_king(&self, sq: Square) -> Bitboard {
        *self.king.get(sq.to_usize()).unwrap()
    }

    pub fn get_inbetween_squares(&self, sq1: Square, sq2: Square) -> Bitboard {
        self.in_between[sq1.to_usize()][sq2.to_usize()]
    }

    pub fn get_horizontal_mask(&self, sq: Square) -> Bitboard {
        get_horizontal_move_mask(sq)
    }

    pub fn get_vertical_mask(&self, sq: Square) -> Bitboard {
        get_vertical_move_mask(sq)
    }

    pub fn get_diag_antidiag_mask(&self, sq: Square) -> &DiagonalAntidiagonal {
        self.diagonal.get(sq.to_usize()).unwrap()
    }

    pub fn get_occ_mask_white_pawns_double_move_mask(&self, sq: Square) -> Bitboard {
        let mut bb = sq.get_square_as_bb();
        bb = self.north(bb);
        bb |= self.north(bb);
        bb
    }
    pub fn get_occ_mask_black_pawns_double_move_mask(&self, sq: Square) -> Bitboard {
        let mut bb = sq.get_square_as_bb();
        bb = self.south(bb);
        bb |= self.south(bb);
        bb
    }

    pub fn get_occ_mask_white_pawns_attacking_sq(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
    pub fn get_occ_mask_black_pawns_attacking_sq(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    pub fn get_occ_mask_white_pawn_capture_non_first_double_move(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    pub fn get_occ_mask_black_pawn_capture_non_first_double_move(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }
    pub fn get_occ_mask_white_pawn_attack_squares(&self, pawn_sq: Square) -> Bitboard {
        let bb = pawn_sq.get_square_as_bb();
        self.north_east(bb) | self.north_west(bb)
    }
    pub fn get_occ_mask_black_pawn_attack_squares(&self, pawn_sq: Square) -> Bitboard {
        let bb = pawn_sq.get_square_as_bb();
        self.south_east(bb) | self.south_west(bb)
    }

    fn north_east(&self, bb: Bitboard) -> Bitboard {
        (bb & !FILE_H_BB) << 9
    }

    fn south_east(&self, bb: Bitboard) -> Bitboard {
        (bb & !FILE_H_BB) >> 7
    }

    fn south(&self, bb: Bitboard) -> Bitboard {
        bb >> 8
    }

    fn north(&self, bb: Bitboard) -> Bitboard {
        bb << 8
    }

    fn north_west(&self, bb: Bitboard) -> Bitboard {
        (bb & !FILE_A_BB) << 7
    }

    fn south_west(&self, bb: Bitboard) -> Bitboard {
        (bb & !FILE_A_BB) >> 9
    }
}

fn get_vertical_move_mask(sq: Square) -> Bitboard {
    let file = sq.file();
    FILE_MASK << file.to_u8()
}

fn get_horizontal_move_mask(sq: Square) -> Bitboard {
    let rank = sq.rank();
    RANK_MASK << (rank.to_u8() << 3)
}

fn populate_knight_occupancy_mask_array() -> [Bitboard; game_board::NUM_SQUARES] {
    let mut retval: [Bitboard; game_board::NUM_SQUARES] =
        [Bitboard::default(); game_board::NUM_SQUARES];

    for sq in square::iterator() {
        let mut bb = Bitboard::new(0);

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

fn set_bb_if_sq_valid(rank: Option<Rank>, file: Option<File>, bb: &mut Bitboard) {
    if let Some(r) = rank {
        if let Some(f) = file {
            let derived_sq = Square::from_rank_file(r, f);
            bb.set_bit(derived_sq);
        }
    }
}

fn populate_king_mask_array() -> [Bitboard; game_board::NUM_SQUARES] {
    let mut retval: [Bitboard; game_board::NUM_SQUARES] =
        [Bitboard::default(); game_board::NUM_SQUARES];

    for sq in square::iterator() {
        let mut bb = Bitboard::new(0);

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

    for sq in square::iterator() {
        let mut bb = Bitboard::new(0);
        let mut rank = sq.rank();
        let mut file = sq.file();

        // move SW
        loop {
            let r = rank.subtract_one();
            let f = file.subtract_one();

            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
                bb.set_bit(derived_sq);

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
            let r = rank.add_one();
            let f = file.add_one();
            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
                bb.set_bit(derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }
        }

        // remove current square
        bb.clear_bit(*sq);

        retval[sq.to_usize()].diag_mask = bb;
    }

    for sq in square::iterator() {
        let mut bb = Bitboard::new(0);

        let mut rank = sq.rank();
        let mut file = sq.file();

        // move NW
        loop {
            let r = rank.add_one();
            let f = file.subtract_one();
            if is_valid_rank_and_file(r, f) {
                let derived_sq = Square::from_rank_file(r.unwrap(), f.unwrap());
                bb.set_bit(derived_sq);

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
                bb.set_bit(derived_sq);

                rank = r.unwrap();
                file = f.unwrap();
            } else {
                break;
            }
        }

        // remove current square
        bb.clear_bit(*sq);

        retval[sq.to_usize()].anti_diag_mask = bb;
    }

    retval
}

fn populate_bishop_mask_array(
    diag_masks: &[DiagonalAntidiagonal; game_board::NUM_SQUARES],
) -> [Bitboard; game_board::NUM_SQUARES] {
    let mut retval: [Bitboard; game_board::NUM_SQUARES] =
        [Bitboard::default(); game_board::NUM_SQUARES];

    for sq in square::iterator() {
        let mut bb = diag_masks[sq.to_usize()].diag_mask | diag_masks[sq.to_usize()].anti_diag_mask;

        // remove current square
        bb.clear_bit(*sq);

        retval[sq.to_usize()] = bb;
    }
    retval
}

fn populate_queen_mask_array(
    diag_masks: &[DiagonalAntidiagonal; game_board::NUM_SQUARES],
) -> [Bitboard; game_board::NUM_SQUARES] {
    let mut retval: [Bitboard; game_board::NUM_SQUARES] =
        [Bitboard::default(); game_board::NUM_SQUARES];

    for sq in square::iterator() {
        let mut bb = get_horizontal_move_mask(*sq)
            | get_vertical_move_mask(*sq)
            | diag_masks[sq.to_usize()].diag_mask
            | diag_masks[sq.to_usize()].anti_diag_mask;

        // remove current square
        bb.clear_bit(*sq);

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
fn populate_intervening_bitboard_array(
) -> [[Bitboard; game_board::NUM_SQUARES]; game_board::NUM_SQUARES] {
    const M1: u64 = 0xffff_ffff_ffff_ffff;
    const A2A7: u64 = 0x0001_0101_0101_0100;
    const B2G7: u64 = 0x0040_2010_0804_0200;
    const H1B7: u64 = 0x0002_0408_1020_4080;

    let mut retval = [[Bitboard::default(); game_board::NUM_SQUARES]; game_board::NUM_SQUARES];

    for sq1 in square::iterator() {
        for sq2 in square::iterator() {
            let btwn = (M1.shl(sq1.to_usize() as u8)) ^ (M1.shl(sq2.to_usize() as u8));
            let file = (sq2.to_usize() as u64 & 7).wrapping_sub(sq1.to_usize() as u64 & 7);
            let rank = ((sq2.to_usize() as u64 | 7).wrapping_sub(sq1.to_usize() as u64)) >> 3;
            let mut line = ((file & 7).wrapping_sub(1)) & A2A7; /* a2a7 if same file */
            line = line.wrapping_add((((rank & 7).wrapping_sub(1)) >> 58).wrapping_mul(2)); /* b1g1 if same rank */
            line = line.wrapping_add((((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & B2G7); /* b2g7 if same diagonal */
            line = line.wrapping_add((((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & H1B7); /* h1b7 if same antidiag */
            line = line.wrapping_mul(btwn & (btwn.wrapping_neg())); /* mul acts like shift by smaller square */
            let val = line & btwn; /* return the bits on that line in-between */

            retval[sq1.to_usize()][sq2.to_usize()] = Bitboard::new(val);
        }
    }

    retval
}

#[cfg(test)]
pub mod tests {
    use super::OccupancyMasks;
    use crate::board::square::*;

    #[test]
    pub fn white_double_first_move_mask() {
        let masks = OccupancyMasks::new();

        let mut bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_A2);
        assert!(bb.is_set(SQUARE_A3));
        assert!(bb.is_set(SQUARE_A4));
        assert!(bb.is_clear(SQUARE_A2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_B2);
        assert!(bb.is_set(SQUARE_B3));
        assert!(bb.is_set(SQUARE_B4));
        assert!(bb.is_clear(SQUARE_B2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_C2);
        assert!(bb.is_set(SQUARE_C3));
        assert!(bb.is_set(SQUARE_C4));
        assert!(bb.is_clear(SQUARE_C2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_D2);
        assert!(bb.is_set(SQUARE_D3));
        assert!(bb.is_set(SQUARE_D4));
        assert!(bb.is_clear(SQUARE_D2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_E2);
        assert!(bb.is_set(SQUARE_E3));
        assert!(bb.is_set(SQUARE_E4));
        assert!(bb.is_clear(SQUARE_E2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_F2);
        assert!(bb.is_set(SQUARE_F3));
        assert!(bb.is_set(SQUARE_F4));
        assert!(bb.is_clear(SQUARE_F2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_G2);
        assert!(bb.is_set(SQUARE_G3));
        assert!(bb.is_set(SQUARE_G4));
        assert!(bb.is_clear(SQUARE_G2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(SQUARE_H2);
        assert!(bb.is_set(SQUARE_H3));
        assert!(bb.is_set(SQUARE_H4));
        assert!(bb.is_clear(SQUARE_H2));
    }

    #[test]
    pub fn black_double_first_move_mask() {
        let masks = OccupancyMasks::new();

        let mut bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_A7);
        assert!(bb.is_set(SQUARE_A6));
        assert!(bb.is_set(SQUARE_A5));
        assert!(bb.is_clear(SQUARE_A7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_B7);
        assert!(bb.is_set(SQUARE_B6));
        assert!(bb.is_set(SQUARE_B5));
        assert!(bb.is_clear(SQUARE_B7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_C7);
        assert!(bb.is_set(SQUARE_C6));
        assert!(bb.is_set(SQUARE_C5));
        assert!(bb.is_clear(SQUARE_C7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_D7);
        assert!(bb.is_set(SQUARE_D6));
        assert!(bb.is_set(SQUARE_D5));
        assert!(bb.is_clear(SQUARE_D7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_E7);
        assert!(bb.is_set(SQUARE_E6));
        assert!(bb.is_set(SQUARE_E5));
        assert!(bb.is_clear(SQUARE_E7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_F7);
        assert!(bb.is_set(SQUARE_F6));
        assert!(bb.is_set(SQUARE_F5));
        assert!(bb.is_clear(SQUARE_F7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_G7);
        assert!(bb.is_set(SQUARE_G6));
        assert!(bb.is_set(SQUARE_G5));
        assert!(bb.is_clear(SQUARE_G7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(SQUARE_H7);
        assert!(bb.is_set(SQUARE_H6));
        assert!(bb.is_set(SQUARE_H5));
        assert!(bb.is_clear(SQUARE_H7));
    }
}
