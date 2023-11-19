use crate::board::piece::Piece;
use crate::board::square::Square;
use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum MoveType {
    Quiet,
    DoublePawn,
    KingCastle,
    QueenCastle,
    Capture,
    EnPassant,
    PromoteBishop,
    PromoteKnight,
    PromoteRook,
    PromoteQueen,
    PromoteBishopCapture,
    PromoteKnightCapture,
    PromoteRookCapture,
    PromoteQueenCapture,
}

impl MoveType {
    pub const fn decode_promotion_role(&self) -> Option<Piece> {
        match self {
            MoveType::PromoteBishop | MoveType::PromoteBishopCapture => Some(Piece::Bishop),
            MoveType::PromoteKnight | MoveType::PromoteKnightCapture => Some(Piece::Knight),
            MoveType::PromoteRook | MoveType::PromoteRookCapture => Some(Piece::Rook),
            MoveType::PromoteQueen | MoveType::PromoteQueenCapture => Some(Piece::Queen),
            _ => None,
        }
    }

    pub const fn is_capture(&self) -> bool {
        matches!(
            self,
            MoveType::Capture
                | MoveType::EnPassant
                | MoveType::PromoteBishopCapture
                | MoveType::PromoteKnightCapture
                | MoveType::PromoteQueenCapture
                | MoveType::PromoteRookCapture
        )
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Move {
    from_sq: Square,
    to_sq: Square,
    move_type: MoveType,
    pce_to_move: Piece,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct ScoredMove {
    mv: Move,
    score: Score,
}

impl ScoredMove {
    pub fn new(mv: Move, score: Score) -> ScoredMove {
        ScoredMove { mv, score }
    }

    pub fn get_move(&self) -> Move {
        self.mv
    }

    pub fn get_score(&self) -> Score {
        self.score
    }
}

pub type Score = i16;

impl Move {
    pub fn from_sq(&self) -> Square {
        self.from_sq
    }

    pub fn to_sq(&self) -> Square {
        self.to_sq
    }

    pub fn move_type(&self) -> MoveType {
        self.move_type
    }

    pub fn piece_to_move(&self) -> Piece {
        self.pce_to_move
    }

    pub const fn encode_move_quiet(from_sq: Square, to_sq: Square, pce_to_move: Piece) -> Move {
        Move {
            from_sq,
            to_sq,
            move_type: MoveType::Quiet,
            pce_to_move,
        }
    }

    pub const fn encode_move_capture(from_sq: Square, to_sq: Square, pce_to_move: Piece) -> Move {
        Move {
            from_sq,
            to_sq,
            move_type: MoveType::Capture,
            pce_to_move,
        }
    }

    pub const fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_role: Piece,
    ) -> Move {
        let mt = match promotion_role {
            Piece::Knight => MoveType::PromoteKnight,
            Piece::Bishop => MoveType::PromoteBishop,
            Piece::Rook => MoveType::PromoteRook,
            Piece::Queen => MoveType::PromoteQueen,
            _ => panic!("Invalid promotion piece"),
        };
        Move {
            from_sq,
            to_sq,
            move_type: mt,
            pce_to_move: Piece::Pawn,
        }
    }

    pub const fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_role: Piece,
    ) -> Move {
        let mt = match promotion_role {
            Piece::Knight => MoveType::PromoteKnightCapture,
            Piece::Bishop => MoveType::PromoteBishopCapture,
            Piece::Rook => MoveType::PromoteRookCapture,
            Piece::Queen => MoveType::PromoteQueenCapture,
            _ => panic!("Invalid promotion piece"),
        };
        Move {
            from_sq,
            to_sq,
            move_type: mt,
            pce_to_move: Piece::Pawn,
        }
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub const fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move {
        Move {
            from_sq,
            to_sq,
            move_type: MoveType::EnPassant,
            pce_to_move: Piece::Pawn,
        }
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from squareClone,
    /// * `to_sq`           - the to square
    ///
    pub const fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Move {
        Move {
            from_sq,
            to_sq,
            move_type: MoveType::DoublePawn,
            pce_to_move: Piece::Pawn,
        }
    }

    /// Encodes a White King-side castle move
    ///
    pub const fn encode_move_castle_kingside_white() -> Move {
        Move {
            from_sq: Square::E1,
            to_sq: Square::G1,
            move_type: MoveType::KingCastle,
            pce_to_move: Piece::King,
        }
    }

    /// Encodes a Black King-side castle move
    ///
    pub const fn encode_move_castle_kingside_black() -> Move {
        Move {
            from_sq: Square::E8,
            to_sq: Square::G8,
            move_type: MoveType::KingCastle,
            pce_to_move: Piece::King,
        }
    }

    /// Encodes a White Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_white() -> Move {
        Move {
            from_sq: Square::E1,
            to_sq: Square::C1,
            move_type: MoveType::QueenCastle,
            pce_to_move: Piece::King,
        }
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_black() -> Move {
        Move {
            from_sq: Square::E8,
            to_sq: Square::C8,
            move_type: MoveType::QueenCastle,
            pce_to_move: Piece::King,
        }
    }

    pub fn print_move(&self) {
        println!("From {:?}, To {:?}", self.from_sq, self.to_sq);
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("[{}", self.from_sq.file()));
        debug_str.push_str(&format!("{}->", self.from_sq.rank()));
        debug_str.push_str(&format!("{}", self.to_sq.file()));
        debug_str.push_str(&format!("{} ", self.to_sq.rank()));

        let mt = match self.move_type {
            MoveType::Quiet => "Quiet",
            MoveType::DoublePawn => "DoublePawn",
            MoveType::KingCastle => "KingCastle",
            MoveType::QueenCastle => "QueenCastle",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "EnPassant",
            MoveType::PromoteBishop => "PromoteBishop",
            MoveType::PromoteKnight => "PromoteKnight",
            MoveType::PromoteRook => "PromoteRook",
            MoveType::PromoteQueen => "PromoteQueen",
            MoveType::PromoteBishopCapture => "PromoteBishopCapture",
            MoveType::PromoteKnightCapture => "PromoteKnightCapture",
            MoveType::PromoteRookCapture => "PromoteRookCapture",
            MoveType::PromoteQueenCapture => "PromoteQueenCapture",
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
        Move {
            from_sq: Square::A1,
            to_sq: Square::B1,
            move_type: MoveType::Quiet,
            pce_to_move: Piece::Pawn,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::piece::Piece;
    use crate::board::square::Square;
    use crate::moves::mov::Move;

    #[test]
    pub fn encode_decode_king_white_castle() {
        let mv = Move::encode_move_castle_kingside_white();

        assert_eq!(mv.from_sq, Square::E1);
        assert_eq!(mv.to_sq, Square::G1);
    }

    #[test]
    pub fn encode_decode_queen_white_castle() {
        let mv = Move::encode_move_castle_queenside_white();

        assert_eq!(mv.from_sq, Square::E1);
        assert_eq!(mv.to_sq, Square::C1);
    }

    #[test]
    pub fn encode_decode_king_black_castle() {
        let mv = Move::encode_move_castle_kingside_black();

        assert_eq!(mv.from_sq, Square::E8);
        assert_eq!(mv.to_sq, Square::G8);
    }

    #[test]
    pub fn encode_decode_queen_black_castle() {
        let mv = Move::encode_move_castle_queenside_black();

        assert_eq!(mv.from_sq, Square::E8);
        assert_eq!(mv.to_sq, Square::C8);
    }

    #[test]
    pub fn encode_decode_quiet_move() {
        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                // encode
                let mv = Move::encode_move_quiet(*from_sq, *to_sq, Piece::Bishop);

                assert_eq!(mv.from_sq, *from_sq);
                assert_eq!(mv.to_sq, *to_sq);
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

                let mv = Move::encode_move_double_pawn_first(*from_sq, *to_sq);

                assert_eq!(mv.from_sq, *from_sq);
                assert_eq!(mv.to_sq, *to_sq);
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

                assert_eq!(mv.from_sq, *from_sq);
                assert_eq!(mv.to_sq, *to_sq);
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

                for role in target_promotions.iter() {
                    let mv = Move::encode_move_with_promotion(*from_sq, *to_sq, *role);

                    let decoded_role = mv.move_type.decode_promotion_role();
                    assert_eq!(decoded_role.unwrap(), *role);

                    assert_eq!(mv.from_sq, *from_sq);
                    assert_eq!(mv.to_sq, *to_sq);
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

                for role in target_promotions.iter() {
                    let mv = Move::encode_move_with_promotion_capture(*from_sq, *to_sq, *role);

                    let decoded_role = mv.move_type.decode_promotion_role();
                    assert_eq!(decoded_role.unwrap(), *role);
                    assert_eq!(mv.from_sq, *from_sq);
                    assert_eq!(mv.to_sq, *to_sq);
                }
            }
        }
    }
}
