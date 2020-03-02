use board::bitboard;
use board::board::Board;
use board::occupancy_masks;
use board::piece::Colour;
use board::piece::Piece;
use board::square::Square;
use std::ops::Shl;

pub fn is_sq_attacked(board: &Board, sq: Square, attacking_side: Colour) -> bool {
    let all_pce_bb = board.get_bitboard();

    match attacking_side {
        Colour::White => {
            if is_attacked_by_pawn_white(board, sq) {
                return true;
            }

            if is_knight_attacking(board, sq, Piece::WhiteKnight) {
                return true;
            }

            // combine piece bitboards
            let mut attack_pce_bb = board.get_piece_bitboard(Piece::WhiteRook);
            attack_pce_bb |= board.get_piece_bitboard(Piece::WhiteQueen);
            if is_horizontal_or_vertical_attacking(all_pce_bb, attack_pce_bb, sq) {
                return true;
            }

            if is_diagonally_attacked(board, sq, Piece::WhiteBishop, Piece::WhiteQueen) {
                return true;
            }
        }
        Colour::Black => {
            if is_attacked_by_pawn_black(board, sq) {
                return true;
            }

            if is_knight_attacking(board, sq, Piece::BlackKnight) {
                return true;
            }

            // combine piece bitboards
            let mut attack_pce_bb = board.get_piece_bitboard(Piece::BlackRook);
            attack_pce_bb |= board.get_piece_bitboard(Piece::BlackQueen);
            if is_horizontal_or_vertical_attacking(all_pce_bb, attack_pce_bb, sq) {
                return true;
            }

            if is_diagonally_attacked(board, sq, Piece::BlackBishop, Piece::BlackQueen) {
                return true;
            }
        }
    }

    if is_attacked_by_king(board, sq, attacking_side) {
        return true;
    }

    return false;
}

fn is_knight_attacking(board: &Board, attack_sq: Square, attacking_pce: Piece) -> bool {
    let mut pce_bb = board.get_piece_bitboard(attacking_pce);

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
            let blocking_pces = get_intervening_bitboard(pce_sq, attack_sq);
            if blocking_pces & all_piece_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }
    return false;
}

fn is_diagonally_attacked(board: &Board, attack_sq: Square, bishop: Piece, queen: Piece) -> bool {
    // combine piece bitboards
    let mut attack_pce_bb = board.get_piece_bitboard(bishop);
    attack_pce_bb |= board.get_piece_bitboard(queen);

    if check_diagonal(board, attack_pce_bb, attack_sq) {
        return true;
    }

    return false;
}

fn check_diagonal(board: &Board, piece_bb: u64, attack_sq: Square) -> bool {
    let all_bb = board.get_bitboard();
    let mut pce_bb = piece_bb;
    while pce_bb != 0 {
        let pce_sq = bitboard::pop_1st_bit(&mut pce_bb);

        let diagonal_bb = occupancy_masks::get_occupancy_mask_bishop(pce_sq);
        if bitboard::is_set(diagonal_bb, attack_sq) {
            // potentially attacking
            let blocking_pces = get_intervening_bitboard(pce_sq, attack_sq);
            if blocking_pces & all_bb == 0 {
                // no blocking pieces, attacked
                return true;
            }
        }
    }
    return false;
}

fn is_attacked_by_king(board: &Board, attacked_sq: Square, attacking_side: Colour) -> bool {
    let attacking_king_sq = board.get_king_sq(attacking_side);
    let king_occ_mask = occupancy_masks::get_occupancy_mask_king(attacking_king_sq);
    return bitboard::is_set(king_occ_mask, attacked_sq);
}

fn is_attacked_by_pawn_white(board: &Board, attacked_sq: Square) -> bool {
    let wp_bb = board.get_piece_bitboard(Piece::WhitePawn);

    // -1 Rank and +/- 1 File
    let r: i8 = attacked_sq.rank() as i8 - 1;
    let mut f: i8 = attacked_sq.file() as i8 - 1;
    if is_pawn_attacking(r, f, wp_bb) {
        return true;
    }
    f = attacked_sq.file() as i8 + 1;
    if is_pawn_attacking(r, f, wp_bb) {
        return true;
    }

    return false;
}

fn is_attacked_by_pawn_black(board: &Board, attacked_sq: Square) -> bool {
    let bp_bb = board.get_piece_bitboard(Piece::BlackPawn);

    // +1 Rank and +/- 1 File
    let r: i8 = attacked_sq.rank() as i8 + 1;
    let mut f: i8 = attacked_sq.file() as i8 - 1;
    if is_pawn_attacking(r, f, bp_bb) {
        return true;
    }

    f = attacked_sq.file() as i8 + 1;
    if is_pawn_attacking(r, f, bp_bb) {
        return true;
    }

    return false;
}

fn is_pawn_attacking(attacking_rank: i8, attacking_file: i8, pawn_bb: u64) -> bool {
    if is_valid_rank_and_file(attacking_rank, attacking_file) {
        let sq_as_bb = Square::get_square_as_bb(attacking_rank as u8, attacking_file as u8);
        if sq_as_bb & pawn_bb != 0 {
            return true;
        }
    }
    return false;
}

fn is_valid_rank_and_file(r: i8, f: i8) -> bool {
    let range = 0..8;
    return range.contains(&r) && range.contains(&f);
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
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

#[cfg(test)]
mod tests {
    use board::board::Board;
    use board::piece::Colour;
    use input::fen;

    #[test]
    pub fn white_knight_attacking_false() {
        // set up a list of FENs with BK and WN such that BK *isn't* under attack
        let fens = vec![
            "8/1k6/8/3N4/1N6/8/8/7K w - - 0 1",
            "k7/1N6/N7/8/8/8/8/K7 w - - 0 1",
            "K7/8/2N5/1N6/8/8/8/k7 w - - 0 1",
            "K7/8/2N5/1Nk5/8/8/8/8 w - - 0 1",
            "K7/8/1kN5/1N6/8/8/8/8 w - - 0 1",
            "K7/8/8/1N6/8/8/8/5N1k w - - 0 1",
            "K7/8/8/8/8/8/6N1/5N1k w - - 0 1",
            "K7/8/8/8/8/8/5kN1/5N2 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_knight_attacking_false() {
        // set up a list of FENs with WK and BN such that WK *isn't* under attack
        let fens = vec![
            "K7/1n6/n7/8/8/8/7k/8 w - - 0 1",
            "K7/8/n1n5/8/8/8/5k2/8 w - - 0 1",
            "k7/8/8/8/8/8/7n/5n1K w - - 0 1",
            "k7/8/8/8/8/8/6Kn/5n2 w - - 0 1",
            "k7/8/8/2Kn4/2n5/8/8/8 w - - 0 1",
            "k7/8/8/3n4/2n5/8/8/K7 w - - 0 1",
            "k7/8/8/3n4/2n5/8/8/K7 w - - 0 1",
            "k6n/5n2/8/8/8/8/8/7K w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }

    #[test]
    pub fn white_knight_attacking_true() {
        // set up a list of FENs with BK and WN such that BK *is* under attack
        let fens = vec![
            "7k/5N2/6N1/8/8/8/8/K7 w - - 0 1",
            "7k/5N2/8/8/8/5N2/8/K7 w - - 0 1",
            "8/8/8/8/8/6N1/5N2/K6k w - - 0 1",
            "8/8/8/8/4k3/6N1/5N2/K7 w - - 0 1",
            "k7/8/1N6/3N4/8/8/8/K7 w - - 0 1",
            "k7/2N5/1N6/8/8/8/8/K7 w - - 0 1",
            "8/8/8/5N2/3k4/1N6/8/K7 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_knight_attacking_true() {
        // set up a list of FENs with WK and BN such that WK *is* under attack
        let fens = vec![
            "7K/k4n2/6n1/8/8/8/8/8 w - - 0 1",
            "7K/k4n2/8/4n3/8/8/8/8 w - - 0 1",
            "8/k4n2/8/4n1K1/8/8/8/8 w - - 0 1",
            "8/k2K4/8/2n1n3/8/8/8/8 w - - 0 1",
            "8/k7/8/2n1n3/8/3K4/8/8 w - - 0 1",
            "8/k7/8/2n1n3/8/3K4/8/8 w - - 0 1",
            "8/k7/8/8/8/1n6/2n5/K7 w - - 0 1",
            "K1n5/8/1n4k1/8/8/8/8/8 w - - 0 1",
            "K7/2n5/1n4k1/8/8/8/8/8 w - - 0 1",
            "8/5n2/8/8/8/k5n1/8/7K w - - 0 1",
            "8/8/8/8/8/k5n1/5n2/7K w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_horizontal_vertical_attacking_false() {
        // set up a list of FENs with BK R and Q such that BK *isn't* under attack
        let fens = vec![
            "7k/8/8/8/8/1R6/2QR4/K7 w - - 0 1",
            "5k2/8/8/8/8/1R6/2QR4/K7 w - - 0 1",
            "k7/8/8/8/8/1R6/2QR4/7K w - - 0 1",
            "K7/8/8/8/8/1R6/2QR4/k7 w - - 0 1",
            "K7/8/8/8/8/1R6/2QR4/4k3 w - - 0 1",
            "K7/8/8/8/8/1R1Q4/3R4/7k w - - 0 1",
            "K7/8/8/8/7k/1R1Q4/3R4/8 w - - 0 1",
            "K7/8/8/1R6/7k/3R4/4Q3/8 w - - 0 1",
            "K7/8/8/1R6/4Q1pk/3R4/8/8 w - - 0 1",
            "K7/7k/6p1/1R6/4Q3/3R4/8/8 w - - 0 1",
            "K7/4k3/4p3/1R6/4Q3/3R4/8/8 w - - 0 1",
            "1k6/1p6/8/1R6/4Q3/3R4/8/1K6 w - - 0 1",
            "1k6/1P6/8/1R6/4Q3/3R4/8/1K6 w - - 0 1",
            "8/7k/8/1R3P2/4Q3/3R4/8/K7 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_horizontal_vertical_attacking_false() {
        // set up a list of FENs with BK, BR and BQ such that WK *isn't* under attack
        let fens = vec![
            "K7/2qr4/8/1r4k1/8/8/8/8 w - - 0 1",
            "K7/1r6/1q6/1r6/8/8/6k1/8 w - - 0 1",
            "1r6/K7/2q5/1r6/8/8/6k1/8 w - - 0 1",
            "1r6/7K/2q5/1r6/8/8/8/k7 w - - 0 1",
            "1r6/7K/1r6/6q1/8/8/1k6/8 w - - 0 1",
            "6r1/7K/6r1/6q1/8/8/1k6/8 w - - 0 1",
            "1k6/8/8/8/8/4r1q1/5r2/7K w - - 0 1",
            "1k6/8/8/8/6r1/4r3/5q2/7K w - - 0 1",
            "7k/8/8/8/8/1r6/2q3r1/K7 w - - 0 1",
            "8/7k/8/8/1qr5/2r5/8/K7 w - - 0 1",
            "8/8/8/4K3/1qr5/2r5/8/k7 w - - 0 1",
            "8/3K4/8/8/1qr5/2r5/6k1/8 w - - 0 1",
            "8/k7/8/8/1qr5/2r5/5K2/8 w - - 0 1",
            "8/k7/8/8/1qr5/2r1PK2/8/8 w - - 0 1",
            "8/4K3/3P4/8/1qr5/2r5/3k4/8 w - - 0 1",
            "8/1K6/1P6/8/1qr5/2r5/5k2/8 w - - 0 1",
            "8/1K6/1p6/8/1qr5/2r5/5k2/8 w - - 0 1",
            "8/2K5/2p5/8/1qr5/2r5/4k3/8 w - - 0 1",
            "8/8/k7/8/1qr2pK1/2r5/8/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }

    #[test]
    pub fn white_horizontal_vertical_attacking_true() {
        // set up a list of FENs with WK, WR and WQ such that BK *is* under attack
        let fens = vec![
            "4k3/8/8/8/8/K7/3R4/4Q2R w - - 0 1",
            "8/8/8/8/8/K7/6R1/4Q1kR w - - 0 1",
            "3Rk3/8/8/8/8/K7/8/4Q2R w - - 0 1",
            "7k/8/8/8/8/K7/6R1/4Q2R w - - 0 1",
            "3R4/8/8/7k/8/K7/8/4Q2R w - - 0 1",
            "k2R4/8/8/8/8/K7/8/4Q2R w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_horizontal_vertical_attacking_true() {
        // set up a list of FENs with BK, BR and BQ such that WK *is* under attack
        let fens = vec![
            "1k6/3r2q1/8/8/8/8/3K4/3r4 w - - 0 1",
            "1k6/3r2q1/8/8/8/8/8/3r2K1 w - - 0 1",
            "1k6/3r2q1/8/8/8/8/8/K2r4 w - - 0 1",
            "1k6/3rK1q1/8/8/8/8/8/3r4 w - - 0 1",
            "7k/K2r2q1/8/8/8/8/8/3r4 w - - 0 1",
            "7k/3r4/3r4/8/8/3K2q1/8/8 w - - 0 1",
            "7k/3r4/K2r4/8/8/6q1/8/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_diagonal_attacking_false() {
        // set up a list of FENs with WK, WB and WQ such that BK *isn't* under attack
        let fens = vec![
            "K7/8/8/8/8/QB6/1B6/7k w - - 0 1",
            "7k/8/8/8/8/QB6/8/K1B5 w - - 0 1",
            "k7/8/8/4Q3/5B2/1B6/4K3/8 w - - 0 1",
            "K7/8/8/4Q3/5B2/8/k1B5/8 w - - 0 1",
            "K7/8/8/4Q3/2k2B2/8/2B5/8 w - - 0 1",
            "K7/8/2k5/4QB2/5B2/8/8/8 w - - 0 1",
            "K7/2k5/3p4/4QB2/5B2/8/8/8 w - - 0 1",
            "1k6/8/3P4/4QB2/5B2/8/8/K7 w - - 0 1",
            "K7/8/3Q4/4BB2/3P4/8/8/k7 w - - 0 1",
            "7k/8/3Q1p2/4BB2/8/8/8/K7 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_diagonal_attacking_false() {
        // set up a list of FENs with BK, BR and BQ such that WK *isn't* under attack
        let fens = vec![
            "k7/8/8/8/8/qb6/1b6/7K w - - 0 1",
            "8/8/2K5/8/8/qb6/1b5k/8 w - - 0 1",
            "8/4q3/2K5/4b3/3b4/8/8/7k w - - 0 1",
            "5k2/4q3/8/8/3bb3/1K6/8/8 w - - 0 1",
            "6k1/8/8/8/2q1b3/4b3/8/K7 w - - 0 1",
            "1k6/8/8/8/5q2/3bb3/8/7K w - - 0 1",
            "7K/8/8/2bq4/8/3b4/k7/8 w - - 0 1",
            "7K/8/5p2/2b1q3/k7/3b4/8/8 w - - 0 1",
            "7K/8/5P2/2b1q3/8/k2b4/8/8 w - - 0 1",
            "K7/8/2P5/2b4k/4q3/3b4/8/8 w - - 0 1",
            "3k4/8/8/2b1q3/8/2Pb4/8/K7 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }

    #[test]
    pub fn white_diagonal_attacking_true() {
        // set up a list of FENs with WK, WB and WQ such that BK *is* under attack
        let fens = vec![
            "8/8/1K6/4Q3/4BB2/8/8/7k w - - 0 1",
            "7K/1Q6/8/8/4BB2/8/8/7k w - - 0 1",
            "6K1/1Q6/8/8/4BB2/8/7k/8 w - - 0 1",
            "7k/1Q6/8/4B3/4B3/8/8/1K6 w - - 0 1",
            "3K4/1Q6/8/4B3/4B3/8/8/1k6 w - - 0 1",
            "3K4/1Q6/8/4B3/4B3/8/1k6/8 w - - 0 1",
            "3K4/1Q6/8/4B3/4B3/8/8/k7 w - - 0 1",
            "3K4/1Q5k/8/4B3/4B3/8/8/8 w - - 0 1",
            "3K4/1Q6/8/4B3/4B3/8/7k/8 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_diagonal_attacking_true() {
        // set up a list of FENs with BK, BB and BQ such that WK *is* under attack
        let fens = vec![
            "1k6/8/8/2b5/4b3/3q4/8/6K1 w - - 0 1",
            "1k6/8/1q6/2b5/4b3/8/8/6K1 w - - 0 1",
            "1k6/8/1q6/2b5/4b3/K7/8/8 w - - 0 1",
            "1k6/8/1q6/2bK4/4b3/8/8/8 w - - 0 1",
            "1k5q/8/8/4K3/8/8/1b6/7b w - - 0 1",
            "1k5q/8/2K5/8/8/8/1b6/7b w - - 0 1",
        ];
        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);
            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_pawn_attacking_true() {
        let fens = vec![
            "8/8/8/8/8/1k6/2P5/8 w - - 0 1",
            "8/8/8/8/8/1k6/P7/8 w - - 0 1",
            "8/8/8/8/8/7k/6P1/8 w - - 0 1",
            "8/8/8/8/7k/6P1/8/8 w - - 0 1",
            "8/8/8/k7/1P6/8/8/8 w - - 0 1",
            "8/8/8/2k5/1P6/8/8/8 w - - 0 1",
            "8/1k6/2P5/8/8/8/8/8 w - - 0 1",
            "8/1k6/P7/8/8/8/8/8 w - - 0 1",
            "2k5/1P6/8/8/8/8/8/8 w - - 0 1",
            "2k5/1P1P4/8/8/8/8/8/8 w - - 0 1",
            "8/8/8/8/2k5/1PP5/8/8 w - - 0 1",
            "8/8/8/8/1k6/1PP5/8/8 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);
            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_pawn_attacking_true() {
        let fens = vec![
            "8/8/8/3p4/2K5/8/8/8 w - - 0 1",
            "8/8/8/3p4/4K3/8/8/8 w - - 0 1",
            "8/3p4/4K3/8/8/8/8/8 w - - 0 1",
            "8/3p4/2K5/8/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/8/2p5/1K6 w - - 0 1",
            "8/8/8/8/8/8/2p5/3K4 w - - 0 1",
            "8/8/8/8/8/8/2p1p3/3K4 w - - 0 1",
            "8/2p1p3/3K4/8/8/8/8/8 w - - 0 1",
            "8/2p5/1Kp5/8/8/8/8/8 w - - 0 1",
            "8/2p5/2p5/1K6/8/8/8/8 w - - 0 1",
            "8/2p5/2p5/3K4/8/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);
            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_pawn_attacking_false() {
        let fens = vec![
            "K7/8/8/2k5/2P1P3/8/8/8 w - - 0 1",
            "K7/8/8/8/2k5/8/1PP5/8 w - - 0 1",
            "K7/8/8/8/2k5/2P5/1P6/8 w - - 0 1",
            "K7/8/8/8/8/2P1k3/4P3/8 w - - 0 1",
            "K7/8/8/8/8/4k3/2P1P3/8 w - - 0 1",
            "4k3/4P3/2P5/8/8/8/8/K7 w - - 0 1",
            "7k/2P4P/8/8/8/8/8/K7 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(super::is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_pawn_attacking_false() {
        let fens = vec![
            "k7/2p1p3/8/8/3K4/8/8/8 w - - 0 1",
            "k7/2p1p3/8/3K4/8/8/8/8 w - - 0 1",
            "8/2pKp3/8/8/8/8/k7/8 w - - 0 1",
            "3K4/2p1p3/8/8/8/8/3k4/8 w - - 0 1",
            "1k6/8/8/8/3p4/2pK4/8/8 w - - 0 1",
            "2k5/8/8/8/3p4/1Kp5/8/8 w - - 0 1",
            "k7/8/8/2K5/2pp4/8/8/8 w - - 0 1",
            "2k5/8/8/1K6/2pp4/8/8/8 w - - 0 1",
            "k7/8/8/4K3/2pp4/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let king_sq = board.get_king_sq(Colour::White);
            assert!(super::is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }
}
