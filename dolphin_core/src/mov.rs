use crate::move_list::MoveList;
use crate::piece::Colour;
use crate::piece::Piece;
use crate::square::Square;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Shr;

// ---- ---- --XX XXXX      From Square
// ---- XXXX XX-- ----      To Square
// XXXX ---- ---- ----      Flags
const MASK_FROM_SQ: u16 = 0x003F;
const MASK_TO_SQ: u16 = 0x0FC0;
const MASK_FLAGS: u16 = 0xF000;

const SHIFT_FROM_SQ: u16 = 0;
const SHIFT_TO_SQ: u16 = 6;

// bitmap for type Move
// See https://www.chessprogramming.org/Encoding_Moves
//
//  0000 ---- Quiet move
//  0001 ---- Double Pawn push
//  0010 ---- King Castle
//  0011 ---- Queen Castle
//  0100 ---- Capture
//  0101 ---- En Passant Capture
//  1000 ---- Promotion Knight
//  1001 ---- Promotion Bishop
//  1010 ---- Promotion Rook
//  1011 ---- Promotion Queen
//  1100 ---- Promotion Knight Capture
//  1101 ---- Promotion Bishop Capture
//  1110 ---- Promotion Rook Capture
//  1111 ---- Promotion Queen Capture

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
const MV_FLG_PROMOTE_KNIGHT_CAPTURE: u16 = MV_FLG_PROMOTE_KNIGHT | MV_FLG_CAPTURE;
const MV_FLG_PROMOTE_BISHOP_CAPTURE: u16 = MV_FLG_PROMOTE_BISHOP | MV_FLG_CAPTURE;
const MV_FLG_PROMOTE_ROOK_CAPTURE: u16 = MV_FLG_PROMOTE_ROOK | MV_FLG_CAPTURE;
const MV_FLG_PROMOTE_QUEEN_CAPTURE: u16 = MV_FLG_PROMOTE_QUEEN | MV_FLG_CAPTURE;

const MV_FLG_BIT_PROMOTE: u16 = 0x8000;

#[repr(u16)]
#[derive(TryFromPrimitive, Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveType {
    Quiet = MV_FLG_QUIET,
    DoublePawn = MV_FLG_DOUBLE_PAWN,
    KingCastle = MV_FLG_KING_CASTLE,
    QueenCastle = MV_FLG_QUEEN_CASTLE,
    Capture = MV_FLG_CAPTURE,
    EnPassant = MV_FLG_EN_PASS,
    PromoteKnightQuiet = MV_FLG_PROMOTE_KNIGHT,
    PromoteBishopQuiet = MV_FLG_PROMOTE_BISHOP,
    PromoteRookQuiet = MV_FLG_PROMOTE_ROOK,
    PromoteQueenQuiet = MV_FLG_PROMOTE_QUEEN,
    PromoteKnightCapture = MV_FLG_PROMOTE_KNIGHT_CAPTURE,
    PromoteBishopCapture = MV_FLG_PROMOTE_BISHOP_CAPTURE,
    PromoteRookCapture = MV_FLG_PROMOTE_ROOK_CAPTURE,
    PromoteQueenCapture = MV_FLG_PROMOTE_QUEEN_CAPTURE,
}

impl MoveType {
    pub fn from_num(num: u16) -> MoveType {
        let mvt = MoveType::try_from(num);
        match mvt {
            Ok(mvt) => return mvt,
            _ => panic!("Invalid piece offset {}.", num),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Mov {
    mv: u16,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct MovPlusScore {
    mov: Mov,
    score: i32,
}

impl MovPlusScore {
    pub fn new(mov: Mov, score: i32) -> MovPlusScore {
        MovPlusScore { mov, score }
    }
}

impl fmt::Debug for Mov {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        let from_sq = self.decode_from_square();
        let to_sq = self.decode_to_square();

        debug_str.push_str(&format!("[{}", from_sq.file()));
        debug_str.push_str(&format!("{}->", from_sq.rank()));
        debug_str.push_str(&format!("{}", to_sq.file()));
        debug_str.push_str(&format!("{} ", to_sq.rank()));

        let mt = match self.get_move_type() {
            MoveType::Quiet => "Quiet",
            MoveType::DoublePawn => "DoubleFirstMove",
            MoveType::KingCastle => "KingCastle",
            MoveType::QueenCastle => "QueenCastle",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "EnPassant",
            MoveType::PromoteKnightQuiet => "PromoteKnightQuiet",
            MoveType::PromoteBishopQuiet => "PromoteBishopQuiet",
            MoveType::PromoteRookQuiet => "PromoteRookQuiet",
            MoveType::PromoteQueenQuiet => "PromoteQueenQuiet",
            MoveType::PromoteKnightCapture => "PromoteKnightCapture",
            MoveType::PromoteBishopCapture => "PromoteBishopCapture",
            MoveType::PromoteRookCapture => "PromoteRookCapture",
            MoveType::PromoteQueenCapture => "PromoteQueenCapture",
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
    pub const fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Mov {
        let mv: u16 = from_sq as u16 & MASK_FROM_SQ | (to_sq as u16) << SHIFT_TO_SQ & MASK_TO_SQ;
        Mov { mv }
    }

    pub const fn encode_move_capture(from_sq: Square, to_sq: Square) -> Mov {
        let mut mov = Mov::encode_move_quiet(from_sq, to_sq);
        mov.mv |= MV_FLG_CAPTURE;
        mov
    }

    pub fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Mov {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        let mut mov = Mov::encode_move_quiet(from_sq, to_sq);

        let mask = match promotion_piece {
            Piece::WhiteKnight | Piece::BlackKnight => MV_FLG_PROMOTE_KNIGHT,
            Piece::WhiteBishop | Piece::BlackBishop => MV_FLG_PROMOTE_BISHOP,
            Piece::WhiteRook | Piece::BlackRook => MV_FLG_PROMOTE_ROOK,
            Piece::WhiteQueen | Piece::BlackQueen => MV_FLG_PROMOTE_QUEEN,
            _ => panic!("Invalid promotion type"),
        };
        mov.mv |= mask;
        mov
    }

    pub fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Mov {
        debug_assert!(
            from_sq != to_sq,
            "from and to square are same : {}",
            from_sq
        );

        let mut mov = Mov::encode_move_with_promotion(from_sq, to_sq, promotion_piece);
        mov.mv |= MV_FLG_CAPTURE;
        mov
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

        let mut mov = Mov::encode_move_quiet(from_sq, to_sq);
        mov.mv |= MV_FLG_EN_PASS;
        mov
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

        let mut mov = Mov::encode_move_quiet(from_sq, to_sq);
        mov.mv |= MV_FLG_DOUBLE_PAWN;
        mov
    }

    /// Encodes a White King-side castle move
    ///
    pub const fn encode_move_castle_kingside_white() -> Mov {
        let mut mov = Mov::encode_move_quiet(Square::e1, Square::g1);
        mov.mv |= MV_FLG_KING_CASTLE;
        mov
    }

    /// Encodes a Black King-side castle move
    ///
    pub const fn encode_move_castle_kingside_black() -> Mov {
        let mut mov = Mov::encode_move_quiet(Square::e8, Square::g8);
        mov.mv |= MV_FLG_KING_CASTLE;
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_white() -> Mov {
        let mut mov = Mov::encode_move_quiet(Square::e1, Square::c1);
        mov.mv |= MV_FLG_QUEEN_CASTLE;
        mov
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_black() -> Mov {
        let mut mov = Mov::encode_move_quiet(Square::e8, Square::c8);
        mov.mv |= MV_FLG_QUEEN_CASTLE;
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn decode_from_square(self) -> Square {
        let sq = (self.mv & MASK_FROM_SQ).shr(SHIFT_FROM_SQ);
        Square::from_num(sq as u8).unwrap()
    }

    ///
    /// Decodes the "to" square from the Move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn decode_to_square(self) -> Square {
        let sq = (self.mv & MASK_TO_SQ).shr(SHIFT_TO_SQ);
        Square::from_num(sq as u8).unwrap()
    }

    pub fn decode_promotion_piece(self, colour: Colour) -> Piece {
        let flags = self.mv & MASK_FLAGS;

        match colour {
            Colour::White => match flags {
                MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => Piece::WhiteKnight,
                MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => Piece::WhiteBishop,
                MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => Piece::WhiteQueen,
                MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => Piece::WhiteRook,
                _ => panic!("Invalid promotion piece"),
            },
            Colour::Black => match flags {
                MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => Piece::BlackKnight,
                MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => Piece::BlackBishop,
                MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => Piece::BlackQueen,
                MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => Piece::BlackRook,
                _ => panic!("Invalid promotion piece"),
            },
        }
    }

    pub fn get_move_type(self) -> MoveType {
        let flag = self.mv & MASK_FLAGS;
        MoveType::from_num(flag)
    }

    /// Tests the given move to see if it is a Capture move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_capture(self) -> bool {
        (self.mv & MASK_FLAGS & MV_FLG_CAPTURE) != 0
    }

    /// Tests the given move to see if it is an En Passant move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_en_passant(self) -> bool {
        self.mv & MASK_FLAGS == MV_FLG_EN_PASS
    }

    /// Tests the given move to see if it is a Castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_castle(self) -> bool {
        self.is_king_castle() || self.is_queen_castle()
    }

    /// Tests the given move to see if it is a Promotion move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_promote(self) -> bool {
        self.mv & MASK_FLAGS & MV_FLG_BIT_PROMOTE > 0
    }

    /// Tests the given move to see if it is a quiet move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_quiet(self) -> bool {
        self.mv & MASK_FLAGS == MV_FLG_QUIET
    }

    /// Tests the given move to see if it is an Queen-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_queen_castle(self) -> bool {
        self.mv & MASK_FLAGS == MV_FLG_QUEEN_CASTLE
    }

    /// Tests the given move to see if it is an King-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_king_castle(self) -> bool {
        self.mv & MASK_FLAGS == MV_FLG_KING_CASTLE
    }

    /// Tests the given move to see if it is a Double pawn first move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_double_pawn(self) -> bool {
        self.mv & MASK_FLAGS == MV_FLG_DOUBLE_PAWN
    }

    pub fn print_move(self) {
        let from_sq = self.decode_from_square();
        let to_sq = self.decode_to_square();
        println!("From {:?}, To {:?}", from_sq, to_sq);
    }
}

pub fn print_move_list(move_list: &MoveList) {
    for mov in move_list.iter() {
        mov.print_move();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::mov::Mov;
    use crate::piece::Piece;
    use crate::square;
    use crate::square::Square;

    #[test]
    pub fn encode_decode_king_white_castle() {
        let mv = Mov::encode_move_castle_kingside_white();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, Square::e1);
        assert_eq!(decoded_to_sq, Square::g1);

        assert_eq!(mv.is_king_castle(), true);
        assert_eq!(mv.is_castle(), true);
        assert_eq!(mv.is_queen_castle(), false);

        assert_eq!(mv.is_quiet(), false);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_double_pawn(), false);
        assert_eq!(mv.is_promote(), false);
    }

    #[test]
    pub fn encode_decode_queen_white_castle() {
        let mv = Mov::encode_move_castle_queenside_white();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, Square::e1);
        assert_eq!(decoded_to_sq, Square::c1);

        assert_eq!(mv.is_king_castle(), false);
        assert_eq!(mv.is_castle(), true);
        assert_eq!(mv.is_queen_castle(), true);

        assert_eq!(mv.is_quiet(), false);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_double_pawn(), false);
        assert_eq!(mv.is_promote(), false);
    }

    #[test]
    pub fn encode_decode_king_black_castle() {
        let mv = Mov::encode_move_castle_kingside_black();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, Square::e8);
        assert_eq!(decoded_to_sq, Square::g8);

        assert_eq!(mv.is_king_castle(), true);
        assert_eq!(mv.is_castle(), true);
        assert_eq!(mv.is_queen_castle(), false);

        assert_eq!(mv.is_quiet(), false);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_double_pawn(), false);
        assert_eq!(mv.is_promote(), false);
    }

    #[test]
    pub fn encode_decode_queen_black_castle() {
        let mv = Mov::encode_move_castle_queenside_black();

        let decoded_from_sq = mv.decode_from_square();
        let decoded_to_sq = mv.decode_to_square();

        assert_eq!(decoded_from_sq, Square::e8);
        assert_eq!(decoded_to_sq, Square::c8);

        assert_eq!(mv.is_king_castle(), false);
        assert_eq!(mv.is_castle(), true);
        assert_eq!(mv.is_queen_castle(), true);

        assert_eq!(mv.is_quiet(), false);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_double_pawn(), false);
        assert_eq!(mv.is_promote(), false);
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

                assert_eq!(mv.is_quiet(), true);
                assert_eq!(mv.is_capture(), false);
                assert_eq!(mv.is_castle(), false);
                assert_eq!(mv.is_double_pawn(), false);
                assert_eq!(mv.is_promote(), false);

                let decoded_from_sq = mv.decode_from_square();
                let decoded_to_sq = mv.decode_to_square();

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
                assert_eq!(mv.is_double_pawn(), true);

                assert_eq!(mv.is_quiet(), false);
                assert_eq!(mv.is_capture(), false);
                assert_eq!(mv.is_castle(), false);
                assert_eq!(mv.is_promote(), false);

                let decoded_from_sq = mv.decode_from_square();
                let decoded_to_sq = mv.decode_to_square();

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

                assert_eq!(mv.is_en_passant(), true);
                assert_eq!(mv.is_capture(), true);
                assert_eq!(mv.is_castle(), false);
                assert_eq!(mv.is_double_pawn(), false);
                assert_eq!(mv.is_promote(), false);

                let decoded_from_sq = mv.decode_from_square();
                let decoded_to_sq = mv.decode_to_square();

                assert_eq!(decoded_from_sq, *from_sq);
                assert_eq!(decoded_to_sq, *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_promotion_move_non_capture() {
        let target_promotions = [
            Piece::WhiteBishop,
            Piece::WhiteKnight,
            Piece::WhiteRook,
            Piece::WhiteQueen,
            Piece::BlackBishop,
            Piece::BlackKnight,
            Piece::BlackRook,
            Piece::BlackQueen,
        ];

        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                for pce in target_promotions.iter() {
                    let mv = Mov::encode_move_with_promotion(*from_sq, *to_sq, *pce);

                    assert_eq!(mv.is_promote(), true);
                    assert_eq!(mv.is_capture(), false);

                    let col = pce.colour();
                    let decoded_pce = mv.decode_promotion_piece(col);
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
        let target_promotions = [
            Piece::WhiteBishop,
            Piece::WhiteKnight,
            Piece::WhiteRook,
            Piece::WhiteQueen,
            Piece::BlackBishop,
            Piece::BlackKnight,
            Piece::BlackRook,
            Piece::BlackQueen,
        ];

        for from_sq in square::SQUARES {
            for to_sq in square::SQUARES {
                if *from_sq == *to_sq {
                    continue;
                }

                for pce in target_promotions.iter() {
                    let mv = Mov::encode_move_with_promotion_capture(*from_sq, *to_sq, *pce);

                    assert_eq!(mv.is_promote(), true);
                    assert_eq!(mv.is_capture(), true);

                    let col = pce.colour();
                    let decoded_piece = mv.decode_promotion_piece(col);
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
