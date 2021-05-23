// Values for piece square arrays are taken from
// https://www.chessprogramming.org/Simplified_Evaluation_Function

use crate::bitboard;
use crate::board;
use crate::board::Board;
use crate::piece::Colour;
use crate::piece::Piece;

#[rustfmt::skip]
static PAWN_SQ_VALUE: [i8; board::NUM_SQUARES] = [
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
static KNIGHT_SQ_VALUE: [i8; board::NUM_SQUARES] = [
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
static BISHOP_SQ_VALUE: [i8; board::NUM_SQUARES] = [
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
static ROOK_SQ_VALUE: [i8; board::NUM_SQUARES] = [
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
static QUEEN_SQ_VALUE: [i8; board::NUM_SQUARES] = [
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
static KING_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    20,     30,     10,     0,      0,      10,     30,     20,
    20,     20,     0,      0,      0,      0,      20,     20, 
    -10,    -20,    -20,    -20,    -20,    -20,    -20,    -10, 
    -20,    -30,    -30,    -40,    -40,    -30,    -30,    -20, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
    -30,    -40,    -40,    -50,    -50,    -40,    -40,    -30, 
];

#[rustfmt::skip]
static KING_SQ_ENDGAME_VALUE: [i8; board::NUM_SQUARES] = [
    -50,    -30,    -30,    -30,    -30,    -30,    -30,    -50,
    -30,    -30,    0,      0,      0,      0,      -30,    -30, 
    -30,    -10,    20,     30,     30,     20,     -10,    -30, 
    -30,    -10,    30,     40,     40,     30,     -10,    -30, 
    -30,    -10,    30,     40,     40,     30,     -10,    -30, 
    -30,    -10,    20,     30,     30,     20,     -10,    -30, 
    -30,    -20,    -10,    0,      0,      -10,    -20,    -30, 
    -50,    -40,    -30,    -20,    -20,    -30,    -40,    -50, 
];

pub fn evaluate_board(board: &Board, side_to_move: Colour) -> i32 {
    let material = board.get_material();

    // material
    let mut score = (material.0.wrapping_sub(material.1)) as i32;

    // piece positions
    let mut board_bb = board.get_bitboard();
    while board_bb != 0 {
        let sq = bitboard::pop_1st_bit(&mut board_bb);
        let pce = board.get_piece_on_square(sq);

        let sq_offset = sq.to_offset();

        score += match pce.unwrap() {
            Piece::WhitePawn => PAWN_SQ_VALUE[sq_offset] as i32,
            Piece::WhiteBishop => BISHOP_SQ_VALUE[sq_offset] as i32,
            Piece::WhiteKnight => KNIGHT_SQ_VALUE[sq_offset] as i32,
            Piece::WhiteRook => ROOK_SQ_VALUE[sq_offset] as i32,
            Piece::WhiteQueen => QUEEN_SQ_VALUE[sq_offset] as i32,
            Piece::WhiteKing => KING_SQ_VALUE[sq_offset] as i32,

            // black scores are negative, offsets are reversed/mirrored
            Piece::BlackPawn => -PAWN_SQ_VALUE[63 - sq_offset] as i32,
            Piece::BlackBishop => -BISHOP_SQ_VALUE[63 - sq_offset] as i32,
            Piece::BlackKnight => -KNIGHT_SQ_VALUE[63 - sq_offset] as i32,
            Piece::BlackRook => -ROOK_SQ_VALUE[63 - sq_offset] as i32,
            Piece::BlackQueen => -QUEEN_SQ_VALUE[63 - sq_offset] as i32,
            Piece::BlackKing => -KING_SQ_VALUE[63 - sq_offset] as i32,
        }
    }

    if side_to_move == Colour::White {
        score
    } else {
        -score
    }
}

#[cfg(test)]
mod tests {
    use crate::fen;
    use crate::occupancy_masks::OccupancyMasks;
    use crate::piece::Colour;
    use crate::position::Position;
    use crate::zobrist_keys::ZobristKeys;

    #[test]
    pub fn evaluate_sample_white_position() {
        let fen = "k7/8/1P3B2/P6P/3Q4/1N6/3K4/7R w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(&zobrist_keys, &occ_masks, parsed_fen);

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
        let parsed_fen = fen::get_position(&fen);
        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(&zobrist_keys, &occ_masks, parsed_fen);

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
