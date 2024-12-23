use crate::board::piece::Piece;
use crate::board::square::Square;
use enumn::N;
use std::fmt;
use std::process;

#[rustfmt::skip]
#[derive(Eq, PartialEq, Copy, Clone, Hash, N)]
pub enum MoveType {
    Normal      = 0b0000_0000_0000_0000,
    Promotion   = 0b0001_0000_0000_0000,
    EnPassant   = 0b0010_0000_0000_0000,
    Castle      = 0b0011_0000_0000_0000,
}

#[rustfmt::skip]
#[derive(Eq, PartialEq, Copy, Clone, Hash, N)]
enum PromotionTypes {
    Bishop  = 0b0000_0000_0000_0000,
    Knight  = 0b0100_0000_0000_0000,
    Rook    = 0b1000_0000_0000_0000,
    Queen   = 0b1100_0000_0000_0000,
}

enum BitShift {
    FromSq = 0,
    ToSq = 6,
}

#[rustfmt::skip]
enum BitMask{
    FromSq      = 0b0000_0000_0011_1111,
    ToSq        = 0b0000_1111_1100_0000,
    MoveType    = 0b0011_0000_0000_0000,
    PromoTarget = 0b1100_0000_0000_0000,
}

// Move bits (copied from StockFish)
// xxxx xxxx xxxx xxxx
// ---- ---- --xx xxxx  source (from) square
// ---- xxxx xx-- ----  target (to) square
// --XX ---- ---- ----  Promotion target (00 bishop, 01 knight, 10 rook, 11 Queen)
// xx-- ---- ---- ----  Flags (01 promotion, 10 en passant, 11 castling)
#[derive(Eq, PartialEq, Copy, Clone, Hash, Default)]
pub struct Move {
    bits: u16,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct ScoredMove {
    mv: Move,
    score: Score,
}

impl ScoredMove {
    pub const fn new(mv: Move, score: Score) -> ScoredMove {
        ScoredMove { mv, score }
    }

    pub const fn get_move(&self) -> Move {
        self.mv
    }

    pub const fn get_score(&self) -> Score {
        self.score
    }
}

pub type Score = i16;

impl Move {
    pub const fn from_sq(&self) -> Square {
        let bits = (self.bits & BitMask::FromSq as u16) >> BitShift::FromSq as u16;
        Square::new(bits as u8).unwrap()
    }

    pub const fn to_sq(&self) -> Square {
        let bits = (self.bits & BitMask::ToSq as u16) >> BitShift::ToSq as u16;
        Square::new(bits as u8).unwrap()
    }

    pub fn move_type(&self) -> MoveType {
        let bits = self.bits & BitMask::MoveType as u16;

        const NORMAL: u16 = MoveType::Normal as u16;
        const PROMOTE: u16 = MoveType::Promotion as u16;
        const EN_PASSANT: u16 = MoveType::EnPassant as u16;
        const CASTLE: u16 = MoveType::Castle as u16;

        match bits {
            NORMAL => MoveType::Normal,
            PROMOTE => MoveType::Promotion,
            EN_PASSANT => MoveType::EnPassant,
            CASTLE => MoveType::Castle,
            _ => panic!("Invalid move type"),
        }
    }

    pub const fn encode_move(from_sq: Square, to_sq: Square) -> Move {
        Move {
            bits: Self::encode_from_to_sq(from_sq, to_sq),
        }
    }

    pub fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_role: Piece,
    ) -> Move {
        let mt = match promotion_role {
            Piece::Knight => PromotionTypes::Knight,
            Piece::Bishop => PromotionTypes::Bishop,
            Piece::Rook => PromotionTypes::Rook,
            Piece::Queen => PromotionTypes::Queen,
            _ => {
                eprintln!("Invalid promotion piece");
                process::exit(1);
            }
        };

        let mut bits = Self::encode_from_to_sq(from_sq, to_sq);
        bits |= mt as u16;
        bits |= MoveType::Promotion as u16;

        Move { bits }
    }

    /// Encodes an En Passant move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub const fn encode_move_en_passant(from_sq: Square, to_sq: Square) -> Move {
        let mut bits = Self::encode_from_to_sq(from_sq, to_sq);
        bits |= MoveType::EnPassant as u16;

        Move { bits }
    }

    /// Encodes a White King-side castle move
    ///
    pub const fn encode_move_castle_kingside_white() -> Move {
        let mut bits = Self::encode_from_to_sq(Square::E1, Square::G1);
        bits |= MoveType::Castle as u16;

        Move { bits }
    }

    /// Encodes a Black King-side castle move
    ///
    pub const fn encode_move_castle_kingside_black() -> Move {
        let mut bits = Self::encode_from_to_sq(Square::E8, Square::G8);
        bits |= MoveType::Castle as u16;

        Move { bits }
    }

    /// Encodes a White Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_white() -> Move {
        let mut bits = Self::encode_from_to_sq(Square::E1, Square::C1);
        bits |= MoveType::Castle as u16;

        Move { bits }
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub const fn encode_move_castle_queenside_black() -> Move {
        let mut bits = Self::encode_from_to_sq(Square::E8, Square::C8);
        bits |= MoveType::Castle as u16;

        Move { bits }
    }

    pub fn print_move(&self) {
        let (from_sq, to_sq) = self.decode_from_to_sq();
        println!("From {:?}, To {:?}", from_sq, to_sq);
    }

    const fn encode_from_to_sq(from_sq: Square, to_sq: Square) -> u16 {
        let mut bits = (from_sq.as_index() as u16) << BitShift::FromSq as usize;
        bits = bits | ((to_sq.as_index() as u16) << BitShift::ToSq as usize);
        bits
    }

    pub fn decode_from_to_sq(&self) -> (Square, Square) {
        let from_sq = (self.bits & BitMask::FromSq as u16) >> BitShift::FromSq as usize;
        let to_sq = (self.bits & BitMask::ToSq as u16) >> BitShift::ToSq as usize;
        (
            Square::new(from_sq as u8).expect("Bad from_sq"),
            Square::new(to_sq as u8).expect("bad to_sq"),
        )
    }

    pub fn decode_promotion_piece(&self) -> Piece {
        let pp = self.bits & BitMask::PromoTarget as u16;
        let promo_type = PromotionTypes::n(pp).expect("Invalid promotion type");
        match promo_type {
            PromotionTypes::Bishop => return Piece::Bishop,
            PromotionTypes::Knight => return Piece::Knight,
            PromotionTypes::Rook => return Piece::Rook,
            PromotionTypes::Queen => return Piece::Queen,
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        let from_sq = self.from_sq();
        let to_sq = self.to_sq();

        debug_str.push_str(&format!("[{}", from_sq.file()));
        debug_str.push_str(&format!("{}->", from_sq.rank()));
        debug_str.push_str(&format!("{}", to_sq.file()));
        debug_str.push_str(&format!("{} ", to_sq.rank()));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
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

        assert_eq!(mv.from_sq(), Square::E1);
        assert_eq!(mv.to_sq(), Square::G1);
    }

    #[test]
    pub fn encode_decode_queen_white_castle() {
        let mv = Move::encode_move_castle_queenside_white();

        assert_eq!(mv.from_sq(), Square::E1);
        assert_eq!(mv.to_sq(), Square::C1);
    }

    #[test]
    pub fn encode_decode_king_black_castle() {
        let mv = Move::encode_move_castle_kingside_black();

        assert_eq!(mv.from_sq(), Square::E8);
        assert_eq!(mv.to_sq(), Square::G8);
    }

    #[test]
    pub fn encode_decode_queen_black_castle() {
        let mv = Move::encode_move_castle_queenside_black();

        assert_eq!(mv.from_sq(), Square::E8);
        assert_eq!(mv.to_sq(), Square::C8);
    }

    #[test]
    pub fn encode_decode_quiet_move() {
        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                // encode
                let mv = Move::encode_move(*from_sq, *to_sq);

                assert_eq!(mv.from_sq(), *from_sq);
                assert_eq!(mv.to_sq(), *to_sq);
            }
        }
    }

    #[test]
    pub fn encode_decode_promotion_piece() {
        let from_sq = Square::D2;
        let to_sq = Square::D1;

        let mut mv = Move::encode_move_with_promotion(from_sq, to_sq, Piece::Bishop);
        assert_eq!(mv.decode_promotion_piece(), Piece::Bishop);

        mv = Move::encode_move_with_promotion(from_sq, to_sq, Piece::Knight);
        assert_eq!(mv.decode_promotion_piece(), Piece::Knight);

        mv = Move::encode_move_with_promotion(from_sq, to_sq, Piece::Rook);
        assert_eq!(mv.decode_promotion_piece(), Piece::Rook);

        mv = Move::encode_move_with_promotion(from_sq, to_sq, Piece::Queen);
        assert_eq!(mv.decode_promotion_piece(), Piece::Queen);
    }

    #[test]
    pub fn encode_decode_en_passant() {
        for from_sq in Square::iterator() {
            for to_sq in Square::iterator() {
                if *from_sq == *to_sq {
                    continue;
                }

                let mv = Move::encode_move_en_passant(*from_sq, *to_sq);

                assert_eq!(mv.from_sq(), *from_sq);
                assert_eq!(mv.to_sq(), *to_sq);
            }
        }
    }
}
