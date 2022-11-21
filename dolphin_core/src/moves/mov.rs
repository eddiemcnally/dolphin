use crate::board::piece::Piece;
use crate::board::square::Square;
use num_enum::TryFromPrimitive;
use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Move {
    m: u64,
}

pub type Score = i32;

// bitmap for Move (u64)
// -- -- -- -- -- -- -- XX      From Square
// -- -- -- -- -- -- XX --      To Square
// -- -- -- -- -- XX -- --      Move Type
// -- -- -- -- XX -- -- --      Unused
// XX XX XX XX -- -- -- --      Score

const MV_MASK_FROM_SQ: u64 = 0x0000_0000_0000_00_FF;
const MV_MASK_TO_SQ: u64 = 0x0000_0000_0000_FF00;
const MV_MASK_MOVE_TYPE: u64 = 0x0000_0000_00FF_00_00;

const MV_SHIFT_FROM_SQ: usize = 0;
const MV_SHIFT_TO_SQ: usize = 8;
const MV_SHIFT_MOVE_TYPE: usize = 16;
const MV_SHIFT_SCORE: usize = 32;

//
// see https://www.chessprogramming.org/Encoding_Moves
//
#[derive(Eq, PartialEq, Copy, Clone, Hash, TryFromPrimitive)]
#[repr(u8)]
#[rustfmt::skip]
pub enum MoveType {
    Quiet                   = 0b0000_0000,
    DoublePawn              = 0b0000_0001,
    KingCastle              = 0b0000_0010,
    QueenCastle             = 0b0000_0011,
    Capture                 = 0b0000_0100,
    EnPassant               = 0b0000_0101,
    PromoteKnightQuiet      = 0b0000_1000,
    PromoteBishopQuiet      = 0b0000_1001,
    PromoteRookQuiet        = 0b0000_1010,
    PromoteQueenQuiet       = 0b0000_1011,
    PromoteKnightCapture    = 0b0000_1100,
    PromoteBishopCapture    = 0b0000_1101,
    PromoteRookCapture      = 0b0000_1110,
    PromoteQueenCapture     = 0b0000_1111,
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
        encode(from_sq, to_sq, MoveType::Quiet)
    }

    pub fn encode_move_capture(from_sq: Square, to_sq: Square) -> Move {
        encode(from_sq, to_sq, MoveType::Capture)
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
            Piece::Knight => MoveType::PromoteKnightQuiet,
            Piece::Bishop => MoveType::PromoteBishopQuiet,
            Piece::Rook => MoveType::PromoteRookQuiet,
            Piece::Queen => MoveType::PromoteQueenQuiet,
            _ => panic!("Invalid promotion piece"),
        };
        encode(from_sq, to_sq, mt)
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
            Piece::Knight => MoveType::PromoteKnightCapture,
            Piece::Bishop => MoveType::PromoteBishopCapture,
            Piece::Rook => MoveType::PromoteRookCapture,
            Piece::Queen => MoveType::PromoteQueenCapture,
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
    pub fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );
        encode(from_sq, to_sq, MoveType::EnPassant)
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
        encode(from_sq, to_sq, MoveType::DoublePawn)
    }

    /// Encodes a White King-side castle move
    ///
    pub const fn encode_move_castle_kingside_white() -> Move {
        encode(Square::E1, Square::G1, MoveType::KingCastle)
    }

    /// Encodes a Black King-side castle move
    ///
    pub const fn encode_move_castle_kingside_black() -> Move {
        encode(Square::E8, Square::G8, MoveType::KingCastle)
    }

    /// Encodes a White Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_white() -> Move {
        encode(Square::E1, Square::C1, MoveType::QueenCastle)
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_black() -> Move {
        encode(Square::E8, Square::C8, MoveType::QueenCastle)
    }

    pub fn decode_from_square(self) -> Square {
        let s: u8 = ((self.m & MV_MASK_FROM_SQ) >> MV_SHIFT_FROM_SQ) as u8;
        Square::new(s).unwrap()
    }

    pub fn decode_to_square(self) -> Square {
        let s: u8 = ((self.m & MV_MASK_TO_SQ) >> MV_SHIFT_TO_SQ) as u8;
        Square::new(s).unwrap()
    }

    pub fn decode_move_type(self) -> MoveType {
        let s: u8 = ((self.m & MV_MASK_MOVE_TYPE) >> MV_SHIFT_MOVE_TYPE) as u8;
        MoveType::try_from(s).unwrap()
    }

    pub fn decode_promotion_piece(self) -> Piece {
        let mt = Self::decode_move_type(self);

        match mt {
            MoveType::PromoteKnightQuiet | MoveType::PromoteKnightCapture => Piece::Knight,
            MoveType::PromoteBishopQuiet | MoveType::PromoteBishopCapture => Piece::Bishop,
            MoveType::PromoteRookQuiet | MoveType::PromoteRookCapture => Piece::Rook,
            MoveType::PromoteQueenQuiet | MoveType::PromoteQueenCapture => Piece::Queen,
            _ => panic!("Invalid promotion piece"),
        }
    }

    pub fn is_capture(self) -> bool {
        Self::decode_move_type(self).is_capture()
    }

    pub fn is_en_passant(self) -> bool {
        Self::decode_move_type(self).is_en_passant()
    }

    pub fn is_castle(self) -> bool {
        Self::decode_move_type(self).is_castle()
    }

    pub fn is_promote(self) -> bool {
        Self::decode_move_type(self).is_promotion()
    }

    pub fn is_quiet(self) -> bool {
        Self::decode_move_type(self).is_quiet()
    }

    pub fn is_queen_castle(self) -> bool {
        Self::decode_move_type(self).is_queen_castle()
    }

    pub fn is_king_castle(self) -> bool {
        Self::decode_move_type(self).is_king_castle()
    }

    pub fn is_double_pawn(self) -> bool {
        Self::decode_move_type(self).is_double_pawn()
    }

    pub fn set_score(&mut self, score: Score) {
        let s: u64 = (score as u64) << MV_SHIFT_SCORE;

        self.m |= s;
    }

    pub fn get_score(&self) -> Score {
        (self.m >> MV_SHIFT_SCORE) as Score
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
    let from: u64 = ((from_sq.to_offset() as u64) << MV_SHIFT_FROM_SQ) as u64 & MV_MASK_FROM_SQ;
    let to: u64 = ((to_sq.to_offset() as u64) << MV_SHIFT_TO_SQ) as u64 & MV_MASK_TO_SQ;
    let move_type: u64 = ((move_type as u64) << MV_SHIFT_MOVE_TYPE) as u64 & MV_MASK_MOVE_TYPE;

    Move {
        m: from | to | move_type,
    }
}

impl MoveType {
    const MOVE_TYPE_MASK_CAPTURE: u8 = 0b0000_0100;
    const MOVE_TYPE_MASK_PROMOTE: u8 = 0b0000_1000;

    const fn is_promotion(&self) -> bool {
        *self as u8 & MoveType::MOVE_TYPE_MASK_PROMOTE != 0
    }

    const fn is_capture(&self) -> bool {
        *self as u8 & MoveType::MOVE_TYPE_MASK_CAPTURE != 0
    }
    fn is_en_passant(&self) -> bool {
        *self == MoveType::EnPassant
    }

    fn is_castle(&self) -> bool {
        self.is_king_castle() || self.is_queen_castle()
    }

    fn is_queen_castle(&self) -> bool {
        *self == MoveType::QueenCastle
    }

    fn is_king_castle(&self) -> bool {
        *self == MoveType::KingCastle
    }

    fn is_quiet(&self) -> bool {
        *self == MoveType::Quiet
    }
    fn is_double_pawn(&self) -> bool {
        *self == MoveType::DoublePawn
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
            MoveType::Quiet => "Quiet",
            MoveType::DoublePawn => "DoublePawn",
            MoveType::KingCastle => "KingCastle",
            MoveType::QueenCastle => "QueenCastle",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "En Passant",
            MoveType::PromoteKnightQuiet => "Promote Knight Quiet",
            MoveType::PromoteBishopQuiet => "Promote Bishop Quiet",
            MoveType::PromoteRookQuiet => "Promote Rook Quiet",
            MoveType::PromoteQueenQuiet => "Promote Queen Quiet",
            MoveType::PromoteKnightCapture => "Promote Knight Capture",
            MoveType::PromoteBishopCapture => "Promote Bishop Capture",
            MoveType::PromoteRookCapture => "Promote Rook Capture",
            MoveType::PromoteQueenCapture => "Promnote Queen Capture",
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
        encode(Square::A1, Square::A1, MoveType::Quiet)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::piece::Piece;
    use crate::board::square::Square;
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
        let target_promotions = [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen];

        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
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

        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
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
