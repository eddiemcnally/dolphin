use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square::Square;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Mov {
    from_sq: Square,
    to_sq: Square,
    flags: u8,
}

impl Mov {
    /// Encodes a Quiet move given the "from" and "to" squares
    ///
    /// # Arguments
    ///
    /// * `from_sq` - the from square
    /// * `to_sq`   - the to square
    ///
    pub fn encode_quiet(from_sq: Square, to_sq: Square) -> Mov {
        Mov::new(from_sq, to_sq)
    }

    fn new(from_sq: Square, to_sq: Square) -> Mov {
        Mov {
            from_sq: from_sq,
            to_sq: to_sq,
            flags: 0,
        }
    }

    /// Encodes a Promotion move that doesn't involve a captured piece
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    /// * 'promotion_piece' - the target promotion piece
    ///
    pub fn encode_move_with_promotion(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);

        let mask: u8;
        match promotion_piece.role() {
            PieceRole::Knight => mask = MV_FLG_PROMOTE_KNIGHT,
            PieceRole::Bishop => mask = MV_FLG_PROMOTE_BISHOP,
            PieceRole::Rook => mask = MV_FLG_PROMOTE_ROOK,
            PieceRole::Queen => mask = MV_FLG_PROMOTE_QUEEN,
            _ => panic!("Invalid promotion type"),
        }
        mov.flags = mov.flags | mask;
        mov
    }

    /// Encodes a Promotion move that involves a captured piece
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    /// * 'promotion_piece' - the target promotion piece
    ///
    pub fn encode_move_with_promotion_capture(
        from_sq: Square,
        to_sq: Square,
        promotion_piece: Piece,
    ) -> Mov {
        let mut mov = Mov::encode_move_with_promotion(from_sq, to_sq, promotion_piece);
        mov.flags = mov.flags | MV_FLG_BIT_CAPTURE;
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
        let mut mov = Mov::encode_quiet(from_sq, to_sq);
        mov.flags = mov.flags | MV_FLG_EN_PASS;
        mov
    }

    /// Encodes a Double Pawn first move
    ///
    /// # Arguments
    ///
    /// * `from_sq`         - the from square
    /// * `to_sq`           - the to square
    ///
    pub fn encode_move_double_pawn_first(from_sq: Square, to_sq: Square) -> Mov {
        let mut mov = Mov::encode_quiet(from_sq, to_sq);
        mov.flags = mov.flags | MV_FLG_DOUBLE_PAWN;
        mov
    }

    /// Encodes a White King-side castle move
    ///
    pub fn encode_move_castle_kingside_white() -> Mov {
        // todo: this can be determined at compile time, so fix this
        let mut mov = Mov::encode_quiet(Square::e1, Square::g1);
        mov.flags = mov.flags | MV_FLG_KING_CASTLE;
        mov
    }

    /// Encodes a Black King-side castle move
    ///
    pub fn encode_move_castle_kingside_black() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e8, Square::g8);
        mov.flags = mov.flags | MV_FLG_KING_CASTLE;
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_white() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e1, Square::c1);
        mov.flags = mov.flags | MV_FLG_QUEEN_CASTLE;
        mov
    }

    /// Encodes a Black Queen-side castle move
    ///
    pub fn encode_move_castle_queenside_black() -> Mov {
        let mut mov = Mov::encode_quiet(Square::e8, Square::c8);
        mov.flags = mov.flags | MV_FLG_QUEEN_CASTLE;
        mov
    }

    /// Encodes a White Queen-side castle move
    ///
    pub fn decode_from_square(mv: &Mov) -> Square {
        mv.from_sq
    }

    ///
    /// Decodes the "to" square from the Move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn decode_to_square(mv: &Mov) -> Square {
        mv.to_sq
    }

    ///
    /// Decodes the promotion piece from the move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    /// * `side`       - the side/colour
    ///
    pub fn decode_promotion_piece(mv: &Mov, side: Colour) -> Piece {
        let role = match mv.flags {
            MV_FLG_PROMOTE_KNIGHT_CAPTURE | MV_FLG_PROMOTE_KNIGHT => PieceRole::Knight,
            MV_FLG_PROMOTE_BISHOP_CAPTURE | MV_FLG_PROMOTE_BISHOP => PieceRole::Bishop,
            MV_FLG_PROMOTE_QUEEN_CAPTURE | MV_FLG_PROMOTE_QUEEN => PieceRole::Queen,
            MV_FLG_PROMOTE_ROOK_CAPTURE | MV_FLG_PROMOTE_ROOK => PieceRole::Rook,
            _ => panic!("Invalid promotion piece"),
        };

        Piece::new(role, side)
    }

    /// Tests the given move to see if it is a Quiet move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_quiet(mv: &Mov) -> bool {
        mv.flags == MV_FLG_QUIET
    }

    /// Tests the given move to see if it is a Capture move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_capture(mv: &Mov) -> bool {
        (mv.flags & MV_FLG_BIT_CAPTURE) != 0
    }

    /// Tests the given move to see if it is a Promotion move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_promote(mv: &Mov) -> bool {
        (mv.flags & MV_FLG_BIT_PROMOTE) != 0
    }

    /// Tests the given move to see if it is an En Passant move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_en_passant(mv: &Mov) -> bool {
        (mv.flags & MV_FLG_EN_PASS) != 0
    }

    /// Tests the given move to see if it is a Castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_castle(mv: &Mov) -> bool {
        Mov::is_king_castle(mv) || Mov::is_queen_castle(mv)
    }

    /// Tests the given move to see if it is an Queen-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_queen_castle(mv: &Mov) -> bool {
        (mv.flags & MV_FLG_QUEEN_CASTLE) != 0
    }

    /// Tests the given move to see if it is an King-side castle move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_king_castle(mv: &Mov) -> bool {
        (mv.flags & MV_FLG_KING_CASTLE) != 0
    }

    /// Tests the given move to see if it is a Double pawn first move
    ///
    /// # Arguments
    ///
    /// * `mv`         - the move to decode
    ///
    pub fn is_double_pawn(mv: &Mov) -> bool {
        (mv.flags & MV_FLG_DOUBLE_PAWN) != 0
    }
}

// bitmap for type Move
// See http://chessprogramming.wikispaces.com/Encoding+Moves
//
//  ---- 0000 Quiet move
//  ---- 0001 Double Pawn push
//  ---- 0010 King Castle
//  ---- 0011 Queen Castle
//  ---- 0100 Capture
//  ---- 0101 En Passant Capture
//  ---- 1000 Promotion Knight
//  ---- 1001 Promotion Bishop
//  ---- 1010 Promotion Rook
//  ---- 1011 Promotion Queen
//  ---- 1100 Promotion Knight Capture
//  ---- 1101 Promotion Bishop Capture
//  ---- 1110 Promotion Rook Capture
//  ---- 1111 Promotion Queen Capture

const MV_FLG_QUIET: u8 = 0x00;
const MV_FLG_DOUBLE_PAWN: u8 = 0x01;
const MV_FLG_KING_CASTLE: u8 = 0x02;
const MV_FLG_QUEEN_CASTLE: u8 = 0x03;
const MV_FLG_CAPTURE: u8 = 0x04;
const MV_FLG_EN_PASS: u8 = 0x05;
const MV_FLG_PROMOTE_KNIGHT: u8 = 0x08;
const MV_FLG_PROMOTE_BISHOP: u8 = 0x09;
const MV_FLG_PROMOTE_ROOK: u8 = 0x0A;
const MV_FLG_PROMOTE_QUEEN: u8 = 0x0B;
const MV_FLG_PROMOTE_KNIGHT_CAPTURE: u8 = 0x0C;
const MV_FLG_PROMOTE_BISHOP_CAPTURE: u8 = 0x0D;
const MV_FLG_PROMOTE_ROOK_CAPTURE: u8 = 0x0E;
const MV_FLG_PROMOTE_QUEEN_CAPTURE: u8 = 0x0F;

const MV_FLG_BIT_PROMOTE: u8 = 0x08;
const MV_FLG_BIT_CAPTURE: u8 = 0x04;

#[cfg(test)]
pub mod tests {}
