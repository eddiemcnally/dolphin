use crate::board::colour::Colour;
use crate::board::piece;
use crate::board::piece::Piece;
use crate::board::square::Square;
use crate::board::square::*;
use crate::board::types::ToInt;
use crate::moves::move_list::MoveList;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::ops::Shr;

// map for u64
//
// XXXX XXXX XXXX XXXX ---- ---- ---- ----  Score (u32)
// ---- ---- ---- ---- ---- ---- XXXX XXXX  Move info (u16)
//
const MASK_MOVE_INFO_MASK: u64 = 0x0000_0000_0000_FFFF;
const MASK_SCORE_MASK: u64 = 0xFFFF_FFFF_0000_0000;
const SHIFT_SCORE: u32 = 32;

// Move info bit map
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
#[rustfmt::skip]
pub enum MoveType {
    Quiet                   = MV_FLG_QUIET,
    DoublePawn              = MV_FLG_DOUBLE_PAWN,
    KingCastle              = MV_FLG_KING_CASTLE,
    QueenCastle             = MV_FLG_QUEEN_CASTLE,
    Capture                 = MV_FLG_CAPTURE,
    EnPassant               = MV_FLG_EN_PASS,
    PromoteKnightQuiet      = MV_FLG_PROMOTE_KNIGHT,
    PromoteBishopQuiet      = MV_FLG_PROMOTE_BISHOP,
    PromoteRookQuiet        = MV_FLG_PROMOTE_ROOK,
    PromoteQueenQuiet       = MV_FLG_PROMOTE_QUEEN,
    PromoteKnightCapture    = MV_FLG_PROMOTE_KNIGHT_CAPTURE,
    PromoteBishopCapture    = MV_FLG_PROMOTE_BISHOP_CAPTURE,
    PromoteRookCapture      = MV_FLG_PROMOTE_ROOK_CAPTURE,
    PromoteQueenCapture     = MV_FLG_PROMOTE_QUEEN_CAPTURE,
}

impl MoveType {
    pub fn from_num(num: u16) -> MoveType {
        let mvt = MoveType::try_from(num);
        match mvt {
            Ok(mvt) => mvt,
            _ => panic!("Invalid piece offset {}.", num),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Mov {
    mv: u64,
}

#[rustfmt::skip]
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
        let mv: u64 = (from_sq.to_u8() as u16 & MASK_FROM_SQ
            | (to_sq.to_u8() as u16) << SHIFT_TO_SQ & MASK_TO_SQ) as u64;
        Mov { mv }
    }

    pub fn encode_move_capture(from_sq: Square, to_sq: Square) -> Mov {
        let mut mov = Mov::encode_move_quiet(from_sq, to_sq);
        mov.mv = set_move_info(MV_FLG_CAPTURE, mov.mv);
        mov
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

        let mut mov = Mov::encode_move_quiet(from_sq, to_sq);

        let mask = match promotion_piece {
            &piece::WHITE_KNIGHT | &piece::BLACK_KNIGHT => MV_FLG_PROMOTE_KNIGHT,
            &piece::WHITE_BISHOP | &piece::BLACK_BISHOP => MV_FLG_PROMOTE_BISHOP,
            &piece::WHITE_ROOK | &piece::BLACK_ROOK => MV_FLG_PROMOTE_ROOK,
            &piece::WHITE_QUEEN | &piece::BLACK_QUEEN => MV_FLG_PROMOTE_QUEEN,
            _ => panic!("Invalid promotion type"),
        };
        mov.mv = set_move_info(mask, mov.mv);
        mov
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

        let mut mov = Mov::encode_move_with_promotion(from_sq, to_sq, promotion_piece);
        mov.mv = set_move_info(MV_FLG_CAPTURE, mov.mv);
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
        mov.mv = set_move_info(MV_FLG_EN_PASS, mov.mv);
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
        mov.mv = set_move_info(MV_FLG_DOUBLE_PAWN, mov.mv);
        mov
    }

    /// Encodes a White King-side castle move
    ///
    pub fn encode_move_castle_kingside_white() -> Mov {
        let mut mov = Mov::encode_move_quiet(SQUARE_E1, SQUARE_G1);
        mov.mv = set_move_info(MV_FLG_KING_CASTLE, mov.mv);
        mov
    }

    /// Encodes a Black King-side castle move
    ///
    pub fn encode_move_castle_kingside_black() -> Mov {
        let mut mov = Mov::encode_move_quiet(SQUARE_E8, SQUARE_G8);
        mov.mv = set_move_info(MV_FLG_KING_CASTLE, mov.mv);
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_white() -> Mov {
        let mut mov = Mov::encode_move_quiet(SQUARE_E1, SQUARE_C1);
        mov.mv = set_move_info(MV_FLG_QUEEN_CASTLE, mov.mv);
        mov
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_black() -> Mov {
        let mut mov = Mov::encode_move_quiet(SQUARE_E8, SQUARE_C8);
        mov.mv = set_move_info(MV_FLG_QUEEN_CASTLE, mov.mv);
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn decode_from_square(self) -> Square {
        let info = extract_move_info(self.mv);
        let sq = (info & MASK_FROM_SQ).shr(SHIFT_FROM_SQ);
        Square::new(sq as u8).unwrap()
    }

    ///
    /// Decodes the "to" square from the Move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn decode_to_square(self) -> Square {
        let info = extract_move_info(self.mv);
        let sq = (info & MASK_TO_SQ).shr(SHIFT_TO_SQ);
        Square::new(sq as u8).unwrap()
    }

    pub fn decode_promotion_piece(self, colour: Colour) -> &'static Piece {
        let info = extract_move_info(self.mv);

        let flags = info & MASK_FLAGS;

        match colour {
            Colour::White => match flags {
                MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => &piece::WHITE_KNIGHT,
                MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => &piece::WHITE_BISHOP,
                MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => &piece::WHITE_QUEEN,
                MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => &piece::WHITE_ROOK,
                _ => panic!("Invalid promotion piece"),
            },
            Colour::Black => match flags {
                MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => &piece::BLACK_KNIGHT,
                MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => &piece::BLACK_BISHOP,
                MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => &piece::BLACK_QUEEN,
                MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => &piece::BLACK_ROOK,
                _ => panic!("Invalid promotion piece"),
            },
        }
    }

    pub fn get_move_type(self) -> MoveType {
        let info = extract_move_info(self.mv);
        let flag = info & MASK_FLAGS;
        MoveType::from_num(flag)
    }

    /// Tests the given move to see if it is a Capture move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_capture(self) -> bool {
        let info = extract_move_info(self.mv);
        (info & MASK_FLAGS & MV_FLG_CAPTURE) != 0
    }

    /// Tests the given move to see if it is an En Passant move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_en_passant(self) -> bool {
        let info = extract_move_info(self.mv);
        info & MASK_FLAGS == MV_FLG_EN_PASS
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
        let info = extract_move_info(self.mv);
        info & MASK_FLAGS & MV_FLG_BIT_PROMOTE > 0
    }

    /// Tests the given move to see if it is a quiet move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_quiet(self) -> bool {
        let info = extract_move_info(self.mv);
        info & MASK_FLAGS == MV_FLG_QUIET
    }

    /// Tests the given move to see if it is an Queen-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_queen_castle(self) -> bool {
        let info = extract_move_info(self.mv);
        info & MASK_FLAGS == MV_FLG_QUEEN_CASTLE
    }

    /// Tests the given move to see if it is an King-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_king_castle(self) -> bool {
        let info = extract_move_info(self.mv);
        info & MASK_FLAGS == MV_FLG_KING_CASTLE
    }

    /// Tests the given move to see if it is a Double pawn first move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub const fn is_double_pawn(self) -> bool {
        let info = extract_move_info(self.mv);
        info & MASK_FLAGS == MV_FLG_DOUBLE_PAWN
    }

    pub fn set_score(&mut self, score: i32) {
        let shifted_score: u64 = (score as u64) << SHIFT_SCORE;
        self.mv |= shifted_score;
    }

    pub fn get_score(&self) -> i32 {
        let score: i32 = (((self.mv & MASK_SCORE_MASK) >> SHIFT_SCORE) as u64)
            .try_into()
            .unwrap();
        score
    }

    pub fn print_move(self) {
        let from_sq = self.decode_from_square();
        let to_sq = self.decode_to_square();
        println!("From {:?}, To {:?}", from_sq, to_sq);
    }
}

#[inline(always)]
const fn set_move_info(info: u16, mv: u64) -> u64 {
    mv | (info as u64 & MASK_MOVE_INFO_MASK)
}
#[inline(always)]
const fn extract_move_info(mv: u64) -> u16 {
    (mv & MASK_MOVE_INFO_MASK) as u16
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
        let mv = Mov::encode_move_castle_queenside_white();

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
        let mv = Mov::encode_move_castle_kingside_black();

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
        let mv = Mov::encode_move_castle_queenside_black();

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

                    let decoded_from_sq = mv.decode_from_square();
                    let decoded_to_sq = mv.decode_to_square();

                    assert_eq!(decoded_from_sq, *from_sq);
                    assert_eq!(decoded_to_sq, *to_sq);
                }
            }
        }
    }
}
