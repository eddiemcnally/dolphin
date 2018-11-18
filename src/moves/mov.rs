use board::piece::Colour;
use board::piece::Piece;
use board::square::Square;
use std::ops::Shl;
use std::ops::Shr;

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

//#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Mov(u16);

impl Mov {
    /// Returns a Quite move
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the tovsquare
    ///
    pub fn encode_quiet(from_sq: Square, to_sq: Square) -> Mov {
        let from = Square::to_u8(from_sq) as u16;
        let to = Square::to_u8(to_sq) as u16;

        let mut mv: u16 = from.shl(MV_SHFT_FROM_SQ);
        mv |= mv & MV_MASK_FROM_SQ;

        mv |= to.shl(MV_SHFT_TO_SQ);
        mv |= mv & MV_MASK_TO_SQ;

        return Mov(mv);
    }

    /// Returns a Promotion move (no capture)
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    /// * 'promotion_piece  - the target promotion piece 
    pub fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);

        match promotion_piece {
            Piece::WKnight => mov.0 = mov.0 | MV_FLG_PROMOTE_KNIGHT,
            Piece::BKnight => mov.0 = mov.0 | MV_FLG_PROMOTE_KNIGHT,
            Piece::WBishop => mov.0 = mov.0 | MV_FLG_PROMOTE_BISHOP,
            Piece::BBishop => mov.0 = mov.0 | MV_FLG_PROMOTE_BISHOP,
            Piece::WRook => mov.0 = mov.0 | MV_FLG_PROMOTE_ROOK,
            Piece::BRook => mov.0 = mov.0 | MV_FLG_PROMOTE_ROOK,
            Piece::WQueen => mov.0 = mov.0 | MV_FLG_PROMOTE_QUEEN,
            Piece::BQueen => mov.0 = mov.0 | MV_FLG_PROMOTE_QUEEN,
            _ => panic!("Invalid promotion type"),
        }
        return mov;
    }

    /// Returns a new promotion move with Capture
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    /// * 'promotion_piece  - the target promotion piece 
    pub fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Mov {
        let mut mov = Mov::encode_move_with_promotion(from_sq, to_sq, promotion_piece);
        mov.0 = mov.0 | MV_FLG_BIT_CAPTURE;
        return mov;
    }

    /// Returns an En Passant move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    pub fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);
        mov.0 = mov.0 | MV_FLG_EN_PASS;
        return mov;
    }

    pub fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);
        mov.0 = mov.0 | MV_FLG_DOUBLE_PAWN;
        return mov;
    }

    pub fn encode_move_castle_kingside_white() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e1, Square::g1);
        mov.0 = mov.0 | MV_FLG_KING_CASTLE;
        return mov;
    }

    pub fn encode_move_castle_kingside_black() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e8, Square::g8);
        mov.0 = mov.0 | MV_FLG_KING_CASTLE;
        return mov;
    }

    pub fn encode_move_castle_queenside_white() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e1, Square::c1);
        mov.0 = mov.0 | MV_FLG_QUEEN_CASTLE;
        return mov;
    }

    pub fn encode_move_castle_queenside_black() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e8, Square::c8);
        mov.0 = mov.0 | MV_FLG_QUEEN_CASTLE;
        return mov;
    }

    pub fn decode_from_square(mv: Mov) -> Square {
        let sq = (mv.0 & MV_MASK_FROM_SQ).shr(MV_SHFT_FROM_SQ);
        return Square::from_u8(sq as u8);
    }

    pub fn decode_to_square(mv: Mov) -> Square {
        let sq = (mv.0 & MV_MASK_TO_SQ).shr(MV_SHFT_TO_SQ);
        return Square::from_u8(sq as u8);
    }

    pub fn decode_promotion_piece(mv: Mov, side: Colour) -> Piece {
        let masked = mv.0 & MV_MASK_FLAGS;

        match side {
            Colour::White => match masked {
                MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => return Piece::WKnight,
                MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => return Piece::WBishop,
                MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => return Piece::WQueen,
                MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => return Piece::WRook,
                _ => panic!("Invalid WHITE promotion piece"),
            },
            Colour::Black => match masked {
                MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => return Piece::BKnight,
                MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => return Piece::BBishop,
                MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => return Piece::BQueen,
                MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => return Piece::BRook,
                _ => panic!("Invalid BLACK promotion piece"),
            },
        }
    }

    pub fn move_is_quiet(mv: Mov) -> bool {
        let m = mv.0 & MV_MASK_FLAGS;
        return m == MV_FLG_QUIET;
    }

    pub fn move_is_capture(mv: Mov) -> bool {
        return (mv.0 & MV_FLG_BIT_CAPTURE) != 0;
    }

    pub fn move_is_promote(mv: Mov) -> bool {
        return (mv.0 & MV_FLG_BIT_PROMOTE) != 0;
    }

    pub fn move_is_en_passant(mv: Mov) -> bool {
        return (mv.0 & MV_FLG_EN_PASS) != 0;
    }
}

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
