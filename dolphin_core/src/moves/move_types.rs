pub type MoveType = u8;

//
// see https://www.chessprogramming.org/Encoding_Moves
//
pub const QUIET: u8 = 0b0000_0000;
pub const DOUBLE_PAWN: u8 = 0b0000_0001;
pub const KING_CASTLE: u8 = 0b0000_0010;
pub const QUEEN_CASTLE: u8 = 0b0000_0011;
pub const CAPTURE: u8 = 0b0000_0100;
pub const EN_PASSANT: u8 = 0b0000_0101;
pub const PROMOTE_KNIGHT_QUIET: u8 = 0b0000_1000;
pub const PROMOTE_BISHOP_QUIET: u8 = 0b0000_1001;
pub const PROMOTE_ROOK_QUIET: u8 = 0b0000_1010;
pub const PROMOTE_QUEEN_QUIET: u8 = 0b0000_1011;
pub const PROMOTE_KNIGHT_CAPTURE: u8 = 0b0000_1100;
pub const PROMOTE_BISHOP_CAPTURE: u8 = 0b0000_1101;
pub const PROMOTE_ROOK_CAPTURE: u8 = 0b0000_1110;
pub const PROMOTE_QUEEN_CAPTURE: u8 = 0b0000_1111;

const MOVE_TYPE_MASK_CAPTURE: u8 = 0b0000_0100;
const MOVE_TYPE_MASK_PROMOTE: u8 = 0b0000_1000;

pub const fn is_promotion(move_type: MoveType) -> bool {
    move_type & MOVE_TYPE_MASK_PROMOTE != 0
}

pub const fn is_capture(move_type: MoveType) -> bool {
    move_type & MOVE_TYPE_MASK_CAPTURE != 0
}
pub const fn is_en_passant(move_type: MoveType) -> bool {
    move_type == EN_PASSANT
}

pub const fn is_castle(move_type: MoveType) -> bool {
    is_king_castle(move_type) || is_queen_castle(move_type)
}

pub const fn is_queen_castle(move_type: MoveType) -> bool {
    move_type == QUEEN_CASTLE
}

pub const fn is_king_castle(move_type: MoveType) -> bool {
    move_type == KING_CASTLE
}

pub const fn is_quiet(move_type: MoveType) -> bool {
    move_type == QUIET
}
pub const fn is_double_pawn(move_type: MoveType) -> bool {
    move_type == DOUBLE_PAWN
}
