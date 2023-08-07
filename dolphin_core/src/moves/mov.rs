use crate::board::piece::Role;
use crate::board::square::Square;
use crate::moves::move_types::*;
use std::fmt;

//
// bit flags for Move (u32)
// ---- ---- ---- ---- ---- ---- --XX XXXX  From Square
// ---- ---- ---- ---- ---- XXXX XX-- ----  To Square
// ---- ---- ---- ---- XXXX ---- ---- ----  Move Type
// XXXX XXXX XXXX XXXX ---- ---- ---- ----  Score

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Move(u32);

pub type Score = i16;

const MV_MASK_FROM_SQ: u32 = 0x0000_003F;
const MV_MASK_TO_SQ: u32 = 0x0000_0FC0;
const MV_MASK_MOVE_TYPE: u32 = 0x0000_F000;
const MV_MASK_SCORE: u32 = 0xFFFF_0000;

const MV_SHIFT_FROM_SQ: usize = 0;
const MV_SHIFT_TO_SQ: usize = 6;
const MV_SHIFT_MOVE_TYPE: usize = 12;
const MV_SHIFT_SCORE: usize = 16;

impl Move {
    /// Encodes a Quiet move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the to square
    ///
    pub const fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Move {
        encode(from_sq, to_sq, QUIET)
    }

    pub const fn encode_move_capture(from_sq: Square, to_sq: Square) -> Move {
        encode(from_sq, to_sq, CAPTURE)
    }

    pub const fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_role: Role,
    ) -> Move {
        let mt = match promotion_role {
            Role::Knight => PROMOTE_KNIGHT_QUIET,
            Role::Bishop => PROMOTE_BISHOP_QUIET,
            Role::Rook => PROMOTE_ROOK_QUIET,
            Role::Queen => PROMOTE_QUEEN_QUIET,
            _ => panic!("Invalid promotion piece"),
        };
        encode(from_sq, to_sq, mt)
    }

    pub const fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_role: Role,
    ) -> Move {
        let mt = match promotion_role {
            Role::Knight => PROMOTE_KNIGHT_CAPTURE,
            Role::Bishop => PROMOTE_BISHOP_CAPTURE,
            Role::Rook => PROMOTE_ROOK_CAPTURE,
            Role::Queen => PROMOTE_QUEEN_CAPTURE,
            _ => panic!("Invalid promotion piece"),
        };
        encode(from_sq, to_sq, mt)
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub const fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move {
        encode(from_sq, to_sq, EN_PASSANT)
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from squareClone,
    /// * `to_sq`           - the to square
    ///
    pub const fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Move {
        encode(from_sq, to_sq, DOUBLE_PAWN)
    }

    /// Encodes a White King-side castle move
    ///
    pub const fn encode_move_castle_kingside_white() -> Move {
        encode(Square::E1, Square::G1, KING_CASTLE)
    }

    /// Encodes a Black King-side castle move
    ///
    pub const fn encode_move_castle_kingside_black() -> Move {
        encode(Square::E8, Square::G8, KING_CASTLE)
    }

    /// Encodes a White Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_white() -> Move {
        encode(Square::E1, Square::C1, QUEEN_CASTLE)
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_black() -> Move {
        encode(Square::E8, Square::C8, QUEEN_CASTLE)
    }

    pub fn decode_from_square(&self) -> Square {
        let num = ((self.0 & MV_MASK_FROM_SQ) >> MV_SHIFT_FROM_SQ) as u8;
        Square::new(num)
    }

    pub const fn decode_to_square(&self) -> Square {
        let num = ((self.0 & MV_MASK_TO_SQ) >> MV_SHIFT_TO_SQ) as u8;
        Square::new(num)
    }

    pub const fn decode_move_type(&self) -> MoveType {
        ((self.0 & MV_MASK_MOVE_TYPE) >> MV_SHIFT_MOVE_TYPE) as MoveType
    }

    pub const fn get_score(&self) -> Score {
        ((self.0 & MV_MASK_SCORE) >> MV_SHIFT_SCORE) as Score
    }

    pub const fn decode_promotion_role(&self) -> Role {
        let mt = Self::decode_move_type(self);

        match mt {
            PROMOTE_KNIGHT_QUIET | PROMOTE_KNIGHT_CAPTURE => Role::Knight,
            PROMOTE_BISHOP_QUIET | PROMOTE_BISHOP_CAPTURE => Role::Bishop,
            PROMOTE_ROOK_QUIET | PROMOTE_ROOK_CAPTURE => Role::Rook,
            PROMOTE_QUEEN_QUIET | PROMOTE_QUEEN_CAPTURE => Role::Queen,
            _ => panic!("Invalid promotion piece"),
        }
    }

    pub const fn is_capture(&self) -> bool {
        let move_type = self.decode_move_type();
        is_capture(move_type)
    }

    pub const fn is_en_passant(&self) -> bool {
        let move_type = self.decode_move_type();
        is_en_passant(move_type)
    }

    pub const fn is_castle(&self) -> bool {
        let move_type = self.decode_move_type();
        is_castle(move_type)
    }

    pub const fn is_promote(&self) -> bool {
        let move_type = self.decode_move_type();
        is_promotion(move_type)
    }

    pub const fn is_quiet(&self) -> bool {
        let move_type = self.decode_move_type();
        is_quiet(move_type)
    }

    pub const fn is_queen_castle(&self) -> bool {
        let move_type = self.decode_move_type();
        is_queen_castle(move_type)
    }

    pub const fn is_king_castle(&self) -> bool {
        let move_type = self.decode_move_type();
        is_king_castle(move_type)
    }

    pub fn is_double_pawn(&self) -> bool {
        let move_type = self.decode_move_type();
        is_double_pawn(move_type)
    }

    pub fn set_score(&mut self, score: Score) {
        let sc: u32 = (score as u32) << MV_SHIFT_SCORE;
        self.0 |= sc;
    }

    pub fn print_move(&self) {
        println!(
            "From {:?}, To {:?}",
            self.decode_from_square(),
            self.decode_to_square()
        );
    }
}

const fn encode(from_sq: Square, to_sq: Square, move_type: MoveType) -> Move {
    let mut mv = 0;

    mv |= ((from_sq as u32) << MV_SHIFT_FROM_SQ) & MV_MASK_FROM_SQ;
    mv |= ((to_sq as u32) << MV_SHIFT_TO_SQ) & MV_MASK_TO_SQ;
    mv |= ((move_type as u32) << MV_SHIFT_MOVE_TYPE) & MV_MASK_MOVE_TYPE;

    Move(mv)
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
            QUIET => "Quiet",
            DOUBLE_PAWN => "DoublePawn",
            KING_CASTLE => "KingCastle",
            QUEEN_CASTLE => "QueenCastle",
            CAPTURE => "Capture",
            EN_PASSANT => "En Passant",
            PROMOTE_KNIGHT_QUIET => "Promote Knight Quiet",
            PROMOTE_BISHOP_QUIET => "Promote Bishop Quiet",
            PROMOTE_ROOK_QUIET => "Promote Rook Quiet",
            PROMOTE_QUEEN_QUIET => "Promote Queen Quiet",
            PROMOTE_KNIGHT_CAPTURE => "Promote Knight Capture",
            PROMOTE_BISHOP_CAPTURE => "Promote Bishop Capture",
            PROMOTE_ROOK_CAPTURE => "Promote Rook Capture",
            PROMOTE_QUEEN_CAPTURE => "Promnote Queen Capture",
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
        encode(Square::A1, Square::A1, QUIET)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::piece::Role;
    use crate::board::square::Square;
    use crate::moves::mov::Move;

    #[test]
    pub fn set_get_score() {
        // negative score
        let mut score = -12345;
        let mut mv = Move::encode_move_quiet(Square::A1, Square::A2);
        mv.set_score(score);

        let mut retr_score = mv.get_score();
        assert_eq!(retr_score, score);

        // positive score
        score = 12345;
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
        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
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
        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
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
        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
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
        let target_promotions = [Role::Bishop, Role::Knight, Role::Rook, Role::Queen];

        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                for role in target_promotions.iter() {
                    let mv = Move::encode_move_with_promotion(*from_sq, *to_sq, *role);

                    assert!(mv.is_promote());
                    assert!(!mv.is_capture());

                    let decoded_role = mv.decode_promotion_role();
                    assert_eq!(decoded_role, *role);

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
        let target_promotions = [Role::Bishop, Role::Knight, Role::Rook, Role::Queen];

        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                for role in target_promotions.iter() {
                    let mv = Move::encode_move_with_promotion_capture(*from_sq, *to_sq, *role);

                    assert!(mv.is_promote());
                    assert!(mv.is_capture());

                    let decoded_role = mv.decode_promotion_role();
                    assert_eq!(decoded_role, *role);

                    let decoded_from_sq = mv.decode_from_square();
                    let decoded_to_sq = mv.decode_to_square();

                    assert_eq!(decoded_from_sq, *from_sq);
                    assert_eq!(decoded_to_sq, *to_sq);
                }
            }
        }
    }
}
