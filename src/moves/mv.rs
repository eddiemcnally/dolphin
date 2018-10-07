#[allow(dead_code)]
use board::square::Square;
use std::ops::Shl;
use std::ops::Shr;
use board::piece::Piece;
use board::piece::Colour;


// bitmap for type Move
// See http://chessprogramming.wikispaces.com/Encoding+Moves
//
//  ---- ---- --11 1111      To Square
//  ---- 1111 11-- ----      From Square
//  0000 ---- ---- ----      Quiet move
//  0001 ---- ---- ----      Double Pawn push
//  0010 ---- ---- ----      King Castle
//  0011 ---- ---- ----      Queen Castle
//  0100 ---- ---- ----      Capture
//  0101 ---- ---- ----      En Passant Capture
//  1000 ---- ---- ----      Promotion Knight
//  1001 ---- ---- ----      Promotion Bishop
//  1010 ---- ---- ----      Promotion Rook
//  1011 ---- ---- ----      Promotion Queen
//  1100 ---- ---- ----      Promotion Knight Capture
//  1101 ---- ---- ----      Promotion Bishop Capture
//  1110 ---- ---- ----      Promotion Rook Capture
//  1111 ---- ---- ----      Promotion Queen Capture

//#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub type Move = u16;

enum move_flags{
    MV_FLG_QUIET = 0x0000,
    MV_FLG_DOUBLE_PAWN = 0x1000,
    MV_FLG_KING_CASTLE = 0x2000,
    MV_FLG_QUEEN_CASTLE = 0x3000,
    MV_FLG_CAPTURE = 0x4000,
    MV_FLG_EN_PASS = 0x5000,
    MV_FLG_PROMOTE_KNIGHT = 0x8000,
    MV_FLG_PROMOTE_BISHOP = 0x9000,
    MV_FLG_PROMOTE_ROOK = 0xA000,
    MV_FLG_PROMOTE_QUEEN = 0xB000,
    MV_FLG_PROMOTE_KNIGHT_CAPTURE = 0xC000,
    MV_FLG_PROMOTE_BISHOP_CAPTURE = 0xD000,
    MV_FLG_PROMOTE_ROOK_CAPTURE = 0xE000,
    MV_FLG_PROMOTE_QUEEN_CAPTURE = 0xF000,
}

enum move_meta_flags{
    MV_FLG_BIT_PROMOTE = 0x8000,
    MV_FLG_BIT_CAPTURE = 0x4000,
}

const MV_SHFT_TO_SQ: u16 = 0;
const MV_SHFT_FROM_SQ: u16 = 6;

const MV_MASK_TO_SQ: u16 = 0x003F;
const MV_MASK_FROM_SQ: u16 = 0x0FC0;
const MV_MASK_FLAGS: u16 = 0xF000;



pub fn encode_move_quiet(from_sq: Square, to_sq: Square) -> Move {
    let from = from_sq as u16;
    let to = to_sq as u16;

    let mut mv: u16 = from.shl(MV_SHFT_FROM_SQ);
    mv |= mv & MV_MASK_FROM_SQ;

    mv |= to.shl(MV_SHFT_TO_SQ);
    mv |= mv & MV_MASK_TO_SQ;

    return mv;
}

pub fn encode_move_with_promotion(
    from_sq: Square,
    to_sq: Square,
    promotionPiece: Piece) -> Move {
        
    let mut mov = encode_move_quiet(from_sq, to_sq);

    match promotionPiece {
        Piece::WKnight => mov = mov | move_flags::MV_FLG_PROMOTE_KNIGHT,
        Piece::BKnight => mov = mov | move_flags::MV_FLG_PROMOTE_KNIGHT,
        Piece::WBishop => mov = mov | move_flags::MV_FLG_PROMOTE_BISHOP,
        Piece::BBishop => mov = mov | move_flags::MV_FLG_PROMOTE_BISHOP,
        Piece::WRook => mov = mov | move_flags::MV_FLG_PROMOTE_ROOK,
        Piece::BRook => mov = mov | move_flags::MV_FLG_PROMOTE_ROOK,
        Piece::WQueen => mov = mov | move_flags::MV_FLG_PROMOTE_QUEEN,
        Piece::BQueen => mov = mov | move_flags::MV_FLG_PROMOTE_QUEEN,
        _ => panic!("Invalid promotion type")
    }
    return mov;
    
}

pub fn encode_move_with_promotion_capture(
    from_sq: Square,
    to_sq: Square,
    promotionPiece: Piece) -> Move {
        
    let mut mov = encode_move_with_promotion(from_sq, to_sq, promotionPiece);
    mov = mov | move_flags::MV_FLG_BIT_CAPTURE;

    return mov;    
}

pub fn encode_move_en_passant(
    from_sq: Square,
    to_sq: Square) -> Move {
        

    let mut mov = encode_move_quiet(from_sq, to_sq);
    mov = mov | move_flags::MV_FLG_EN_PASS;

    return mov;    
}

pub fn encode_move_double_pawn_first(
    from_sq: Square,
    to_sq: Square) -> Move {
        
    let mut mov = encode_move_quiet(from_sq, to_sq);
    mov = mov | move_flags::MV_FLG_DOUBLE_PAWN;

    return mov;    
}


pub fn encode_move_castle_kingside_white() -> Move
{
    let mut mov = encode_move_quiet( Square::e1, Square::g1 );
    mov = mov | move_flags::MV_FLG_KING_CASTLE;
    return mov;
}

pub fn encode_move_castle_kingside_black() -> Move
{
    let mut mov = encode_move_quiet( Square::e8, Square::g8 );
    mov = mov | move_flags::MV_FLG_KING_CASTLE;
    return mov;
}

pub fn encode_move_castle_queenside_white() -> Move
{
    let mut mov = encode_move_quiet( Square::e1, Square::c1 );
    mov = mov | move_flags::MV_FLG_QUEEN_CASTLE;
    return mov;
}

pub fn encode_move_castle_queenside_black() -> Move
{
    let mut mov = encode_move_quiet( Square::e8, Square::c8 );
    mov = mov | move_flags::MV_FLG_QUEEN_CASTLE;
    return mov;
}

pub fn decode_from_square(mv: Move) -> Square{
    let sq= (mv & MV_MASK_FROM_SQ ).shr(MV_SHFT_FROM_SQ );

    return Square::get_square(sq);
}

pub fn decode_to_square(mv: Move) -> Square{
    let sq= (mv & MV_MASK_TO_SQ ).shr(MV_SHFT_TO_SQ );

    return Square::get_square(sq);
}

pub fn decode_promotion_piece(mv: Move, side: Colour) -> Piece{
    let masked = mv & MV_MASK_FLAGS;

    match side {
        Colour::White => {
            match masked {
                move_flags::MV_FLG_PROMOTE_KNIGHT_CAPTURE | move_flags::MV_FLG_PROMOTE_KNIGHT => return Piece::WKnight, 
                move_flags::MV_FLG_PROMOTE_BISHOP_CAPTURE | move_flags::MV_FLG_PROMOTE_BISHOP => return Piece::WBishop,
                move_flags::MV_FLG_PROMOTE_QUEEN_CAPTURE | move_flags::MV_FLG_PROMOTE_QUEEN => return Piece::WQueen,
                move_flags::MV_FLG_PROMOTE_ROOK_CAPTURE | move_flags::MV_FLG_PROMOTE_ROOK => return Piece::WRook,
                _ => panic!("Invalid WHITE promotion piece"),
            }
        },
        Colour::Black => {
            match masked {
                move_flags::MV_FLG_PROMOTE_KNIGHT_CAPTURE | move_flags::MV_FLG_PROMOTE_KNIGHT => return Piece::BKnight, 
                move_flags::MV_FLG_PROMOTE_BISHOP_CAPTURE | move_flags::MV_FLG_PROMOTE_BISHOP => return Piece::BBishop,
                move_flags::MV_FLG_PROMOTE_QUEEN_CAPTURE | move_flags::MV_FLG_PROMOTE_QUEEN => return Piece::BQueen,
                move_flags::MV_FLG_PROMOTE_ROOK_CAPTURE | move_flags::MV_FLG_PROMOTE_ROOK => return Piece::BRook,
                _ => panic!("Invalid BLACK promotion piece"),
            }        
        }
    }
}


pub fn move_is_quiet(mv: Move) -> bool{
    let m = mv & MV_MASK_FLAGS;
    return m == move_flags::MV_FLG_QUIET;    
}

pub fn move_is_capture(mv: Move) -> bool{
    return ( mv & MV_FLG_BIT_CAPTURE ) != 0;
}

pub fn move_is_promote(mv: Move) -> bool{
    return ( mv & MV_FLG_BIT_PROMOTE ) != 0;
}

pub fn move_is_en_passant(mv: Move) -> bool{
    return ( mv & move_flags::MV_FLG_EN_PASS ) != 0;
}





#[cfg(test)]
mod tests {
    use super::extract_from_sq;
    use super::extract_promotion;
    use super::extract_to_sq;
    use super::set_move;
    use super::set_move_with_promotion;
    use super::Move;
    use super::Promotion;
    use square::Square;

    #[test]
    pub fn test_constructor_no_promotion() {
        let mut fsq = Square::a3;
        let mut tsq = Square::b3;
        let mut mv: Move = 0;
        set_move(fsq, tsq, &mut mv);
        let mut from_sq = extract_from_sq(mv);
        let mut to_sq = extract_to_sq(mv);
        assert_eq!(from_sq, fsq);
        assert_eq!(to_sq, tsq);

        fsq = Square::h6;
        tsq = Square::b6;
        mv = 0;
        set_move(fsq, tsq, &mut mv);
        from_sq = extract_from_sq(mv);
        to_sq = extract_to_sq(mv);
        assert_eq!(from_sq, fsq);
        assert_eq!(to_sq, tsq);
    }

    #[test]
    pub fn test_constructor_with_promotion_knight() {
        let fsq = Square::a3;
        let tsq = Square::b3;
        let prom = Promotion::Knight;
        let mut mv: Move = 0;
        set_move_with_promotion(fsq, tsq, prom, &mut mv);
        let from_sq = extract_from_sq(mv);
        let to_sq = extract_to_sq(mv);
        let promotion = extract_promotion(mv).unwrap();

        assert_eq!(from_sq, fsq);
        assert_eq!(to_sq, tsq);
        assert_eq!(promotion, prom);
    }

    #[test]
    pub fn test_constructor_with_promotion_bishop() {
        let fsq = Square::a3;
        let tsq = Square::b3;
        let prom = Promotion::Bishop;
        let mut mv: Move = 0;
        set_move_with_promotion(fsq, tsq, prom, &mut mv);
        let from_sq = extract_from_sq(mv);
        let to_sq = extract_to_sq(mv);
        let promotion = extract_promotion(mv).unwrap();

        assert_eq!(from_sq, fsq);
        assert_eq!(to_sq, tsq);
        assert_eq!(promotion, prom);
    }

    #[test]
    pub fn test_constructor_with_promotion_rook() {
        let fsq = Square::a3;
        let tsq = Square::b3;
        let prom = Promotion::Rook;
        let mut mv: Move = 0;
        set_move_with_promotion(fsq, tsq, prom, &mut mv);
        let from_sq = extract_from_sq(mv);
        let to_sq = extract_to_sq(mv);
        let promotion = extract_promotion(mv).unwrap();

        assert_eq!(from_sq, fsq);
        assert_eq!(to_sq, tsq);
        assert_eq!(promotion, prom);
    }

    #[test]
    pub fn test_constructor_with_promotion_queen() {
        let fsq = Square::a3;
        let tsq = Square::b3;
        let prom = Promotion::Queen;
        let mut mv: Move = 0;
        set_move_with_promotion(fsq, tsq, prom, &mut mv);
        let from_sq = extract_from_sq(mv);
        let to_sq = extract_to_sq(mv);
        let promotion = extract_promotion(mv).unwrap();

        assert_eq!(from_sq, fsq);
        assert_eq!(to_sq, tsq);
        assert_eq!(promotion, prom);
    }
}
