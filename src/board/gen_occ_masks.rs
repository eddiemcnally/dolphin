use board;
use board::bitboard;
use board::square::file::File;
use board::square::rank::Rank;
use board::square::Square;
use utils;

pub fn gen_knight_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;
        let rank: i8 = sq.rank() as i8;
        let file: i8 = sq.file() as i8;

        // rank + 2, file +/- 1
        let mut dest_rank: i8 = rank + 2;
        let mut dest_file: i8 = file + 1;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
        dest_file = file - 1;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);

        // rank + 1, file +/- 2
        dest_rank = rank + 1;
        dest_file = file + 2;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
        dest_file = file - 2;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);

        // rank - 1, file +/- 2
        dest_rank = rank - 1;
        dest_file = file + 2;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
        dest_file = file - 2;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);

        // rank - 2, file +/- 1
        dest_rank = rank - 2;
        dest_file = file + 1;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
        dest_file = file - 1;
        set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);

        retval.push(bb);
    }
    return retval;
}

pub fn gen_white_pawn_capture_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        if sq.rank() == Rank::Rank1 || sq.rank() == Rank::Rank8 {
            retval.push(bb);
        } else {
            let rank: i8 = sq.rank() as i8;
            let file: i8 = sq.file() as i8;

            // rank + 1, file +/- 1
            let dest_rank: i8 = rank as i8 + 1;
            let mut dest_file: i8 = file as i8 + 1;
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
            dest_file = file as i8 - 1;
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);

            retval.push(bb);
        }
    }
    return retval;
}

pub fn gen_black_pawn_capture_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        if sq.rank() == Rank::Rank1 || sq.rank() == Rank::Rank8 {
            retval.push(bb);
        } else {
            let rank: i8 = sq.rank() as i8;
            let file: i8 = sq.file() as i8;

            // rank - 1, file +/- 1
            let dest_rank: i8 = rank as i8 - 1;
            let mut dest_file: i8 = file as i8 + 1;
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
            dest_file = file as i8 - 1;
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);

            retval.push(bb);
        }
    }
    return retval;
}

pub fn gen_king_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;
        let rank: i8 = sq.rank() as i8;
        let file: i8 = sq.file() as i8;

        // rank + 1
        set_dest_sq_if_valid(&mut bb, rank + 1, file - 1);
        set_dest_sq_if_valid(&mut bb, rank + 1, file);
        set_dest_sq_if_valid(&mut bb, rank + 1, file + 1);

        // rank
        set_dest_sq_if_valid(&mut bb, rank, file - 1);
        set_dest_sq_if_valid(&mut bb, rank, file + 1);

        // rank - 1
        set_dest_sq_if_valid(&mut bb, rank - 1, file - 1);
        set_dest_sq_if_valid(&mut bb, rank - 1, file);
        set_dest_sq_if_valid(&mut bb, rank - 1, file + 1);

        retval.push(bb);
    }
    return retval;
}

pub fn gen_rank_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    for r in Rank::iterator() {
        let rank = *r as i8;
        let mut bb: u64 = 0;

        for f in File::iterator() {
            let file = *f as i8;
            set_dest_sq_if_valid(&mut bb, rank, file);
        }
        retval.push(bb);
    }
    return retval;
}

pub fn gen_file_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    for f in File::iterator() {
        let file = *f as i8;
        let mut bb: u64 = 0;

        for r in Rank::iterator() {
            let rank = *r as i8;
            set_dest_sq_if_valid(&mut bb, rank, file);
        }
        retval.push(bb);
    }

    return retval;
}

pub fn gen_queen_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let bishop_masks = gen_bishop_masks();
    let rook_masks = gen_rook_masks();

    if bishop_masks.len() != 64 || bishop_masks.len() != rook_masks.len() {
        panic!("Problem");
    }

    let squares = utils::get_ordered_square_list_by_file();

    for sq in squares {
        let mut queen_mask = bishop_masks[sq.to_offset()] | rook_masks[sq.to_offset()];

        // remove current square
        bitboard::clear_bit(&mut queen_mask, sq);

        retval.push(queen_mask);
    }

    return retval;
}

pub fn get_diagonal_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;

        // move left and down
        let mut dest_rank = sq.rank() as i8;
        let mut dest_file = sq.file() as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
            dest_rank -= 1;
            dest_file -= 1;
        }

        // move right and up
        dest_rank = sq.rank() as i8;
        dest_file = sq.file() as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
            dest_rank += 1;
            dest_file += 1;
        }

        // remove current square
        bitboard::clear_bit(&mut bb, *sq);

        retval.push(bb);
    }
    return retval;
}

pub fn get_anti_diagonal_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;
        let mut dest_rank = sq.rank() as i8;
        let mut dest_file = sq.file() as i8;

        // move left and up
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
            dest_rank += 1;
            dest_file -= 1;
        }

        // move right and down
        dest_rank = sq.rank() as i8;
        dest_file = sq.file() as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            set_dest_sq_if_valid(&mut bb, dest_rank, dest_file);
            dest_rank -= 1;
            dest_file += 1;
        }

        // remove current square
        bitboard::clear_bit(&mut bb, *sq);

        retval.push(bb);
    }
    return retval;
}

pub fn gen_bishop_masks() -> Vec<u64> {
    let diag_masks = get_diagonal_masks();
    let anti_diag_masks = get_anti_diagonal_masks();

    if diag_masks.len() != 64 || diag_masks.len() != anti_diag_masks.len() {
        panic!("Problem");
    }
    let mut retval = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bishop_mask = diag_masks[sq.to_offset()] | anti_diag_masks[sq.to_offset()];

        // remove current square
        bitboard::clear_bit(&mut bishop_mask, *sq);

        retval.push(bishop_mask);
    }

    return retval;
}

pub fn gen_rook_masks() -> Vec<u64> {
    let mut retval: Vec<u64> = Vec::new();

    let squares = board::square::SQUARES;

    for sq in squares {
        let mut bb: u64 = 0;
        let dest_rank = sq.rank() as i8;
        let dest_file = sq.file() as i8;

        // move up the ranks of this file
        for r in Rank::iterator() {
            set_dest_sq_if_valid(&mut bb, *r as i8, dest_file);
        }

        for f in File::iterator() {
            set_dest_sq_if_valid(&mut bb, dest_rank, *f as i8);
        }

        // remove current square
        bitboard::clear_bit(&mut bb, *sq);

        retval.push(bb);
    }
    return retval;
}

fn is_valid_rank(r: i8) -> bool {
    r >= Rank::Rank1 as i8 && r <= Rank::Rank8 as i8
}

fn is_valid_file(f: i8) -> bool {
    f >= File::FileA as i8 && f <= File::FileH as i8
}

fn set_dest_sq_if_valid(bb: &mut u64, dest_rank: i8, dest_file: i8) {
    if is_valid_rank(dest_rank) && is_valid_file(dest_file) {
        let r: Rank = Rank::from_int(dest_rank as u8);
        let f: File = File::from_int(dest_file as u8);

        let sq = Square::get_square(r, f);

        bitboard::set_bit(bb, sq);
    }
}

#[cfg(test)]
pub mod tests {
    use board;
    use board::occupancy_masks;

    #[test]
    pub fn compare_knight_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_knight_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_occupancy_mask_knight(*sq);

            assert_eq!(new_mask, ref_mask);
        }
    }

    #[test]
    pub fn compare_bishop_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_bishop_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_occupancy_mask_bishop(*sq);

            assert!(new_mask == ref_mask);
        }
    }

    #[test]
    pub fn compare_rook_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_rook_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_occupancy_mask_rook(*sq);

            assert!(new_mask == ref_mask);
        }
    }

    #[test]
    pub fn compare_black_pawn_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_black_pawn_capture_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_black_pawn_capture_mask(*sq);

            assert!(new_mask == ref_mask);
        }
    }

    #[test]
    pub fn compare_white_pawn_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_white_pawn_capture_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_white_pawn_capture_mask(*sq);

            assert!(new_mask == ref_mask);
        }
    }

    #[test]
    pub fn compare_queen_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_queen_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_occupancy_mask_queen(*sq);

            assert!(new_mask == ref_mask);
        }
    }

    #[test]
    pub fn compare_king_masks() {
        let squares = board::square::SQUARES;

        let masks = super::gen_king_masks();

        for sq in squares {
            let new_mask = masks[sq.to_offset()];

            let ref_mask = occupancy_masks::get_occupancy_mask_king(*sq);

            assert!(new_mask == ref_mask);
        }
    }
}
