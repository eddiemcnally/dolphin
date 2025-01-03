// Values for piece square arrays are taken from
// https://www.chessprogramming.org/Simplified_Evaluation_Function

use crate::board::colour::Colour;
use crate::board::game_board::Board;
use crate::board::piece::Piece;

use crate::moves::mov::Score;

#[rustfmt::skip]
const PAWN_SQ_VALUE: [i8; Board::NUM_SQUARES] = [
    0,      0,      0,      0,      0,      0,      0,      0,
    5,      10,     10,     -20,    -20,    10,     10,     5, 
    5,      -5,     -10,    0,      0,      -10,    -5,     5, 
    0,      0,      0,      20,     20,     0,      0,      0,  
    5,      5,      10,     25,     25,     10,     5,      5,  
    10,     10,     20,     30,     30,     20,     10,     10, 
    50,     50,     50,     50,     50,     50,     50,     50, 
    0,      0,      0,      0,      0,      0,      0,      0, 
];

#[rustfmt::skip]
const KNIGHT_SQ_VALUE: [i8; Board::NUM_SQUARES] = [
    -50,    -40,    -30,    -30,    -30,    -30,    -40,    -50,
    -40,    -20,    0,      5,      5,      0,      -20,    -40, 
    -30,    5,      10,     15,     15,     10,     5,      -30, 
    -30,    0,      15,     20,     20,     15,     0,      -30, 
    -30,    5,      15,     20,     20,     15,     5,      -30, 
    -30,    0,      10,     15,     15,     10,     0,      -30, 
    -40,    -20,    0,      0,      0,      0,      -20,    -40, 
    -50,    -40,    -30,    -30,    -30,    -30,    -40,    -50, 
];

#[rustfmt::skip]
const BISHOP_SQ_VALUE: [i8; Board::NUM_SQUARES] = [
    -20,    -10,    -10,    -10,    -10,    -10,    -10,    -20,
    -10,    5,      0,      0,      0,      0,      5,      -10, 
    -10,    10,     10,     10,     10,     10,     10,     -10, 
    -10,    0,      10,     10,     10,     10,     0,      -10, 
    -10,    5,      5,      10,     10,     5,      5,      -10, 
    -10,    0,      5,      10,     10,     5,      0,      -10, 
    -10,    0,      0,      0,      0,      0,      0,      -10, 
    -20,    -10,    -10,    -10,    -10,    -10,    -10,    -20, 
];

#[rustfmt::skip]
const ROOK_SQ_VALUE: [i8; Board::NUM_SQUARES] = [
    0,      0,      0,      5,      5,      0,      0,      0,
    -5,     0,      0,      0,      0,      0,      0,      -5, 
    -5,     0,      0,      0,      0,      0,      0,      -5, 
    -5,     0,      0,      0,      0,      0,      0,      -5, 
    -5,     0,      0,      0,      0,      0,      0,      -5, 
    -5,     0,      0,      0,      0,      0,      0,      -5, 
    5,      10,     10,     10,     10,     10,     10,     5, 
    0,      0,      0,      0,      0,      0,      0,      0, 
];

#[rustfmt::skip]
const QUEEN_SQ_VALUE: [i8; Board::NUM_SQUARES] = [
    -20,    -10,    -10,    -5,     -5,     -10,    -10,    -20,
    -10,    0,      5,      0,      0,      0,      0,      -10, 
    -10,    5,      5,      5,      5,      5,      0,      -10, 
    0,      0,      5,      5,      5,      5,      0,      -5, 
    -5,     0,      5,      5,      5,      5,      0,      -5, 
    -10,    0,      5,      5,      5,      5,      0,      -10,
    -10,    0,      0,      0,      0,      0,      0,      -10, 
    -20,    -10,    -10,    -5,     -5,     -10,    -10,    -20, 
];

#[rustfmt::skip]
const KING_SQ_VALUE: [i8; Board::NUM_SQUARES] = [
    20,     30,     10,     0,      0,      10,     30,     20,
    20,     20,     0,      0,      0,      0,      20,     20, 
    -10,    -20,    -20,    -20,    -20,    -20,    -20,    -10, 
    -20,    -30,    -30,    -40,    -40,    -30,    -30,    -20, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
];

static PIECE_MAP: [(Piece, &[i8; Board::NUM_SQUARES]); 6] = [
    (Piece::Pawn, &PAWN_SQ_VALUE),
    (Piece::Bishop, &BISHOP_SQ_VALUE),
    (Piece::Knight, &KNIGHT_SQ_VALUE),
    (Piece::Rook, &ROOK_SQ_VALUE),
    (Piece::Queen, &QUEEN_SQ_VALUE),
    (Piece::King, &KING_SQ_VALUE),
];

pub fn evaluate_board(board: &Board, side_to_move: Colour) -> Score {
    let mut score = board.get_net_material();

    // white
    PIECE_MAP.iter().for_each(|(pce, map)| {
        board
            .get_piece_bitboard(pce, &Colour::White)
            .iterator()
            .for_each(|sq| score += map[sq.as_index()] as Score);
    });

    // black (note negative score, and mirror'ed table lookup)
    PIECE_MAP.iter().for_each(|(pce, map)| {
        board
            .get_piece_bitboard(pce, &Colour::Black)
            .iterator()
            .for_each(|sq| score -= map[63 - sq.as_index()] as Score);
    });

    if side_to_move == Colour::White {
        score
    } else {
        -score
    }
}

#[cfg(test)]
mod tests {
    use crate::board::colour::Colour;
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::io::fen;
    use crate::position::attack_checker::AttackChecker;
    use crate::position::game_position::Position;
    use crate::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn evaluate_sample_white_position() {
        let fen = "k7/8/1P3B2/P6P/3Q4/1N6/3K4/7R w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let score = super::evaluate_board(pos.board(), Colour::White);
        assert_eq!(score, 2365);

        // Pawn = 100,
        // Knight = 320,
        // Bishop = 330,
        // Rook = 500,
        // Queen = 900,
        // King = 20000,

        // white material = 22350
        //  - 3x pawns      = 300
        //  - 1x knight     = 320
        //  - 1x rook       = 500
        //  - 1x bishop     = 330
        //  - 1x queen      = 900
        //  - 1x king       = 20000
        //
        // black material = 20000
        //  - 1x king       = 20000

        //
        // white piece positions = 35
        //  - Pawns: 5 + 10 + 5 = 20
        //  - Knight: 5         = 5
        //  - Queen: 5          = 5
        //  - King: 0           = 0
        //  - Bishop: 5         = 5
        //  - Rook: 0           = 0
        //
        // Black position pieces = 20
        //
        // expected score   = (22350 - 20000) + (35 - 20)
        //                  = 2365
    }

    #[test]
    pub fn evaluate_sample_black_position() {
        let fen = "1k6/1pp3q1/5b2/1n6/7p/8/3K4/8 b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let score = super::evaluate_board(pos.board(), Colour::White);
        assert_eq!(score, -1915);

        // white material = 20000
        //  - 1x king       = 20000
        //
        // black material = 21850
        //  - 3x pawns      = 300
        //  - 1x knight     = 320
        //  - 1x bishop     = 330
        //  - 1x queen      = 900
        //  - 1x king       = 20000
        //
        // Black piece positions    = 65
        //  - Pawns: 10 + 10 + 5 = 25
        //  - Knight: 5          = 5
        //  - Queen: 0           = 0
        //  - King: 30           = 30
        //  - Bishop: 5          = 5
        //
        // White position pieces = 0
        //
        // expected score   = (20000 - 21850) + (0 - 60)
        //                  = -1915
    }
}
