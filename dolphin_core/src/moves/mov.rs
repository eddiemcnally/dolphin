use crate::board::piece::Piece;
use crate::board::square::Square;
use crate::core::types::ToInt;
use std::fmt;
use std::ops::{Shl, Shr};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Move {
    flags: u64,
}

// Move bit map
//
// XX XX XX XX -- -- -- --      Move score (i32)
// -- -- -- -- -- XX -- --      MoveType
// -- -- -- -- -- -- XX --      To Square
// -- -- -- -- -- -- -- XX      From Square

const MV_MASK_SCORE: u64 = 0xFFFF_FFFF_0000_0000;
const MV_MASK_MV_TYPE: u64 = 0x0000_0000_00FF_0000;
const MV_MASK_TO_SQ: u64 = 0x0000_0000_0000_FF00;
const MV_MASK_FROM_SQ: u64 = 0x0000_0000_0000_00FF;

const MV_BIT_SHFT_SCORE: usize = 32;
const MV_BIT_SHFT_MOVE_TYPE: usize = 16;
const MV_BIT_SHFT_TO_SQ: usize = 8;
const MV_BIT_SHFT_FROM_SQ: usize = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MoveType(u8);

impl MoveType {
    pub const QUIET: MoveType = MoveType(0);
    pub const DOUBLE_PAWN: MoveType = MoveType(1);
    pub const KING_CASTLE: MoveType = MoveType(2);
    pub const QUEEN_CASTLE: MoveType = MoveType(3);
    pub const CAPTURE: MoveType = MoveType(4);
    pub const EN_PASSANT: MoveType = MoveType(5);
    pub const PROMOTE_KNIGHT_QUIET: MoveType = MoveType(6);
    pub const PROMOTE_BISHOP_QUIET: MoveType = MoveType(7);
    pub const PROMOTE_ROOK_QUIET: MoveType = MoveType(8);
    pub const PROMOTE_QUEEN_QUIET: MoveType = MoveType(9);
    pub const PROMOTE_KNIGHT_CAPTURE: MoveType = MoveType(10);
    pub const PROMOTE_BISHOP_CAPTURE: MoveType = MoveType(11);
    pub const PROMOTE_ROOK_CAPTURE: MoveType = MoveType(12);
    pub const PROMOTE_QUEEN_CAPTURE: MoveType = MoveType(13);

    pub fn is_valid_move_type(&self) -> bool {
        matches!(
            *self,
            MoveType::QUIET
                | MoveType::DOUBLE_PAWN
                | MoveType::KING_CASTLE
                | MoveType::QUEEN_CASTLE
                | MoveType::CAPTURE
                | MoveType::EN_PASSANT
                | MoveType::PROMOTE_KNIGHT_QUIET
                | MoveType::PROMOTE_BISHOP_QUIET
                | MoveType::PROMOTE_ROOK_QUIET
                | MoveType::PROMOTE_QUEEN_QUIET
                | MoveType::PROMOTE_KNIGHT_CAPTURE
                | MoveType::PROMOTE_BISHOP_CAPTURE
                | MoveType::PROMOTE_ROOK_CAPTURE
                | MoveType::PROMOTE_QUEEN_CAPTURE
        )
    }
}

impl Move {
    /// Encodes a Quiet move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the to square
    ///
    pub fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Move {
        new(from_sq, to_sq, MoveType::QUIET)
    }

    pub fn encode_move_capture(from_sq: Square, to_sq: Square) -> Move {
        new(from_sq, to_sq, MoveType::CAPTURE)
    }

    pub fn encode_move_with_promotion(
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
            Piece::Knight => MoveType::PROMOTE_KNIGHT_QUIET,
            Piece::Bishop => MoveType::PROMOTE_BISHOP_QUIET,
            Piece::Rook => MoveType::PROMOTE_ROOK_QUIET,
            Piece::Queen => MoveType::PROMOTE_QUEEN_QUIET,
            _ => panic!("Invalid promotion piece"),
        };
        new(from_sq, to_sq, mt)
    }

    pub fn encode_move_with_promotion_capture(
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
            Piece::Knight => MoveType::PROMOTE_KNIGHT_CAPTURE,
            Piece::Bishop => MoveType::PROMOTE_BISHOP_CAPTURE,
            Piece::Rook => MoveType::PROMOTE_ROOK_CAPTURE,
            Piece::Queen => MoveType::PROMOTE_QUEEN_CAPTURE,
            _ => panic!("Invalid promotion piece"),
        };
        new(from_sq, to_sq, mt)
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );
        new(from_sq, to_sq, MoveType::EN_PASSANT)
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from squareClone,
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );
        new(from_sq, to_sq, MoveType::DOUBLE_PAWN)
    }

    /// Encodes a White King-side castle move
    ///
    pub fn encode_move_castle_kingside_white() -> Move {
        new(Square::E1, Square::G1, MoveType::KING_CASTLE)
    }

    /// Encodes a Black King-side castle move
    ///
    pub fn encode_move_castle_kingside_black() -> Move {
        new(Square::E8, Square::G8, MoveType::KING_CASTLE)
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_white() -> Move {
        new(Square::E1, Square::C1, MoveType::QUEEN_CASTLE)
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_black() -> Move {
        new(Square::E8, Square::C8, MoveType::QUEEN_CASTLE)
    }

    pub fn decode_from_square(&self) -> Square {
        let s = ((self.flags as u64) & MV_MASK_FROM_SQ).shr(MV_BIT_SHFT_FROM_SQ);
        Square::new(s as u8).unwrap()
    }

    pub fn decode_to_square(&self) -> Square {
        let s = ((self.flags as u64) & MV_MASK_TO_SQ).shr(MV_BIT_SHFT_TO_SQ);
        Square::new(s as u8).unwrap()
    }

    pub fn decode_move_type(&self) -> MoveType {
        let s = ((self.flags as u64) & MV_MASK_MV_TYPE).shr(MV_BIT_SHFT_MOVE_TYPE);
        let retval = MoveType(s as u8);
        debug_assert!(retval.is_valid_move_type(), "MoveType is invalid");
        retval
    }

    pub fn decode_promotion_piece(&self) -> Piece {
        let mt = Self::decode_move_type(self);

        match mt {
            MoveType::PROMOTE_KNIGHT_QUIET | MoveType::PROMOTE_KNIGHT_CAPTURE => Piece::Knight,
            MoveType::PROMOTE_BISHOP_QUIET | MoveType::PROMOTE_BISHOP_CAPTURE => Piece::Bishop,
            MoveType::PROMOTE_ROOK_QUIET | MoveType::PROMOTE_ROOK_CAPTURE => Piece::Rook,
            MoveType::PROMOTE_QUEEN_QUIET | MoveType::PROMOTE_QUEEN_CAPTURE => Piece::Queen,
            _ => panic!("Invalid promotion piece"),
        }
    }

    pub fn is_capture(&self) -> bool {
        let mt = Self::decode_move_type(self);

        matches!(
            mt,
            MoveType::CAPTURE
                | MoveType::EN_PASSANT
                | MoveType::PROMOTE_KNIGHT_CAPTURE
                | MoveType::PROMOTE_BISHOP_CAPTURE
                | MoveType::PROMOTE_ROOK_CAPTURE
                | MoveType::PROMOTE_QUEEN_CAPTURE
        )
    }

    pub fn is_en_passant(&self) -> bool {
        Self::decode_move_type(self) == MoveType::EN_PASSANT
    }

    pub fn is_castle(&self) -> bool {
        let mt = Self::decode_move_type(self);

        matches!(mt, MoveType::KING_CASTLE | MoveType::QUEEN_CASTLE)
    }

    pub fn is_promote(&self) -> bool {
        let mt = Self::decode_move_type(self);

        matches!(
            mt,
            MoveType::PROMOTE_KNIGHT_QUIET
                | MoveType::PROMOTE_KNIGHT_CAPTURE
                | MoveType::PROMOTE_BISHOP_QUIET
                | MoveType::PROMOTE_BISHOP_CAPTURE
                | MoveType::PROMOTE_ROOK_QUIET
                | MoveType::PROMOTE_ROOK_CAPTURE
                | MoveType::PROMOTE_QUEEN_QUIET
                | MoveType::PROMOTE_QUEEN_CAPTURE
        )
    }

    pub fn is_quiet(&self) -> bool {
        Self::decode_move_type(self) == MoveType::QUIET
    }

    pub fn is_queen_castle(&self) -> bool {
        Self::decode_move_type(self) == MoveType::QUEEN_CASTLE
    }

    pub fn is_king_castle(&self) -> bool {
        Self::decode_move_type(self) == MoveType::KING_CASTLE
    }

    pub fn is_double_pawn(&self) -> bool {
        Self::decode_move_type(self) == MoveType::DOUBLE_PAWN
    }

    pub fn set_score(&mut self, score: i32) {
        let mut s: u64 = score as u64;
        s = s.shl(MV_BIT_SHFT_SCORE);

        self.flags |= s & MV_MASK_SCORE;
    }

    pub fn get_score(&self) -> i32 {
        let s = self.flags.shr(MV_BIT_SHFT_SCORE);
        s as i32
    }

    pub fn print_move(&self) {
        println!(
            "From {:?}, To {:?}",
            self.decode_from_square(),
            self.decode_to_square()
        );
    }
}

#[inline(always)]
fn new(from_sq: Square, to_sq: Square, move_type: MoveType) -> Move {
    let from_sq = (from_sq.to_u8() as u64).shl(MV_BIT_SHFT_FROM_SQ);
    let to_sq = (to_sq.to_u8() as u64).shl(MV_BIT_SHFT_TO_SQ);
    let mt = (move_type.0 as u64).shl(MV_BIT_SHFT_MOVE_TYPE);

    Move {
        flags: from_sq | to_sq | mt,
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        let from_sq = self.decode_from_square();
        let to_sq = self.decode_to_square();

        debug_str.push_str(&format!("[{}", from_sq.file()));
        debug_str.push_str(&format!("{}->", from_sq.rank()));
        debug_str.push_str(&format!("{}", to_sq.file()));
        debug_str.push_str(&format!("{} ", to_sq.rank()));

        let mt = match self.decode_move_type() {
            MoveType::QUIET => "Quiet",
            MoveType::DOUBLE_PAWN => "DoublePawn",
            MoveType::KING_CASTLE => "KingCastle",
            MoveType::QUEEN_CASTLE => "QueenCastle",
            MoveType::CAPTURE => "Capture",
            MoveType::EN_PASSANT => "En Passant",
            MoveType::PROMOTE_KNIGHT_QUIET => "Promote Knight Quiet",
            MoveType::PROMOTE_BISHOP_QUIET => "Promote Bishop Quiet",
            MoveType::PROMOTE_ROOK_QUIET => "Promote Rook Quiet",
            MoveType::PROMOTE_QUEEN_QUIET => "Promnote Queen Quiet",
            MoveType::PROMOTE_KNIGHT_CAPTURE => "Promote Knight Capture",
            MoveType::PROMOTE_BISHOP_CAPTURE => "Promote Bishop Capture",
            MoveType::PROMOTE_ROOK_CAPTURE => "Promote Rook Capture",
            MoveType::PROMOTE_QUEEN_CAPTURE => "Promnote Queen Capture",
            _ => panic!("Invalid move type"),
        };
        debug_str.push_str(&format!(" : {}]", mt));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Default for Move {
    fn default() -> Move {
        new(Square::A1, Square::A1, MoveType::QUIET)
    }
}

impl Default for MoveType {
    fn default() -> MoveType {
        MoveType::QUIET
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::piece::Piece;
    use crate::board::square;
    use crate::board::square::*;
    use crate::moves::mov::Move;

    #[test]
    pub fn set_get_score() {
        // negative score
        let mut score = -1234567;
        let mut mv = Move::encode_move_quiet(Square::A1, Square::A2);
        mv.set_score(score);

        let mut retr_score = mv.get_score();
        assert_eq!(retr_score, score);

        // positive score
        score = 1234567;
        mv = Move::encode_move_quiet(Square::A1, Square::A2);
        mv.set_score(score);

        retr_score = mv.get_score();
        assert_eq!(retr_score, score);
    }

    #[test]
    pub fn encode_decode_king_white_castle() {
        let mv = Move::encode_move_castle_kingside_white();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, Square::E1);
        assert_eq!(decoded_to_sq, Square::G1);

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

        assert_eq!(decoded_from_sq, Square::E1);
        assert_eq!(decoded_to_sq, Square::C1);

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

        assert_eq!(decoded_from_sq, Square::E8);
        assert_eq!(decoded_to_sq, Square::G8);

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

        assert_eq!(decoded_from_sq, Square::E8);
        assert_eq!(decoded_to_sq, Square::C8);

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
