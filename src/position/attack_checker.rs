use board::bitboard;
use board::board;
use board::board::Board;
use board::occupancy_masks;
use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square;
use board::square::Square;

use std::ops::Shl;

pub struct AttackChecker {
    // A lookup array of bitmasks for squares between the "from" and "to"
    // squares.
    // Since there is a commutative property associated with to/from squares
    // when identifying intervening squares, it's irrelevent whether you index using
    // [from][to] or [to][from]
    inter_sq_lookup: [[u64; board::NUM_SQUARES]; board::NUM_SQUARES],

    // cache some things that otherwise would need to be created each time the code is run
    pawn_black: Piece,
    pawn_white: Piece,
    knight_black: Piece,
    knight_white: Piece,
    king_black: Piece,
    king_white: Piece,
    rook_queen_black_vec: Vec<Piece>,
    rook_queen_white_vec: Vec<Piece>,
    bishop_queen_black_vec: Vec<Piece>,
    bishop_queen_white_vec: Vec<Piece>,
}

impl AttackChecker {
    pub fn new() -> AttackChecker {
        let rook_black = Piece::new(PieceRole::Rook, Colour::Black);
        let queen_black = Piece::new(PieceRole::Queen, Colour::Black);
        let rook_white = Piece::new(PieceRole::Rook, Colour::White);
        let queen_white = Piece::new(PieceRole::Queen, Colour::White);

        let bishop_black = Piece::new(PieceRole::Bishop, Colour::Black);
        let bishop_white = Piece::new(PieceRole::Bishop, Colour::White);

        let mut checker = AttackChecker {
            inter_sq_lookup: [[0u64; board::NUM_SQUARES]; board::NUM_SQUARES],

            pawn_black: Piece::new(PieceRole::Pawn, Colour::Black),
            pawn_white: Piece::new(PieceRole::Pawn, Colour::White),
            knight_black: Piece::new(PieceRole::Knight, Colour::Black),
            knight_white: Piece::new(PieceRole::Knight, Colour::White),
            king_black: Piece::new(PieceRole::King, Colour::Black),
            king_white: Piece::new(PieceRole::King, Colour::White),

            rook_queen_black_vec: vec![rook_black, queen_black],
            rook_queen_white_vec: vec![rook_white, queen_white],

            bishop_queen_black_vec: vec![bishop_black, queen_black],
            bishop_queen_white_vec: vec![bishop_white, queen_white],
        };

        populate_intervening_sq_lookup(&mut checker);
        checker
    }

    pub fn is_sq_attacked(&self, board: &Board, sq: Square, attacking_side: Colour) -> bool {
        if is_knight_attacking(&self, board, sq, attacking_side) {
            return true;
        }

        if is_horizontal_or_vertical_attacking(&self, board, sq, attacking_side) {
            return true;
        }

        if is_diagonally_attacked(&self, board, sq, attacking_side) {
            return true;
        }

        if is_attacked_by_pawn(&self, board, sq, attacking_side) {
            return true;
        }

        if is_attacked_by_king(&self, board, sq, attacking_side) {
            return true;
        }

        return false;
    }
}

fn is_knight_attacking(
    checker: &AttackChecker,
    board: &Board,
    attack_sq: Square,
    attacking_side: Colour,
) -> bool {
    let pce = match attacking_side {
        Colour::Black => checker.knight_black,
        Colour::White => checker.knight_white,
    };

    let mut pce_bb = board.get_piece_bitboard(pce);

    while pce_bb != 0 {
        let sq = bitboard::pop_1st_bit(&mut pce_bb);
        let occ_mask = occupancy_masks::get_occupancy_mask_knight(sq);
        if bitboard::is_set(occ_mask, attack_sq) {
            return true;
        }
    }
    return false;
}

// checks for rook and partial queen
fn is_horizontal_or_vertical_attacking(
    checker: &AttackChecker,
    board: &Board,
    attack_sq: Square,
    attacking_side: Colour,
) -> bool {
    let target_rank = attack_sq.rank();
    let target_file = attack_sq.file();

    let pces = match attacking_side {
        Colour::Black => &checker.rook_queen_black_vec,
        Colour::White => &checker.rook_queen_white_vec,
    };

    let all_pces_bb = board.get_bitboard();

    for pce in pces {
        let mut pce_bb = board.get_piece_bitboard(*pce);
        while pce_bb != 0 {
            let pce_sq = bitboard::pop_1st_bit(&mut pce_bb);

            if pce_sq.rank() == target_rank || pce_sq.file() == target_file {
                let blocking_pces = checker.inter_sq_lookup[pce_sq as usize][attack_sq as usize];
                if blocking_pces & all_pces_bb == 0 {
                    // no blocking pieces, attacked
                    return true;
                }
            }
        }
    }
    return false;
}

fn is_diagonally_attacked(
    checker: &AttackChecker,
    board: &Board,
    attack_sq: Square,
    attacking_side: Colour,
) -> bool {
    let pces = match attacking_side {
        Colour::Black => &checker.bishop_queen_black_vec,
        Colour::White => &checker.bishop_queen_white_vec,
    };

    let all_pces_bb = board.get_bitboard();

    for pce in pces {
        let mut pce_bb = board.get_piece_bitboard(*pce);
        while pce_bb != 0 {
            let pce_sq = bitboard::pop_1st_bit(&mut pce_bb);

            let diagonal_bb = occupancy_masks::get_occupancy_mask_bishop(pce_sq);
            if bitboard::is_set(diagonal_bb, attack_sq) {
                // potentially attacking
                let blocking_pces = checker.inter_sq_lookup[pce_sq as usize][attack_sq as usize];
                if blocking_pces & all_pces_bb == 0 {
                    // no blocking pieces, attacked
                    return true;
                }
            }
        }
    }

    return false;
}

fn is_attacked_by_king(
    checker: &AttackChecker,
    board: &Board,
    attacked_sq: Square,
    attacking_side: Colour,
) -> bool {
    let attacked_king = match attacking_side {
        Colour::Black => checker.king_white,
        Colour::White => checker.king_black,
    };
    let mut pce_bb = board.get_piece_bitboard(attacked_king);
    let attacking_king_sq = bitboard::pop_1st_bit(&mut pce_bb);

    let king_occ_mask = occupancy_masks::get_occupancy_mask_king(attacking_king_sq);

    return bitboard::is_set(king_occ_mask, attacked_sq);
}

fn is_attacked_by_pawn(
    checker: &AttackChecker,
    board: &Board,
    attacked_sq: Square,
    attacking_side: Colour,
) -> bool {
    let attacking_pce = match attacking_side {
        Colour::Black => checker.pawn_black,
        Colour::White => checker.pawn_white,
    };

    let mut pce_bb = board.get_piece_bitboard(attacking_pce);
    while pce_bb != 0 {
        let pce_sq = bitboard::pop_1st_bit(&mut pce_bb);

        // TODO : this won't work if the Pawn is on the 1st 2 ranks, and the king is up there was well
        // TODO : see the occupancy mask code
        let occ_mask = match attacking_side {
            Colour::White => occupancy_masks::get_white_pawn_capture_mask(pce_sq),
            Colour::Black => occupancy_masks::get_black_pawn_capture_mask(pce_sq),
        };

        if bitboard::is_set(occ_mask, attacked_sq) {
            // attacked
            return true;
        }
    }
    return false;
}

fn populate_intervening_sq_lookup(checker: &mut AttackChecker) {
    for from_sq in square::get_square_array() {
        for to_sq in square::get_square_array() {
            let bitmap = get_intervening_bitboard(*from_sq, *to_sq);
            checker.inter_sq_lookup[*from_sq as usize][*to_sq as usize] = bitmap;
        }
    }
}

// This code returns a bitboard with bits set representing squares between
// the given 2 squares.
//
// The code is taken from :
// https://www.chessprogramming.org/Square_Attacked_By
//
pub fn get_intervening_bitboard(sq1: Square, sq2: Square) -> u64 {
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
    use position::attack_checker::AttackChecker;

    #[test]
    pub fn white_knight_attacking_false() {
        // set up a list of FENs with BK and WN such that BK *isn't* under attack
        let fens = vec![
            "8/1k6/8/3N4/1N6/8/8/8 w - - 0 1",
            "k7/1N6/N7/8/8/8/8/8 w - - 0 1",
            "8/8/2N5/1N6/8/8/8/k7 w - - 0 1",
            "8/8/2N5/1Nk5/8/8/8/8 w - - 0 1",
            "8/8/1kN5/1N6/8/8/8/8 w - - 0 1",
            "8/8/8/1N6/8/8/8/5N1k w - - 0 1",
            "8/8/8/8/8/8/6N1/5N1k w - - 0 1",
            "8/8/8/8/8/8/5kN1/5N2 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_knight_attacking_false() {
        // set up a list of FENs with WK and BN such that WK *isn't* under attack
        let fens = vec![
            "K7/1n6/n7/8/8/8/8/8 w - - 0 1",
            "K7/8/n1n5/8/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/8/7n/5n1K w - - 0 1",
            "8/8/8/8/8/8/6Kn/5n2 w - - 0 1",
            "8/8/8/2Kn4/2n5/8/8/8 w - - 0 1",
            "8/8/8/3n4/2n5/8/8/K7 w - - 0 1",
            "8/8/8/3n4/2n5/8/8/K7 w - - 0 1",
            "7n/5n2/8/8/8/8/8/7K w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }

    #[test]
    pub fn white_knight_attacking_true() {
        // set up a list of FENs with BK and WN such that BK *is* under attack
        let fens = vec![
            "7k/5N2/6N1/8/8/8/8/8 w - - 0 1",
            "7k/5N2/4N3/8/8/8/8/8 w - - 0 1",
            "3k4/5N2/4N3/8/8/8/8/8 w - - 0 1",
            "8/5N2/3kN3/8/8/8/8/8 w - - 0 1",
            "8/5N2/4N3/4k3/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/1N6/2N5/k7 w - - 0 1",
            "8/8/8/8/8/8/N1N5/k7 w - - 0 1",
            "8/8/8/8/8/8/5N2/5N1k w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_knight_attacking_true() {
        // set up a list of FENs with WK and BN such that WK *is* under attack
        let fens = vec![
            "7K/5n2/6n1/8/8/8/8/8 w - - 0 1",
            "7K/5n2/8/4n3/8/8/8/8 w - - 0 1",
            "8/5n2/7K/5n2/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/6n1/5n2/7K w - - 0 1",
            "8/8/8/8/8/6n1/8/5n1K w - - 0 1",
            "8/8/8/8/8/8/2n5/K1n5 w - - 0 1",
            "8/8/8/8/8/1n6/2n5/K7 w - - 0 1",
            "8/8/8/8/8/nn6/8/K7 w - - 0 1",
            "8/8/1n6/3n4/2K5/8/8/8 w - - 0 1",
            "8/8/1n6/3n4/8/2K5/8/8 w - - 0 1",
            "8/8/1n6/3n4/1K6/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_horizontal_vertical_attacking_false() {
        // set up a list of FENs with BK R and Q such that BK *isn't* under attack
        let fens = vec![
            "7k/8/8/8/8/1R6/2QR4/8 w - - 0 1",
            "5k2/8/8/8/8/1R6/2QR4/8 w - - 0 1",
            "k7/8/8/8/8/1R6/2QR4/8 w - - 0 1",
            "8/8/8/8/8/1R6/2QR4/k7 w - - 0 1",
            "8/8/8/8/8/1R6/2QR4/4k3 w - - 0 1",
            "8/8/8/8/8/1R1Q4/3R4/7k w - - 0 1",
            "8/8/8/8/7k/1R1Q4/3R4/8 w - - 0 1",
            "8/8/8/1R6/7k/3R4/4Q3/8 w - - 0 1",
            "8/8/8/1R6/4Q1pk/3R4/8/8 w - - 0 1",
            "8/7k/6p1/1R6/4Q3/3R4/8/8 w - - 0 1",
            "8/4k3/4p3/1R6/4Q3/3R4/8/8 w - - 0 1",
            "1k6/1p6/8/1R6/4Q3/3R4/8/8 w - - 0 1",
            "1k6/1P6/8/1R6/4Q3/3R4/8/8 w - - 0 1",
            "8/7k/8/1R3P2/4Q3/3R4/8/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_horizontal_vertical_attacking_false() {
        // set up a list of FENs with BK, BR and BQ such that WK *isn't* under attack
        let fens = vec![
            "K7/2qr4/8/1r6/8/8/8/8 w - - 0 1",
            "K7/1r6/1q6/1r6/8/8/8/8 w - - 0 1",
            "1r6/K7/2q5/1r6/8/8/8/8 w - - 0 1",
            "1r6/7K/2q5/1r6/8/8/8/8 w - - 0 1",
            "1r6/7K/1r6/6q1/8/8/8/8 w - - 0 1",
            "6r1/7K/6r1/6q1/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/4r1q1/5r2/7K w - - 0 1",
            "8/8/8/8/6r1/4r3/5q2/7K w - - 0 1",
            "8/8/8/8/8/1r6/2q3r1/K7 w - - 0 1",
            "8/8/8/8/1qr5/2r5/8/K7 w - - 0 1",
            "8/8/8/4K3/1qr5/2r5/8/8 w - - 0 1",
            "8/3K4/8/8/1qr5/2r5/8/8 w - - 0 1",
            "8/8/8/8/1qr5/2r5/5K2/8 w - - 0 1",
            "8/8/8/8/1qr5/2r1PK2/8/8 w - - 0 1",
            "8/4K3/3P4/8/1qr5/2r5/8/8 w - - 0 1",
            "8/1K6/1P6/8/1qr5/2r5/8/8 w - - 0 1",
            "8/1K6/1p6/8/1qr5/2r5/8/8 w - - 0 1",
            "8/2K5/2p5/8/1qr5/2r5/8/8 w - - 0 1",
            "8/8/8/8/1qr2pK1/2r5/8/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }

    #[test]
    pub fn white_horizontal_vertical_attacking_true() {
        // set up a list of FENs with WK, WR and WQ such that BK *is* under attack
        let fens = vec![
            "8/8/8/1R6/8/8/4Q3/3R3k w - - 0 1",
            "8/8/8/1R6/8/8/3R4/4Q2k w - - 0 1",
            "8/8/8/8/7k/8/3R4/4Q2R w - - 0 1",
            "4k3/8/8/8/8/8/3R4/4Q2R w - - 0 1",
            "k7/8/8/8/8/8/3R4/Q6R w - - 0 1",
            "k6R/8/8/8/8/8/3R4/5Q2 w - - 0 1",
            "k6R/8/8/8/8/8/3R4/5Q2 w - - 0 1",
            "8/8/8/RR1k4/8/8/8/5Q2 w - - 0 1",
            "3Q4/8/8/3k4/8/3R4/3R4/8 w - - 0 1",
            "3Q4/8/8/3k4/8/6R1/R7/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_horizontal_vertical_attacking_true() {
        // set up a list of FENs with BK, BR and BQ such that WK *is* under attack
        let fens = vec![
            "8/8/8/8/8/8/2qr4/3r3K w - - 0 1",
            "8/8/8/8/8/8/2qr3K/3r4 w - - 0 1",
            "3K4/8/8/8/8/8/2qr4/3r4 w - - 0 1",
            "8/8/8/8/8/8/K1qr4/3r4 w - - 0 1",
            "8/3r2q1/8/8/8/8/3K4/3r4 w - - 0 1",
            "8/K2r2q1/8/8/8/8/8/3r4 w - - 0 1",
            "8/K2r4/8/8/8/8/q7/3r4 w - - 0 1",
            "8/K2r4/8/8/r7/8/q7/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_diagonal_attacking_false() {
        // set up a list of FENs with WK, WB and WQ such that BK *isn't* under attack
        let fens = vec![
            "8/8/8/8/8/QB6/1B6/7k w - - 0 1",
            "7k/8/8/8/8/QB6/8/2B5 w - - 0 1",
            "k7/8/8/4Q3/5B2/1B6/8/8 w - - 0 1",
            "8/8/8/4Q3/5B2/8/k1B5/8 w - - 0 1",
            "8/8/8/4Q3/2k2B2/8/2B5/8 w - - 0 1",
            "8/8/2k5/4QB2/5B2/8/8/8 w - - 0 1",
            "8/2k5/3p4/4QB2/5B2/8/8/8 w - - 0 1",
            "1k6/8/3P4/4QB2/5B2/8/8/8 w - - 0 1",
            "8/8/3Q4/4BB2/3P4/8/8/k7 w - - 0 1",
            "7k/8/3Q1p2/4BB2/8/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_diagonal_attacking_false() {
        // set up a list of FENs with BK, BR and BQ such that WK *isn't* under attack
        let fens = vec![
            "8/8/8/8/8/qb6/1b6/7K w - - 0 1",
            "8/8/2K5/8/8/qb6/1b6/8 w - - 0 1",
            "8/4q3/2K5/4b3/3b4/8/8/8 w - - 0 1",
            "8/4q3/8/8/3bb3/1K6/8/8 w - - 0 1",
            "8/8/8/8/2q1b3/4b3/8/K7 w - - 0 1",
            "8/8/8/8/5q2/3bb3/8/7K w - - 0 1",
            "7K/8/8/2bq4/8/3b4/8/8 w - - 0 1",
            "7K/8/5p2/2b1q3/8/3b4/8/8 w - - 0 1",
            "7K/8/5P2/2b1q3/8/3b4/8/8 w - - 0 1",
            "K7/8/2P5/2b5/4q3/3b4/8/8 w - - 0 1",
            "8/8/8/2b1q3/8/2Pb4/8/K7 w - - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }

    #[test]
    pub fn white_diagonal_attacking_true() {
        // set up a list of FENs with WK, WB and WQ such that BK *is* under attack
        let fens = vec![
            "8/8/8/4Q3/4BB2/8/8/7k w - - 0 1",
            "8/1Q6/8/8/4BB2/8/8/7k w - - 0 1",
            "8/1Q6/8/8/4BB2/8/7k/8 w - - 0 1",
            "7k/1Q6/8/4B3/4B3/8/8/8 w - - 0 1",
            "7k/8/3B4/8/4B3/8/1Q6/8 w - - 0 1",
            "3k4/8/8/5B2/7B/3Q4/8/8 w - - 0 1",
            "8/8/8/2B2k2/4B3/3Q4/8/8 w - - 0 1",
            "8/8/8/2B5/4B3/3Q4/8/1k6 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);

            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);

            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == true);
        }
    }

    #[test]
    pub fn black_diagonal_attacking_true() {
        // set up a list of FENs with BK, BB and BQ such that WK *is* under attack
        let fens = vec![
            "8/8/8/2b5/4b3/3q4/8/6K1 w - - 0 1",
            "8/8/8/4b3/3qb3/8/8/K7 w - - 0 1",
            "7K/8/8/4b3/3qb3/8/8/8 w - - 0 1",
            "K7/8/8/4b3/3qb3/8/8/8 w - - 0 1",
            "8/8/1K6/4b3/3qb3/8/8/8 w - - 0 1",
            "8/8/2b2K2/8/3b4/8/8/q7 w - - 0 1",
            "8/6b1/2b5/8/3K4/8/8/q7 w - - 0 1",
        ];
        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);
            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == true);
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
            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);
            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == true);
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
            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);
            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == true);
        }
    }

    #[test]
    pub fn white_pawn_attacking_false() {
        let fens = vec![
            "8/8/8/2k5/2P1P3/8/8/8 w - - 0 1",
            "8/8/8/8/2k5/8/1PP5/8 w - - 0 1",
            "8/8/8/8/2k5/2P5/1P6/8 w - - 0 1",
            "8/8/8/8/8/2P1k3/4P3/8 w - - 0 1",
            "8/8/8/8/8/4k3/2P1P3/8 w - - 0 1",
            "4k3/4P3/2P5/8/8/8/8/8 w - - 0 1",
            "7k/2P3P1/8/8/8/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::Black);
            assert!(checker.is_sq_attacked(&board, king_sq, Colour::White) == false);
        }
    }

    #[test]
    pub fn black_pawn_attacking_false() {
        let fens = vec![
            "8/2p1p3/8/8/3K4/8/8/8 w - - 0 1",
            "8/2p1p3/8/3K4/8/8/8/8 w - - 0 1",
            "8/2pKp3/8/8/8/8/8/8 w - - 0 1",
            "3K4/2p1p3/8/8/8/8/8/8 w - - 0 1",
            "8/8/8/8/3p4/2pK4/8/8 w - - 0 1",
            "8/8/8/8/3p4/1Kp5/8/8 w - - 0 1",
            "8/8/8/2K5/2pp4/8/8/8 w - - 0 1",
            "8/8/8/1K6/2pp4/8/8/8 w - - 0 1",
            "8/8/8/4K3/2pp4/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            print!("fen={}", fen);
            let parsed_fen = fen::get_position(&fen);
            let board = Board::from_fen(&parsed_fen);
            let checker = AttackChecker::new();
            let king_sq = board.get_king_sq(Colour::White);
            assert!(checker.is_sq_attacked(&board, king_sq, Colour::Black) == false);
        }
    }
}
