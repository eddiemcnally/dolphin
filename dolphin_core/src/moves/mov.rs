use crate::board::colour::Colour;
use crate::board::piece;
use crate::board::piece::Piece;
use crate::board::square::Square;
use crate::board::square::*;
use crate::moves::move_list::MoveList;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum MoveType {
    Quiet,
    DoublePawn,
    KingCastle,
    QueenCastle,
    Capture,
    EnPassant,
    PromoteKnightQuiet,
    PromoteBishopQuiet,
    PromoteRookQuiet,
    PromoteQueenQuiet,
    PromoteKnightCapture,
    PromoteBishopCapture,
    PromoteRookCapture,
    PromoteQueenCapture,
}

impl Default for MoveType {
    fn default() -> MoveType {
        MoveType::Quiet
    }
}

const WHITE_KING_CASTLE: Mov = Mov {
    from_sq: SQUARE_E1,
    to_sq: SQUARE_G1,
    score: 0,
    move_type: MoveType::KingCastle,
};

const WHITE_QUEEN_CASTLE: Mov = Mov {
    from_sq: SQUARE_E1,
    to_sq: SQUARE_C1,
    score: 0,
    move_type: MoveType::QueenCastle,
};

const BLACK_KING_CASTLE: Mov = Mov {
    from_sq: SQUARE_E8,
    to_sq: SQUARE_G8,
    score: 0,
    move_type: MoveType::KingCastle,
};

const BLACK_QUEEN_CASTLE: Mov = Mov {
    from_sq: SQUARE_E8,
    to_sq: SQUARE_C8,
    score: 0,
    move_type: MoveType::QueenCastle,
};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Mov {
    from_sq: Square,
    to_sq: Square,
    move_type: MoveType,
    score: i32,
}

#[rustfmt::skip]
impl fmt::Debug for Mov {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("[{}", self.from_sq.file()));
        debug_str.push_str(&format!("{}->", self.from_sq.rank()));
        debug_str.push_str(&format!("{}", self.to_sq.file()));
        debug_str.push_str(&format!("{} ", self.to_sq.rank()));

        let mt = match self.move_type() {
            MoveType::Quiet         => "Quiet",
            MoveType::DoublePawn    => "DoubleFirstMove",
            MoveType::KingCastle    => "KingCastle",
            MoveType::QueenCastle   => "QueenCastle",
            MoveType::Capture       => "Capture",
            MoveType::EnPassant     => "EnPassant",
            MoveType::PromoteKnightQuiet    => "PromoteKnightQuiet",
            MoveType::PromoteBishopQuiet    => "PromoteBishopQuiet",
            MoveType::PromoteRookQuiet      => "PromoteRookQuiet",
            MoveType::PromoteQueenQuiet     => "PromoteQueenQuiet",
            MoveType::PromoteKnightCapture  => "PromoteKnightCapture",
            MoveType::PromoteBishopCapture  => "PromoteBishopCapture",
            MoveType::PromoteRookCapture    => "PromoteRookCapture",
            MoveType::PromoteQueenCapture   => "PromoteQueenCapture",
        };
        debug_str.push_str(&format!(" : {}]", mt));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Mov {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Mov {
    /// Encodes a Quiet move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the to square
    ///
    pub fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Mov {
        Mov {
            from_sq,
            to_sq,
            move_type: MoveType::Quiet,
            score: 0,
        }
    }

    pub fn encode_move_capture(from_sq: Square, to_sq: Square) -> Mov {
        Mov {
            from_sq,
            to_sq,
            move_type: MoveType::Capture,
            score: 0,
        }
    }

    pub fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: &'static Piece,
    ) -> Mov {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        let mt = match promotion_piece {
            &piece::WHITE_KNIGHT | &piece::BLACK_KNIGHT => MoveType::PromoteKnightQuiet,
            &piece::WHITE_BISHOP | &piece::BLACK_BISHOP => MoveType::PromoteBishopQuiet,
            &piece::WHITE_ROOK | &piece::BLACK_ROOK => MoveType::PromoteRookQuiet,
            &piece::WHITE_QUEEN | &piece::BLACK_QUEEN => MoveType::PromoteQueenQuiet,
            _ => panic!("Invalid promotion piece"),
        };

        Mov {
            from_sq,
            to_sq,
            score: 0,
            move_type: mt,
        }
    }

    pub fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: &'static Piece,
    ) -> Mov {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        let mt = match promotion_piece {
            &piece::WHITE_KNIGHT | &piece::BLACK_KNIGHT => MoveType::PromoteKnightCapture,
            &piece::WHITE_BISHOP | &piece::BLACK_BISHOP => MoveType::PromoteBishopCapture,
            &piece::WHITE_ROOK | &piece::BLACK_ROOK => MoveType::PromoteRookCapture,
            &piece::WHITE_QUEEN | &piece::BLACK_QUEEN => MoveType::PromoteQueenCapture,
            _ => panic!("Invalid promotion piece"),
        };
        Mov {
            from_sq,
            to_sq,
            score: 0,
            move_type: mt,
        }
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Mov {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        Mov {
            from_sq,
            to_sq,
            score: 0,
            move_type: MoveType::EnPassant,
        }
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from squareClone,
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Mov {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        Mov {
            from_sq,
            to_sq,
            score: 0,
            move_type: MoveType::DoublePawn,
        }
    }

    /// Encodes a White King-side castle move
    ///
    pub const fn encode_move_castle_kingside_white() -> Mov {
        WHITE_KING_CASTLE
    }

    /// Encodes a Black King-side castle move
    ///
    pub const fn encode_move_castle_kingside_black() -> Mov {
        BLACK_KING_CASTLE
    }

    /// Encodes a White Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_white() -> Mov {
        WHITE_QUEEN_CASTLE
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_black() -> Mov {
        BLACK_QUEEN_CASTLE
    }

    pub const fn from_square(&self) -> Square {
        self.from_sq
    }

    ///
    /// Decodes the "to" square from the Move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn to_square(&self) -> Square {
        self.to_sq
    }

    pub fn decode_promotion_piece(&self, colour: Colour) -> &'static Piece {
        if colour == Colour::White {
            match self.move_type {
                MoveType::PromoteKnightQuiet | MoveType::PromoteKnightCapture => {
                    &piece::WHITE_KNIGHT
                }
                MoveType::PromoteBishopQuiet | MoveType::PromoteBishopCapture => {
                    &piece::WHITE_BISHOP
                }
                MoveType::PromoteRookQuiet | MoveType::PromoteRookCapture => &piece::WHITE_ROOK,
                MoveType::PromoteQueenQuiet | MoveType::PromoteQueenCapture => &piece::WHITE_QUEEN,
                _ => panic!("Invalid promotion piece"),
            }
        } else {
            match self.move_type {
                MoveType::PromoteKnightQuiet | MoveType::PromoteKnightCapture => {
                    &piece::BLACK_KNIGHT
                }
                MoveType::PromoteBishopQuiet | MoveType::PromoteBishopCapture => {
                    &piece::BLACK_BISHOP
                }
                MoveType::PromoteRookQuiet | MoveType::PromoteRookCapture => &piece::BLACK_ROOK,
                MoveType::PromoteQueenQuiet | MoveType::PromoteQueenCapture => &piece::BLACK_QUEEN,
                _ => panic!("Invalid promotion piece"),
            }
        }
    }

    pub fn move_type(&self) -> MoveType {
        self.move_type
    }

    /// Tests the given move to see if it is a Capture move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_capture(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::Capture
                | MoveType::EnPassant
                | MoveType::PromoteKnightCapture
                | MoveType::PromoteBishopCapture
                | MoveType::PromoteRookCapture
                | MoveType::PromoteQueenCapture
        )
    }

    /// Tests the given move to see if it is an En Passant move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_en_passant(&self) -> bool {
        self.move_type == MoveType::EnPassant
    }

    /// Tests the given move to see if it is a Castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_castle(&self) -> bool {
        matches!(self.move_type, MoveType::KingCastle | MoveType::QueenCastle)
    }

    /// Tests the given move to see if it is a Promotion move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_promote(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::PromoteKnightQuiet
                | MoveType::PromoteBishopQuiet
                | MoveType::PromoteRookQuiet
                | MoveType::PromoteQueenQuiet
                | MoveType::PromoteKnightCapture
                | MoveType::PromoteBishopCapture
                | MoveType::PromoteRookCapture
                | MoveType::PromoteQueenCapture
        )
    }

    /// Tests the given move to see if it is a quiet move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_quiet(&self) -> bool {
        self.move_type == MoveType::Quiet
    }

    /// Tests the given move to see if it is an Queen-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_queen_castle(&self) -> bool {
        self.move_type == MoveType::QueenCastle
    }

    /// Tests the given move to see if it is an King-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_king_castle(&self) -> bool {
        self.move_type == MoveType::KingCastle
    }

    /// Tests the given move to see if it is a Double pawn first move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_double_pawn(&self) -> bool {
        self.move_type == MoveType::DoublePawn
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn print_move(&self) {
        println!("From {:?}, To {:?}", self.from_sq, self.to_sq);
    }
}

pub fn print_move_list(move_list: &MoveList) {
    for mov in move_list.iter() {
        mov.print_move();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::piece;
    use crate::board::square;
    use crate::board::square::*;
    use crate::moves::mov::Mov;

    #[test]
    pub fn encode_decode_king_white_castle() {
        let mv = Mov::encode_move_castle_kingside_white();

        let decoded_from_sq = mv.from_square();
        let decoded_to_sq = mv.to_square();

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
        let mv = Mov::encode_move_castle_queenside_white();

        let decoded_from_sq = mv.from_square();
        let decoded_to_sq = mv.to_square();

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
        let mv = Mov::encode_move_castle_kingside_black();

        let decoded_from_sq = mv.from_square();
        let decoded_to_sq = mv.to_square();

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
        let mv = Mov::encode_move_castle_queenside_black();

        let decoded_from_sq = mv.from_square();
        let decoded_to_sq = mv.to_square();

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
        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                // encode
                let mv = Mov::encode_move_quiet(*from_sq, *to_sq);

                assert!(mv.is_quiet());
                assert!(!mv.is_capture());
                assert!(!mv.is_castle());
                assert!(!mv.is_double_pawn());
                assert!(!mv.is_promote());

                let decoded_from_sq = mv.from_square();
                let decoded_to_sq = mv.to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_double_pawn_first_move() {
        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                // encode
                let mv = Mov::encode_move_double_pawn_first(*from_sq, *to_sq);
                assert!(mv.is_double_pawn());

                assert!(!mv.is_quiet());
                assert!(!mv.is_capture());
                assert!(!mv.is_castle());
                assert!(!mv.is_promote());

                let decoded_from_sq = mv.from_square();
                let decoded_to_sq = mv.to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_en_passant() {
        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                let mv = Mov::encode_move_en_passant(*from_sq, *to_sq);

                assert!(mv.is_en_passant());
                assert!(mv.is_capture());
                assert!(!mv.is_castle());
                assert!(!mv.is_double_pawn());
                assert!(!mv.is_promote());

                let decoded_from_sq = mv.from_square();
                let decoded_to_sq = mv.to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_promotion_move_non_capture() {
        let target_promotions = [
            &piece::WHITE_BISHOP,
            &piece::WHITE_KNIGHT,
            &piece::WHITE_ROOK,
            &piece::WHITE_QUEEN,
            &piece::BLACK_BISHOP,
            &piece::BLACK_KNIGHT,
            &piece::BLACK_ROOK,
            &piece::BLACK_QUEEN,
        ];

        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                for pce in target_promotions.iter() {
                    let mv = Mov::encode_move_with_promotion(*from_sq, *to_sq, pce);

                    assert!(mv.is_promote());
                    assert!(!mv.is_capture());

                    let col = pce.colour();
                    let decoded_pce = mv.decode_promotion_piece(col);
                    assert_eq!(decoded_pce, *pce);

                    let decoded_from_sq = mv.from_square();
                    let decoded_to_sq = mv.to_square();

                    assert_eq!(decoded_from_sq, *from_sq);
                    assert_eq!(decoded_to_sq, *to_sq);
                }
            }
        }
    }

    #[test]
    pub fn encode_decode_promotion_move_capture() {
        let target_promotions = [
            &piece::WHITE_BISHOP,
            &piece::WHITE_KNIGHT,
            &piece::WHITE_ROOK,
            &piece::WHITE_QUEEN,
            &piece::BLACK_BISHOP,
            &piece::BLACK_KNIGHT,
            &piece::BLACK_ROOK,
            &piece::BLACK_QUEEN,
        ];

        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                for pce in target_promotions.iter() {
                    let mv = Mov::encode_move_with_promotion_capture(*from_sq, *to_sq, pce);

                    assert!(mv.is_promote());
                    assert!(mv.is_capture());

                    let col = pce.colour();
                    let decoded_piece = mv.decode_promotion_piece(col);
                    assert_eq!(decoded_piece, *pce);

                    let decoded_from_sq = mv.from_square();
                    let decoded_to_sq = mv.to_square();

                    assert_eq!(decoded_from_sq, *from_sq);
                    assert_eq!(decoded_to_sq, *to_sq);
                }
            }
        }
    }
}
