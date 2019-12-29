use board::bitboard;
use board::square::file::File;
use board::square::rank::Rank;
use board::square::Square;
use utils;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Dest {
    rank: i8,
    file: i8,
}

pub fn gen_knight_masks() -> Vec<u64> {
    let mut retval = Vec::new();

    let mut dest_sq: Vec<Dest> = Vec::new();

    let map = utils::get_square_rank_file_map();

    for (square, (rank, file)) in map {
        // rank + 2, file +/- 1
        let mut dest_r: i8 = rank as i8 + 2;
        let mut dest_f: i8 = file as i8 + 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);
        dest_f = rank as i8 - 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);

        // rank + 1, file +/- 2
        dest_r = rank as i8 + 1;
        dest_f = file as i8 + 2;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);
        dest_f = file as i8 - 2;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);

        // rank - 1, file +/- 2
        dest_r = rank as i8 - 1;
        dest_f = file as i8 + 2;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);
        dest_f = file as i8 - 2;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);

        // rank - 2, file +/- 1
        dest_r = rank as i8 - 2;
        dest_f = file as i8 + 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);
        dest_f = file as i8 - 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);

        let bb = gen_bitboard(square, &dest_sq);
        retval.push(bb);
    }
    return retval;
}

pub fn gen_white_pawn_capture_masks() -> Vec<u64> {
    let mut retval = Vec::new();

    let mut dest_sq = Vec::new();

    let map = utils::get_square_rank_file_map();

    for (square, (rank, file)) in map {
        if rank == Rank::Rank1 || rank == Rank::Rank8 {
            continue;
        }

        // rank + 1, file +/- 1
        let dest_r: i8 = rank as i8 + 1;
        let mut dest_f: i8 = file as i8 + 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);
        dest_f = rank as i8 - 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);

        let bb = gen_bitboard(square, &dest_sq);
        retval.push(bb);
    }
    return retval;
}

pub fn gen_black_pawn_capture_masks() -> Vec<u64> {
    let mut retval = Vec::new();

    let mut dest_sq = Vec::new();

    let map = utils::get_square_rank_file_map();

    for (square, (rank, file)) in map {
        if rank == Rank::Rank1 || rank == Rank::Rank8 {
            continue;
        }

        // rank - 1, file +/- 1
        let dest_r: i8 = rank as i8 - 1;
        let mut dest_f: i8 = file as i8 + 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);
        dest_f = rank as i8 - 1;
        add_possible_dest(&mut dest_sq, dest_r, dest_f);

        let bb = gen_bitboard(square, &dest_sq);
        retval.push(bb);
    }
    return retval;
}

pub fn gen_king_masks() -> Vec<u64> {
    let mut retval = Vec::new();

    let mut dest_sq = Vec::new();

    let map = utils::get_square_rank_file_map();

    for (square, (rank, file)) in map {
        // rank + 1
        add_possible_dest(&mut dest_sq, rank as i8 + 1, file as i8 - 1);
        add_possible_dest(&mut dest_sq, rank as i8 + 1, file as i8);
        add_possible_dest(&mut dest_sq, rank as i8 + 1, file as i8 + 1);

        // rank
        add_possible_dest(&mut dest_sq, rank as i8, file as i8 - 1);
        add_possible_dest(&mut dest_sq, rank as i8, file as i8 + 1);

        // rank - 1
        add_possible_dest(&mut dest_sq, rank as i8 - 1, file as i8 - 1);
        add_possible_dest(&mut dest_sq, rank as i8 - 1, file as i8);
        add_possible_dest(&mut dest_sq, rank as i8 - 1, file as i8 + 1);

        let bb = gen_bitboard(square, &dest_sq);
        retval.push(bb);
    }
    return retval;
}

pub fn gen_horizontal_vertical_masks() -> Vec<u64> {
    let mut retval = Vec::new();

    let mut dest_sq = Vec::new();

    let map = utils::get_square_rank_file_map();

    for (square, (rank, file)) in map {
        for r in Rank::Rank1 as i8..Rank::Rank8 as i8 {
            for f in File::FileA as i8..File::FileH as i8 {
                if r == rank as i8 && f == file as i8 {
                    continue;
                }
                add_possible_dest(&mut dest_sq, r as i8, f as i8);
            }
        }

        for f in File::FileA as i8..File::FileH as i8 {
            for r in Rank::Rank1 as i8..Rank::Rank8 as i8 {
                if r == rank as i8 && f == file as i8 {
                    continue;
                }
                add_possible_dest(&mut dest_sq, r as i8, f as i8);
            }
        }
        let bb = gen_bitboard(square, &dest_sq);
        retval.push(bb);
    }
    return retval;
}

pub fn gen_both_diagonal_masks() -> Vec<u64> {
    let mut retval = Vec::new();

    let mut dest_sq = Vec::new();

    let map = utils::get_square_rank_file_map();

    for (square, (rank, file)) in map {
        // move left and down
        let mut dest_rank = rank as i8;
        let mut dest_file = file as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            add_possible_dest(&mut dest_sq, dest_rank as i8, dest_file as i8);
            dest_rank -= 1;
            dest_file -= 1;
        }

        // move left and up
        dest_rank = rank as i8;
        dest_file = file as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            add_possible_dest(&mut dest_sq, dest_rank as i8, dest_file as i8);
            dest_rank += 1;
            dest_file -= 1;
        }

        // move right and down
        dest_rank = rank as i8;
        dest_file = file as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            add_possible_dest(&mut dest_sq, dest_rank as i8, dest_file as i8);
            dest_rank -= 1;
            dest_file += 1;
        }

        // move right and up
        dest_rank = rank as i8;
        dest_file = file as i8;
        while is_valid_file(dest_file) && is_valid_rank(dest_rank) {
            add_possible_dest(&mut dest_sq, dest_rank as i8, dest_file as i8);
            dest_rank += 1;
            dest_file += 1;
        }

        let bb = gen_bitboard(square, &dest_sq);
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

fn is_dest_valid(dest: &Dest) -> bool {
    is_valid_rank(dest.rank) && is_valid_file(dest.file)
}

fn gen_bitboard(start_sq: Square, target_sq_list: &Vec<Dest>) -> u64 {
    let mut bb: u64 = 0;

    for dest in target_sq_list {
        if is_dest_valid(dest) {
            let r: Rank = Rank::from_int(dest.rank as u8);
            let f: File = File::from_int(dest.file as u8);

            let sq = Square::get_square(r, f);

            bitboard::set_bit(&mut bb, sq);
        }
    }

    // clear the start square
    bitboard::clear_bit(&mut bb, start_sq);

    return bb;
}

fn add_possible_dest(list: &mut Vec<Dest>, rank: i8, file: i8) {
    list.push(Dest {
        rank: rank,
        file: file,
    });
}
