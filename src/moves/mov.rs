use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square::Square;
use std::ops::Shl;
use std::ops::Shr;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Mov(u16);

impl Mov {
    /// Encodes a Quiet move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the to square
    ///
    pub fn encode_quiet(from_sq: &Square, to_sq: &Square) -> Mov {
        let from = Square::to_u8(*from_sq) as u16;
        let to = Square::to_u8(*to_sq) as u16;

        let mut mv: u16 = from.shl(MV_SHFT_FROM_SQ);
        mv |= mv & MV_MASK_FROM_SQ;

        mv |= to.shl(MV_SHFT_TO_SQ);
        mv |= mv & MV_MASK_TO_SQ;

        Mov(mv)
    }

    /// Encodes a Promotion move that doesn't involve a captured piece
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    /// * 'promotion_piece' - the target promotion piece
    ///
    pub fn encode_move_with_promotion(
        from_sq: &Square,
        to_sq: &Square,
        promotion_piece: &Piece,
    ) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);

        let mask: u16;
        match promotion_piece.role() {
            PieceRole::Knight => mask = MV_FLG_PROMOTE_KNIGHT,
            PieceRole::Bishop => mask = MV_FLG_PROMOTE_BISHOP,
            PieceRole::Rook => mask = MV_FLG_PROMOTE_ROOK,
            PieceRole::Queen => mask = MV_FLG_PROMOTE_QUEEN,
            _ => panic!("Invalid promotion type"),
        }
        mov.0 = mov.0 | mask;
        mov
    }

    /// Encodes a Promotion move that involves a captured piece
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    /// * 'promotion_piece' - the target promotion piece
    ///
    pub fn encode_move_with_promotion_capture(
        from_sq: &Square,
        to_sq: &Square,
        promotion_piece: &Piece,
    ) -> Mov {
        let mut mov = Mov::encode_move_with_promotion(from_sq, to_sq, promotion_piece);
        mov.0 = mov.0 | MV_FLG_BIT_CAPTURE;
        mov
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_en_passant(from_sq: &Square, to_sq: &Square) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);
        mov.0 = mov.0 | MV_FLG_EN_PASS;
        mov
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_double_pawn_first(from_sq: &Square, to_sq: &Square) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);
        mov.0 = mov.0 | MV_FLG_DOUBLE_PAWN;
        mov
    }

    /// Encodes a White King-side castle move
    ///
    pub fn encode_move_castle_kingside_white() -> Mov {
        // todo: this can be determined at compile time, so fix this
        let mut mov = Mov::encode_quiet(&Square::e1, &Square::g1);
        mov.0 = mov.0 | MV_FLG_KING_CASTLE;
        mov
    }

    /// Encodes a Black King-side castle move
    ///
    pub fn encode_move_castle_kingside_black() -> Mov {
        let mut mov = Mov::encode_quiet(&Square::e8, &Square::g8);
        mov.0 = mov.0 | MV_FLG_KING_CASTLE;
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_white() -> Mov {
        let mut mov = Mov::encode_quiet(&Square::e1, &Square::c1);
        mov.0 = mov.0 | MV_FLG_QUEEN_CASTLE;
        mov
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_black() -> Mov {
        let mut mov = Mov::encode_quiet(&Square::e8, &Square::c8);
        mov.0 = mov.0 | MV_FLG_QUEEN_CASTLE;
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn decode_from_square(mv: &Mov) -> Square {
        let sq = (mv.0 & MV_MASK_FROM_SQ).shr(MV_SHFT_FROM_SQ);
        Square::from_u8(sq as u8)
    }

    ///
    /// Decodes the "to" square from the Move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn decode_to_square(mv: &Mov) -> Square {
        let sq = (mv.0 & MV_MASK_TO_SQ).shr(MV_SHFT_TO_SQ);
        Square::from_u8(sq as u8)
    }

    ///
    /// Decodes the promotion piece from the move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    /// * `side`       - the side/colour
    ///
    pub fn decode_promotion_piece(mv: &Mov, side: Colour) -> Piece {
        let masked = mv.0 & MV_MASK_FLAGS;

        let role = match masked {
            MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => PieceRole::Knight,
            MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => PieceRole::Bishop,
            MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => PieceRole::Queen,
            MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => PieceRole::Rook,
            _ => panic!("Invalid promotion piece"),
        };

        Piece::new(role, side)
    }

    /// Tests the given move to see if it is a Quiet move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_quiet(mv: &Mov) -> bool {
        let m = mv.0 & MV_MASK_FLAGS;
        m == MV_FLG_QUIET
    }

    /// Tests the given move to see if it is a Capture move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_capture(mv: &Mov) -> bool {
        (mv.0 & MV_FLG_BIT_CAPTURE) != 0
    }

    /// Tests the given move to see if it is a Promotion move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_promote(mv: &Mov) -> bool {
        (mv.0 & MV_FLG_BIT_PROMOTE) != 0
    }

    /// Tests the given move to see if it is an En Passant move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_en_passant(mv: &Mov) -> bool {
        (mv.0 & MV_FLG_EN_PASS) != 0
    }

    /// Tests the given move to see if it is a Castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_castle(mv: &Mov) -> bool {
        Mov::is_king_castle(mv) || Mov::is_queen_castle(mv)
    }

    /// Tests the given move to see if it is an Queen-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_queen_castle(mv: &Mov) -> bool {
        (mv.0 & MV_FLG_QUEEN_CASTLE) != 0
    }

    /// Tests the given move to see if it is an King-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_king_castle(mv: &Mov) -> bool {
        (mv.0 & MV_FLG_KING_CASTLE) != 0
    }

    /// Tests the given move to see if it is a Double pawn first move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_double_pawn(mv: &Mov) -> bool {
        (mv.0 & MV_FLG_DOUBLE_PAWN) != 0
    }
}

// bitmap for type Move
// See http://chessprogramming.wikispaces.com/Encoding+Moves
//
//  ---- ---- --11 1111      To Square
//  ---- 1111 11-- ----      From Square
//  0000 ---- ---- ----      Quiet move
//  0001 ---- ---- ----      Double Pawn push
//  0010 ---- ---- ----      King Castle
//  0011 ---- ---- ----      Queen Castle
//  0100 ---- ---- ----      Capture
//  0101 ---- ---- ----      En Passant Capture
//  1000 ---- ---- ----      Promotion Knight
//  1001 ---- ---- ----      Promotion Bishop
//  1010 ---- ---- ----      Promotion Rook
//  1011 ---- ---- ----      Promotion Queen
//  1100 ---- ---- ----      Promotion Knight Capture
//  1101 ---- ---- ----      Promotion Bishop Capture
//  1110 ---- ---- ----      Promotion Rook Capture
//  1111 ---- ---- ----      Promotion Queen Capture

const MV_FLG_QUIET: u16 = 0x0000;
const MV_FLG_DOUBLE_PAWN: u16 = 0x1000;
const MV_FLG_KING_CASTLE: u16 = 0x2000;
const MV_FLG_QUEEN_CASTLE: u16 = 0x3000;
const MV_FLG_CAPTURE: u16 = 0x4000;
const MV_FLG_EN_PASS: u16 = 0x5000;
const MV_FLG_PROMOTE_KNIGHT: u16 = 0x8000;
const MV_FLG_PROMOTE_BISHOP: u16 = 0x9000;
const MV_FLG_PROMOTE_ROOK: u16 = 0xA000;
const MV_FLG_PROMOTE_QUEEN: u16 = 0xB000;
const MV_FLG_PROMOTE_KNIGHT_CAPTURE: u16 = 0xC000;
const MV_FLG_PROMOTE_BISHOP_CAPTURE: u16 = 0xD000;
const MV_FLG_PROMOTE_ROOK_CAPTURE: u16 = 0xE000;
const MV_FLG_PROMOTE_QUEEN_CAPTURE: u16 = 0xF000;

const MV_FLG_BIT_PROMOTE: u16 = 0x8000;
const MV_FLG_BIT_CAPTURE: u16 = 0x4000;

const MV_SHFT_TO_SQ: u16 = 0;
const MV_SHFT_FROM_SQ: u16 = 6;

const MV_MASK_TO_SQ: u16 = 0x003F;
const MV_MASK_FROM_SQ: u16 = 0x0FC0;
const MV_MASK_FLAGS: u16 = 0xF000;

#[cfg(test)]
pub mod tests {}
