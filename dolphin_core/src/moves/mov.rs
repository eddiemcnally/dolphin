use std::ops::{Shl, Shr};

use crate::board::piece::Piece;
use crate::board::square::Square;
use crate::board::square::*;
use crate::board::types::ToInt;
use crate::moves::move_list::MoveList;

pub type Move = u64;

// Move type bit maps
//
// XX XX XX XX -- -- -- --      Move score (i32)
// -- -- -- -- -- XX -- --      MoveType
// -- -- -- -- -- -- XX --      To Square
// -- -- -- -- -- -- -- XX      From Square

const MV_MASK_MV_TYPE: u64 = 0x0000_0000_00FF_0000;
const MV_MASK_TO_SQ: u64 = 0x0000_0000_0000_FF00;
const MV_MASK_FROM_SQ: u64 = 0x0000_0000_0000_00FF;

const MV_BIT_SHFT_SCORE: usize = 32;
const MV_BIT_SHFT_MOVE_TYPE: usize = 16;
const MV_BIT_SHFT_TO_SQ: usize = 8;
const MV_BIT_SHFT_FROM_SQ: usize = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MoveType(u8);

pub const MOVE_TYPE_QUIET: MoveType = MoveType(0);
pub const MOVE_TYPE_DOUBLE_PAWN: MoveType = MoveType(1);
pub const MOVE_TYPE_KING_CASTLE: MoveType = MoveType(2);
pub const MOVE_TYPE_QUEEN_CASTLE: MoveType = MoveType(3);
pub const MOVE_TYPE_CAPTURE: MoveType = MoveType(4);
pub const MOVE_TYPE_EN_PASSANT: MoveType = MoveType(5);
pub const MOVE_TYPE_PROMOTE_KNIGHT_QUIET: MoveType = MoveType(6);
pub const MOVE_TYPE_PROMOTE_BISHOP_QUIET: MoveType = MoveType(7);
pub const MOVE_TYPE_PROMOTE_ROOK_QUIET: MoveType = MoveType(8);
pub const MOVE_TYPE_PROMOTE_QUEEN_QUIET: MoveType = MoveType(9);
pub const MOVE_TYPE_PROMOTE_KNIGHT_CAPTURE: MoveType = MoveType(10);
pub const MOVE_TYPE_PROMOTE_BISHOP_CAPTURE: MoveType = MoveType(11);
pub const MOVE_TYPE_PROMOTE_ROOK_CAPTURE: MoveType = MoveType(12);
pub const MOVE_TYPE_PROMOTE_QUEEN_CAPTURE: MoveType = MoveType(13);

static MV_WHITE_KING_CASTLE: Move = Move::new(SQUARE_E1, SQUARE_G1, MOVE_TYPE_KING_CASTLE);

impl Default for MoveType {
    fn default() -> MoveType {
        MOVE_TYPE_QUIET
    }
}

pub trait MoveTrait {
    fn new(from_sq: Square, to_sq: Square, move_type: MoveType) -> Self;
    fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Move;

    fn encode_move_capture(from_sq: Square, to_sq: Square) -> Move;

    fn encode_move_with_promotion(from_sq: Square, to_sq: Square, promotion_piece: Piece) -> Move;

    fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Move;

    fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move;

    fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Move;
    fn encode_move_castle_kingside_white() -> Move;
    fn encode_move_castle_kingside_black() -> Move;
    fn encode_move_castle_queenside_white() -> Move;
    fn encode_move_castle_queenside_black() -> Move;

    fn decode_from_square(&self) -> Square;
    fn decode_to_square(&self) -> Square;
    fn decode_promotion_piece(&self) -> Piece;
    fn decode_move_type(&self) -> MoveType;

    fn is_capture(&self) -> bool;
    fn is_en_passant(&self) -> bool;
    fn is_castle(&self) -> bool;
    fn is_promote(&self) -> bool;
    fn is_quiet(&self) -> bool;
    fn is_queen_castle(&self) -> bool;
    fn is_king_castle(&self) -> bool;
    fn is_double_pawn(&self) -> bool;

    fn set_score(&mut self, score: i32);
    fn get_score(&self) -> i32;

    fn print_move(&self);
}

impl MoveTrait for Move {
    fn new(from_sq: Square, to_sq: Square, move_type: MoveType) -> Move {
        let fsq = (from_sq.to_u8() as u64).shl(MV_BIT_SHFT_FROM_SQ) & MV_MASK_FROM_SQ;
        let tsq = (to_sq.to_u8() as u64).shl(MV_BIT_SHFT_TO_SQ) & MV_MASK_TO_SQ;
        let mt = (move_type.0 as u64).shl(MV_BIT_SHFT_MOVE_TYPE) & MV_MASK_MV_TYPE;

        fsq | tsq | mt
    }

    /// Encodes a Quiet move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the to square
    ///
    fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Move {
        Self::new(from_sq, to_sq, MOVE_TYPE_QUIET)
    }

    fn encode_move_capture(from_sq: Square, to_sq: Square) -> Move {
        Self::new(from_sq, to_sq, MOVE_TYPE_CAPTURE)
    }

    fn encode_move_with_promotion(from_sq: Square, to_sq: Square, promotion_piece: Piece) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        let mt = match promotion_piece {
            Piece::Knight => MOVE_TYPE_PROMOTE_KNIGHT_QUIET,
            Piece::Bishop => MOVE_TYPE_PROMOTE_BISHOP_QUIET,
            Piece::Rook => MOVE_TYPE_PROMOTE_ROOK_QUIET,
            Piece::Queen => MOVE_TYPE_PROMOTE_QUEEN_QUIET,
            _ => panic!("Invalid promotion piece"),
        };
        Self::new(from_sq, to_sq, mt)
    }

    fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        let mt = match promotion_piece {
            Piece::Knight => MOVE_TYPE_PROMOTE_KNIGHT_CAPTURE,
            Piece::Bishop => MOVE_TYPE_PROMOTE_BISHOP_CAPTURE,
            Piece::Rook => MOVE_TYPE_PROMOTE_ROOK_CAPTURE,
            Piece::Queen => MOVE_TYPE_PROMOTE_QUEEN_CAPTURE,
            _ => panic!("Invalid promotion piece"),
        };
        Self::new(from_sq, to_sq, mt)
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );
        Self::new(from_sq, to_sq, MOVE_TYPE_EN_PASSANT)
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from squareClone,
    /// * `to_sq`           - the to square
    ///
    fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );
        Self::new(from_sq, to_sq, MOVE_TYPE_DOUBLE_PAWN)
    }

    /// Encodes a White King-side castle move
    ///
    fn encode_move_castle_kingside_white() -> Move {
        Self::new(SQUARE_E1, SQUARE_G1, MOVE_TYPE_KING_CASTLE)
    }

    /// Encodes a Black King-side castle move
    ///
    fn encode_move_castle_kingside_black() -> Move {
        Self::new(SQUARE_E8, SQUARE_G8, MOVE_TYPE_KING_CASTLE)
    }

    /// Encodes a White Queen-side castle move
    ///
    fn encode_move_castle_queenside_white() -> Move {
        Self::new(SQUARE_E1, SQUARE_C1, MOVE_TYPE_QUEEN_CASTLE)
    }

    /// Encodes a Black Queen-side castle move
    ///
    fn encode_move_castle_queenside_black() -> Move {
        Self::new(SQUARE_E8, SQUARE_C8, MOVE_TYPE_QUEEN_CASTLE)
    }

    fn decode_from_square(&self) -> Square {
        let s = ((*self as u64) & MV_MASK_FROM_SQ).shr(MV_BIT_SHFT_FROM_SQ);
        Square::new(s as u8).unwrap()
    }

    fn decode_to_square(&self) -> Square {
        let s = ((*self as u64) & MV_MASK_TO_SQ).shr(MV_BIT_SHFT_TO_SQ);
        Square::new(s as u8).unwrap()
    }

    fn decode_move_type(&self) -> MoveType {
        let s = ((*self as u64) & MV_MASK_MV_TYPE).shr(MV_BIT_SHFT_MOVE_TYPE);
        //assert!(s < MOVE_TYPE_MAX as u64, "Invalid move type value {}", s);
        MoveType(s as u8)
    }

    fn decode_promotion_piece(&self) -> Piece {
        let mt = Self::decode_move_type(self);

        match mt {
            MOVE_TYPE_PROMOTE_KNIGHT_QUIET | MOVE_TYPE_PROMOTE_KNIGHT_CAPTURE => Piece::Knight,
            MOVE_TYPE_PROMOTE_BISHOP_QUIET | MOVE_TYPE_PROMOTE_BISHOP_CAPTURE => Piece::Bishop,
            MOVE_TYPE_PROMOTE_ROOK_QUIET | MOVE_TYPE_PROMOTE_ROOK_CAPTURE => Piece::Rook,
            MOVE_TYPE_PROMOTE_QUEEN_QUIET | MOVE_TYPE_PROMOTE_QUEEN_CAPTURE => Piece::Queen,
            _ => panic!("Invalid promotion piece"),
        }
    }

    fn is_capture(&self) -> bool {
        let mt = Self::decode_move_type(self);

        matches!(
            mt,
            MOVE_TYPE_CAPTURE
                | MOVE_TYPE_EN_PASSANT
                | MOVE_TYPE_PROMOTE_KNIGHT_CAPTURE
                | MOVE_TYPE_PROMOTE_BISHOP_CAPTURE
                | MOVE_TYPE_PROMOTE_ROOK_CAPTURE
                | MOVE_TYPE_PROMOTE_QUEEN_CAPTURE
        )
    }

    fn is_en_passant(&self) -> bool {
        Self::decode_move_type(self) == MOVE_TYPE_EN_PASSANT
    }

    fn is_castle(&self) -> bool {
        let mt = Self::decode_move_type(self);
        if mt == MOVE_TYPE_KING_CASTLE || mt == MOVE_TYPE_QUEEN_CASTLE {
            return true;
        }
        false
    }
    fn is_promote(&self) -> bool {
        let mt = Self::decode_move_type(self);

        matches!(
            mt,
            MOVE_TYPE_PROMOTE_KNIGHT_QUIET
                | MOVE_TYPE_PROMOTE_KNIGHT_CAPTURE
                | MOVE_TYPE_PROMOTE_BISHOP_QUIET
                | MOVE_TYPE_PROMOTE_BISHOP_CAPTURE
                | MOVE_TYPE_PROMOTE_ROOK_QUIET
                | MOVE_TYPE_PROMOTE_ROOK_CAPTURE
                | MOVE_TYPE_PROMOTE_QUEEN_QUIET
                | MOVE_TYPE_PROMOTE_QUEEN_CAPTURE
        )
    }

    fn is_quiet(&self) -> bool {
        Self::decode_move_type(self) == MOVE_TYPE_QUIET
    }

    fn is_queen_castle(&self) -> bool {
        Self::decode_move_type(self) == MOVE_TYPE_QUEEN_CASTLE
    }

    fn is_king_castle(&self) -> bool {
        Self::decode_move_type(self) == MOVE_TYPE_KING_CASTLE
    }

    fn is_double_pawn(&self) -> bool {
        Self::decode_move_type(self) == MOVE_TYPE_DOUBLE_PAWN
    }

    fn set_score(&mut self, score: i32) {
        let s: u64 = score.shl(MV_BIT_SHFT_SCORE).try_into().unwrap();
        *self |= s;
    }

    fn get_score(&self) -> i32 {
        self.shr(MV_BIT_SHFT_SCORE) as i32
    }

    fn print_move(&self) {
        println!(
            "From {:?}, To {:?}",
            self.decode_from_square(),
            self.decode_to_square()
        );
    }
}
pub fn print_move_list(move_list: &MoveList) {
    for mov in move_list.iter() {
        mov.print_move();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::piece::Piece;
    use crate::board::square;
    use crate::board::square::*;
    use crate::moves::mov::{Move, MoveTrait};

    #[test]
    pub fn encode_decode_king_white_castle() {
        let mv = Move::encode_move_castle_kingside_white();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, SQUARE_E1);
        assert_eq!(decoded_to_sq, SQUARE_G1);

        assert!(mv.is_king_castle());
        assert!(mv.is_castle());
        assert!(!mv.is_queen_castle());

        assert!(!mv.is_quiet());
        assert!(!mv.is_capture());
        assert!(!mv.is_double_pawn());
        assert!(!mv.is_promote());
    }

    #[test]
    pub fn encode_decode_queen_white_castle() {
        let mv = Move::encode_move_castle_queenside_white();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, SQUARE_E1);
        assert_eq!(decoded_to_sq, SQUARE_C1);

        assert!(!mv.is_king_castle());
        assert!(mv.is_castle());
        assert!(mv.is_queen_castle());

        assert!(!mv.is_quiet());
        assert!(!mv.is_capture());
        assert!(!mv.is_double_pawn());
        assert!(!mv.is_promote());
    }

    #[test]
    pub fn encode_decode_king_black_castle() {
        let mv = Move::encode_move_castle_kingside_black();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, SQUARE_E8);
        assert_eq!(decoded_to_sq, SQUARE_G8);

        assert!(mv.is_king_castle());
        assert!(mv.is_castle());
        assert!(!mv.is_queen_castle());

        assert!(!mv.is_quiet());
        assert!(!mv.is_capture());
        assert!(!mv.is_double_pawn());
        assert!(!mv.is_promote());
    }

    #[test]
    pub fn encode_decode_queen_black_castle() {
        let mv = Move::encode_move_castle_queenside_black();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, SQUARE_E8);
        assert_eq!(decoded_to_sq, SQUARE_C8);

        assert!(!mv.is_king_castle());
        assert!(mv.is_castle());
        assert!(mv.is_queen_castle());

        assert!(!mv.is_quiet());
        assert!(!mv.is_capture());
        assert!(!mv.is_double_pawn());
        assert!(!mv.is_promote());
    }

    #[test]
    pub fn encode_decode_quiet_move() {
        for from_sq in square::iterator() {
            for to_sq in square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                // encode
                let mv = Move::encode_move_quiet(*from_sq, *to_sq);

                assert!(mv.is_quiet());
                assert!(!mv.is_capture());
                assert!(!mv.is_castle());
                assert!(!mv.is_double_pawn());
                assert!(!mv.is_promote());

                let decoded_from_sq = mv.decode_from_square();
                let decoded_to_sq = mv.decode_to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_double_pawn_first_move() {
        for from_sq in square::iterator() {
            for to_sq in square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                // encode
                let mv = Move::encode_move_double_pawn_first(*from_sq, *to_sq);
                assert!(mv.is_double_pawn());

                assert!(!mv.is_quiet());
                assert!(!mv.is_capture());
                assert!(!mv.is_castle());
                assert!(!mv.is_promote());

                let decoded_from_sq = mv.decode_from_square();
                let decoded_to_sq = mv.decode_to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_en_passant() {
        for from_sq in square::iterator() {
            for to_sq in square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                let mv = Move::encode_move_en_passant(*from_sq, *to_sq);

                assert!(mv.is_en_passant());
                assert!(mv.is_capture());
                assert!(!mv.is_castle());
                assert!(!mv.is_double_pawn());
                assert!(!mv.is_promote());

                let decoded_from_sq = mv.decode_from_square();
                let decoded_to_sq = mv.decode_to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_promotion_move_non_capture() {
        let target_promotions = [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen];

        for from_sq in square::iterator() {
            for to_sq in square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                for pce in target_promotions.iter() {
                    let mv = Move::encode_move_with_promotion(*from_sq, *to_sq, *pce);

                    assert!(mv.is_promote());
                    assert!(!mv.is_capture());

                    let decoded_pce = mv.decode_promotion_piece();
                    assert_eq!(decoded_pce, *pce);

                    let decoded_from_sq = mv.decode_from_square();
                    let decoded_to_sq = mv.decode_to_square();

                    assert_eq!(decoded_from_sq, *from_sq);
                    assert_eq!(decoded_to_sq, *to_sq);
                }
            }
        }
    }

    #[test]
    pub fn encode_decode_promotion_move_capture() {
        let target_promotions = [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen];

        for from_sq in square::iterator() {
            for to_sq in square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                for pce in target_promotions.iter() {
                    let mv = Move::encode_move_with_promotion_capture(*from_sq, *to_sq, *pce);

                    assert!(mv.is_promote());
                    assert!(mv.is_capture());

                    let decoded_piece = mv.decode_promotion_piece();
                    assert_eq!(decoded_piece, *pce);

                    let decoded_from_sq = mv.decode_from_square();
                    let decoded_to_sq = mv.decode_to_square();

                    assert_eq!(decoded_from_sq, *from_sq);
                    assert_eq!(decoded_to_sq, *to_sq);
                }
            }
        }
    }
}
