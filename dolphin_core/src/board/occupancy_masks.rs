use crate::board::bitboard::Bitboard;
use crate::board::file::File;
use crate::board::game_board::Board;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::core::array_offset::EnumAsOffset;
use std::ops::Shl;

const RANK_MASK: Bitboard = Bitboard::new(0x0000_0000_0000_00ff);
const FILE_MASK: Bitboard = Bitboard::new(0x0101_0101_0101_0101);

pub const FILE_A_BB: Bitboard = FILE_MASK;
pub const FILE_H_BB: Bitboard = Bitboard::new(0x8080_8080_8080_8080);

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
            knight: [Bitboard::default(); Board::NUM_SQUARES],
            diagonal: [DiagonalAntidiagonal::default(); Board::NUM_SQUARES],
            king: [Bitboard::default(); Board::NUM_SQUARES],
            in_between: [[Bitboard::default(); Board::NUM_SQUARES]; Board::NUM_SQUARES],
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct OccupancyMasks {
    knight: [Bitboard; Board::NUM_SQUARES],
    diagonal: [DiagonalAntidiagonal; Board::NUM_SQUARES],
    king: [Bitboard; Board::NUM_SQUARES],
    in_between: [[Bitboard; Board::NUM_SQUARES]; Board::NUM_SQUARES],
}

impl OccupancyMasks {
    pub fn new() -> Box<OccupancyMasks> {
        let knight = populate_knight_occupancy_mask_array();
        let diagonal = populate_diagonal_mask_array();
        // let queen = populate_queen_mask_array(&diagonal);
        let king = populate_king_mask_array();
        let in_between = populate_intervening_bitboard_array();

        Box::new(OccupancyMasks {
            knight,
            diagonal,
            king,
            in_between,
        })
    }

    pub fn get_occupancy_mask_bishop(&self, sq: Square) -> Bitboard {
        let d = self.diagonal[sq.as_index()];
        d.get_diag_mask() | d.get_anti_diag_mask()
    }

    pub fn get_occupancy_mask_knight(&self, sq: Square) -> Bitboard {
        *self.knight.get(sq.as_index()).unwrap()
    }

    pub fn get_occupancy_mask_king(&self, sq: Square) -> Bitboard {
        *self.king.get(sq.as_index()).unwrap()
    }

    pub fn get_inbetween_squares(&self, sq1: Square, sq2: Square) -> Bitboard {
        self.in_between[sq1.as_index()][sq2.as_index()]
    }

    pub fn get_horizontal_mask(&self, sq: Square) -> Bitboard {
        get_horizontal_move_mask(sq)
    }

    pub fn get_vertical_mask(&self, sq: Square) -> Bitboard {
        get_vertical_move_mask(sq)
    }

    pub fn get_diag_antidiag_mask(&self, sq: Square) -> &DiagonalAntidiagonal {
        self.diagonal.get(sq.as_index()).unwrap()
    }

    pub fn get_occ_mask_white_pawns_double_move_mask(&self, sq: Square) -> Bitboard {
        let mut bb = sq.get_square_as_bb();
        bb = bb.north();
        bb |= bb.north();
        bb
    }
    pub fn get_occ_mask_black_pawns_double_move_mask(&self, sq: Square) -> Bitboard {
        let mut bb = sq.get_square_as_bb();
        bb = bb.south();
        bb |= bb.south();
        bb
    }

    pub fn get_occ_mask_white_pawns_attacking_sq(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        bb.south_east() | bb.south_west()
    }
    pub fn get_occ_mask_black_pawns_attacking_sq(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        bb.north_east() | bb.north_west()
    }
    pub fn get_occ_mask_white_pawn_capture_non_first_double_move(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        bb.north_east() | bb.north_west()
    }
    pub fn get_occ_mask_black_pawn_capture_non_first_double_move(&self, sq: Square) -> Bitboard {
        let bb = sq.get_square_as_bb();
        bb.south_east() | bb.south_west()
    }
    pub fn get_occ_mask_white_pawn_attack_squares(&self, pawn_sq: Square) -> Bitboard {
        let bb = pawn_sq.get_square_as_bb();
        bb.north_east() | bb.north_west()
    }
    pub fn get_occ_mask_black_pawn_attack_squares(&self, pawn_sq: Square) -> Bitboard {
        let bb = pawn_sq.get_square_as_bb();
        bb.south_east() | bb.south_west()
    }

    // bitboards for squares between castle squares (eg White King side = f1 and g1)
    pub const CASTLE_MASK_WK: Bitboard = Bitboard::new(0x0000_0000_0000_0060);
    pub const CASTLE_MASK_WQ: Bitboard = Bitboard::new(0x0000_0000_0000_000E);
    pub const CASTLE_MASK_BK: Bitboard = Bitboard::new(0x6000_0000_0000_0000);
    pub const CASTLE_MASK_BQ: Bitboard = Bitboard::new(0x0E00_0000_0000_0000);

    // Bitboards representing commonly used ranks
    pub const RANK_2_BB: Bitboard = Bitboard::new(0x0000_0000_0000_FF00);
    pub const RANK_2_TO_6_BB: Bitboard = Bitboard::new(0x0000_FFFF_FFFF_FF00);
    pub const RANK_7_BB: Bitboard = Bitboard::new(0x00FF_0000_0000_0000);
}

fn get_vertical_move_mask(sq: Square) -> Bitboard {
    let file = sq.file();
    FILE_MASK << file.as_index() as u8
}

fn get_horizontal_move_mask(sq: Square) -> Bitboard {
    let rank = sq.rank();
    RANK_MASK << ((rank.as_index() as u8) << 3)
}

fn populate_knight_occupancy_mask_array() -> [Bitboard; Board::NUM_SQUARES] {
    let mut retval: [Bitboard; Board::NUM_SQUARES] = [Bitboard::default(); Board::NUM_SQUARES];

    for sq in Square::iterator() {
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

        retval[sq.as_index()] = bb;
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

fn populate_king_mask_array() -> [Bitboard; Board::NUM_SQUARES] {
    let mut retval: [Bitboard; Board::NUM_SQUARES] = [Bitboard::default(); Board::NUM_SQUARES];

    for sq in Square::iterator() {
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

        retval[sq.as_index()] = bb;
    }
    retval
}

fn is_valid_rank_and_file(rank: Option<Rank>, file: Option<File>) -> bool {
    rank.is_some() && file.is_some()
}

fn populate_diagonal_mask_array() -> [DiagonalAntidiagonal; Board::NUM_SQUARES] {
    let mut retval: [DiagonalAntidiagonal; Board::NUM_SQUARES] =
        [DiagonalAntidiagonal::default(); Board::NUM_SQUARES];

    for sq in Square::iterator() {
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

        retval[sq.as_index()].diag_mask = bb;
    }

    for sq in Square::iterator() {
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

        retval[sq.as_index()].anti_diag_mask = bb;
    }

    retval
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
fn populate_intervening_bitboard_array() -> [[Bitboard; Board::NUM_SQUARES]; Board::NUM_SQUARES] {
    const M1: u64 = 0xffff_ffff_ffff_ffff;
    const A2A7: u64 = 0x0001_0101_0101_0100;
    const B2G7: u64 = 0x0040_2010_0804_0200;
    const H1B7: u64 = 0x0002_0408_1020_4080;

    let mut retval = [[Bitboard::default(); Board::NUM_SQUARES]; Board::NUM_SQUARES];

    for sq1 in Square::iterator() {
        for sq2 in Square::iterator() {
            let btwn = (M1.shl(sq1.as_index() as u8)) ^ (M1.shl(sq2.as_index() as u8));
            let file = (sq2.as_index() as u64 & 7).wrapping_sub(sq1.as_index() as u64 & 7);
            let rank = ((sq2.as_index() as u64 | 7).wrapping_sub(sq1.as_index() as u64)) >> 3;
            let mut line = ((file & 7).wrapping_sub(1)) & A2A7; /* a2a7 if same file */
            line = line.wrapping_add((((rank & 7).wrapping_sub(1)) >> 58).wrapping_mul(2)); /* b1g1 if same rank */
            line = line.wrapping_add((((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & B2G7); /* b2g7 if same diagonal */
            line = line.wrapping_add((((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & H1B7); /* h1b7 if same antidiag */
            line = line.wrapping_mul(btwn & (btwn.wrapping_neg())); /* mul acts like shift by smaller square */
            let val = line & btwn; /* return the bits on that line in-between */

            retval[sq1.as_index()][sq2.as_index()] = Bitboard::new(val);
        }
    }

    retval
}

#[cfg(test)]
pub mod tests {
    use super::OccupancyMasks;
    use crate::board::square::Square;

    #[test]
    pub fn white_double_first_move_mask() {
        let masks = OccupancyMasks::new();

        let mut bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::A2);
        assert!(bb.is_set(Square::A3));
        assert!(bb.is_set(Square::A4));
        assert!(bb.is_clear(Square::A2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::B2);
        assert!(bb.is_set(Square::B3));
        assert!(bb.is_set(Square::B4));
        assert!(bb.is_clear(Square::B2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::C2);
        assert!(bb.is_set(Square::C3));
        assert!(bb.is_set(Square::C4));
        assert!(bb.is_clear(Square::C2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::D2);
        assert!(bb.is_set(Square::D3));
        assert!(bb.is_set(Square::D4));
        assert!(bb.is_clear(Square::D2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::E2);
        assert!(bb.is_set(Square::E3));
        assert!(bb.is_set(Square::E4));
        assert!(bb.is_clear(Square::E2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::F2);
        assert!(bb.is_set(Square::F3));
        assert!(bb.is_set(Square::F4));
        assert!(bb.is_clear(Square::F2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::G2);
        assert!(bb.is_set(Square::G3));
        assert!(bb.is_set(Square::G4));
        assert!(bb.is_clear(Square::G2));

        bb = masks.get_occ_mask_white_pawns_double_move_mask(Square::H2);
        assert!(bb.is_set(Square::H3));
        assert!(bb.is_set(Square::H4));
        assert!(bb.is_clear(Square::H2));
    }

    #[test]
    pub fn black_double_first_move_mask() {
        let masks = OccupancyMasks::new();

        let mut bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::A7);
        assert!(bb.is_set(Square::A6));
        assert!(bb.is_set(Square::A5));
        assert!(bb.is_clear(Square::A7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::B7);
        assert!(bb.is_set(Square::B6));
        assert!(bb.is_set(Square::B5));
        assert!(bb.is_clear(Square::B7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::C7);
        assert!(bb.is_set(Square::C6));
        assert!(bb.is_set(Square::C5));
        assert!(bb.is_clear(Square::C7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::D7);
        assert!(bb.is_set(Square::D6));
        assert!(bb.is_set(Square::D5));
        assert!(bb.is_clear(Square::D7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::E7);
        assert!(bb.is_set(Square::E6));
        assert!(bb.is_set(Square::E5));
        assert!(bb.is_clear(Square::E7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::F7);
        assert!(bb.is_set(Square::F6));
        assert!(bb.is_set(Square::F5));
        assert!(bb.is_clear(Square::F7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::G7);
        assert!(bb.is_set(Square::G6));
        assert!(bb.is_set(Square::G5));
        assert!(bb.is_clear(Square::G7));

        bb = masks.get_occ_mask_black_pawns_double_move_mask(Square::H7);
        assert!(bb.is_set(Square::H6));
        assert!(bb.is_set(Square::H5));
        assert!(bb.is_clear(Square::H7));
    }
}
