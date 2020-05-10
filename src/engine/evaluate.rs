use components::bitboard;
use components::board;
use components::board::Board;
use components::piece::Colour;
use components::piece::Piece;
use components::square::Square;
use core::core_traits::ArrayAccessor;

// Values for piece square arrays are taken from
// https://www.chessprogramming.org/Simplified_Evaluation_Function

static PAWN_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

static KNIGHT_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

static BISHOP_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

static ROOK_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

static QUEEN_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

static KING_SQ_VALUE: [i8; board::NUM_SQUARES] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

static KING_SQ_ENDGAME_VALUE: [i8; board::NUM_SQUARES] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];

static MIRROR_VALUE: [i8; board::NUM_SQUARES] = [
    56, 57, 58, 59, 60, 61, 62, 63, 48, 49, 50, 51, 52, 53, 54, 55, 40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23,
    8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7,
];

pub fn evaluate_board(board: &Board, side_to_move: Colour) -> i32 {
    let material = board.get_material();

    // material
    let mut score = (material.0.wrapping_sub(material.1)) as i32;

    // piece positions
    score = score + evaluate_piece_positions(board);

    if side_to_move == Colour::White {
        score
    } else {
        -score
    }
}

fn evaluate_piece_positions(board: &Board) -> i32 {
    // set up bitboards needed
    let white_pawn_bb = board.get_piece_bitboard(Piece::WhitePawn);
    let white_knight_bb = board.get_piece_bitboard(Piece::WhiteKnight);
    let white_bishop_bb = board.get_piece_bitboard(Piece::WhiteBishop);
    let white_rook_bb = board.get_piece_bitboard(Piece::WhiteRook);
    let white_queen_bb = board.get_piece_bitboard(Piece::WhiteQueen);
    let white_king_bb = board.get_piece_bitboard(Piece::WhiteKing);

    let black_pawn_bb = board.get_piece_bitboard(Piece::BlackPawn);
    let black_knight_bb = board.get_piece_bitboard(Piece::BlackKnight);
    let black_bishop_bb = board.get_piece_bitboard(Piece::BlackBishop);
    let black_rook_bb = board.get_piece_bitboard(Piece::BlackRook);
    let black_queen_bb = board.get_piece_bitboard(Piece::BlackQueen);
    let black_king_bb = board.get_piece_bitboard(Piece::BlackKing);

    let mut score: i32 = 0;

    // evaluate piece locations
    score += eval_white_piece_on_square(white_pawn_bb, &PAWN_SQ_VALUE);
    score += eval_white_piece_on_square(white_bishop_bb, &BISHOP_SQ_VALUE);
    score += eval_white_piece_on_square(white_knight_bb, &KNIGHT_SQ_VALUE);
    score += eval_white_piece_on_square(white_rook_bb, &ROOK_SQ_VALUE);
    score += eval_white_piece_on_square(white_queen_bb, &QUEEN_SQ_VALUE);
    score += eval_white_piece_on_square(white_king_bb, &KING_SQ_VALUE);

    score -= eval_black_piece_on_square(black_pawn_bb, &PAWN_SQ_VALUE);
    score -= eval_black_piece_on_square(black_bishop_bb, &BISHOP_SQ_VALUE);
    score -= eval_black_piece_on_square(black_knight_bb, &KNIGHT_SQ_VALUE);
    score -= eval_black_piece_on_square(black_rook_bb, &ROOK_SQ_VALUE);
    score -= eval_black_piece_on_square(black_queen_bb, &QUEEN_SQ_VALUE);
    score -= eval_black_piece_on_square(black_king_bb, &KING_SQ_VALUE);

    return score;
}

fn eval_white_piece_on_square(pce_bb: u64, values: &[i8]) -> i32 {
    let mut score: i32 = 0;
    let mut bb = pce_bb;

    while bb != 0 {
        let square = bitboard::pop_1st_bit(&mut bb);
        let off = map_square_to_array_offset(square);
        score += values[off] as i32;
    }
    score
}

fn eval_black_piece_on_square(pce_bb: u64, values: &[i8]) -> i32 {
    let mut score: i32 = 0;
    let mut bb = pce_bb;

    while bb != 0 {
        let square = bitboard::pop_1st_bit(&mut bb);
        let off = map_square_to_array_offset(square);
        score += values[MIRROR_VALUE[off] as usize] as i32;
    }
    score
}

fn map_square_to_array_offset(sq: Square) -> usize {
    let off = sq.to_offset();
    return MIRROR_VALUE[off] as usize;
}

#[cfg(test)]
mod tests {
    use components::piece::Colour;
    use engine::position::Position;
    use input::fen;

    #[test]
    pub fn evaluate_sample_white_position() {
        let fen = "k7/8/1P3B2/P6P/3Q4/1N6/3K4/7R w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let pos = Position::new(parsed_fen);

        let score = super::evaluate_board(pos.board(), Colour::White);
        assert_eq!(score, 2365);

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
        let pos = Position::new(parsed_fen);

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

// match role {
//     PieceRole::Pawn => 100,
//     PieceRole::Knight => 320,
//     PieceRole::Bishop => 330,
//     PieceRole::Rook => 500,
//     PieceRole::Queen => 900,
//     PieceRole::King => 20000,
// }
