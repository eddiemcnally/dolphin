#[allow(dead_code)]
use board::square::Square;
use std::mem::transmute;
use std::ops::Shl;
use std::ops::Shr;



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

//#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Promotion {
    Knight,
    Bishop,
    Rook,
    Queen,
}


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
const MV_FLG_PROMOTE_KNIGHT_CAPTURE: u16 = 0xC000;
const MV_FLG_PROMOTE_BISHOP_CAPTURE: u16 = 0xD000;
const MV_FLG_PROMOTE_ROOK_CAPTURE: u16 = 0xE000;
const MV_FLG_PROMOTE_QUEEN_CAPTURE: u16 = 0xF000;

// meta-bit flags
const MV_FLG_BIT_PROMOTE: u16 = 0x8000;
const MV_FLG_BIT_CAPTURE: u16 = 0x4000;

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
    promotion: Promotion) -> Move {
        
    let mut mov = encode_move_quiet(from_sq, to_sq);

    match promotion {
        Promotion::Knight => mov = mov | MV_FLG_PROMOTE_KNIGHT,
        Promotion::Bishop => mov = mov | MV_FLG_PROMOTE_BISHOP,
        Promotion::Rook => mov = mov | MV_FLG_PROMOTE_ROOK,
        Promotion::Queen => mov = mov | MV_FLG_PROMOTE_QUEEN,
    }
    return mov;
    
}

pub fn encode_move_with_promotion_capture(
    from_sq: Square,
    to_sq: Square,
    promotion: Promotion) -> Move {
        
    let mut mov = encode_move_with_promotion(from_sq, to_sq, promotion);
    mov = mov | MV_FLG_BIT_CAPTURE;

    return mov;    
}

pub fn encode_move_en_passant(
    from_sq: Square,
    to_sq: Square) -> Move {
        

    let mut mov = encode_move_quiet(from_sq, to_sq);
    mov = mov | MV_FLG_EN_PASS;

    return mov;    
}

pub fn encode_move_double_pawn_first(
    from_sq: Square,
    to_sq: Square) -> Move {
        
    let mut mov = encode_move_quiet(from_sq, to_sq);
    mov = mov | MV_FLG_DOUBLE_PAWN;

    return mov;    
}


pub fn encode_move_castle_kingside_white() -> Move
{
    let mut mov = encode_move_quiet( Square::e1, Square::g1 );
    mov = mov | MV_FLG_KING_CASTLE;
    return mov;
}

pub fn encode_move_castle_kingside_black() -> Move
{
    let mut mov = encode_move_quiet( Square::e8, Square::g8 );
    mov = mov | MV_FLG_KING_CASTLE;
    return mov;
}

pub fn encode_move_castle_queenside_white() -> Move
{
    let mut mov = encode_move_quiet( Square::e1, Square::c1 );
    mov = mov | MV_FLG_QUEEN_CASTLE;
    return mov;
}

pub fn encode_move_castle_queenside_black() -> Move
{
    let mut mov = encode_move_quiet( Square::e8, Square::c8 );
    mov = mov | MV_FLG_QUEEN_CASTLE;
    return mov;
}


// enum square move_decode_from_sq ( const uint16_t mv );
// enum square move_decode_to_sq ( const uint16_t mv );
// enum piece move_decode_promotion_piece ( const uint16_t mv , const enum colour side);
// bool move_is_quiet ( const uint16_t mv );
// bool move_is_capture ( const uint16_t mv );
// bool move_is_promotion ( const uint16_t mv );
// bool move_is_en_passant ( const uint16_t mv );

// char *move_print ( uint16_t mv );

// bool validate_move ( const uint16_t mv );



pub fn extract_from_sq(mv: Move) -> Square {
    let fsq = mv & BITMASK_FROM_SQ;
    let sq: Square = unsafe { transmute(fsq as u8) };
    return sq;
}

pub fn extract_to_sq(mv: Move) -> Square {
    let mut tsq = mv & BITMASK_TO_SQ;
    tsq = tsq.shr(OFFSET_TO_SQ);
    let sq: Square = unsafe { transmute(tsq as u8) };
    return sq;
}

pub fn extract_promotion(mv: Move) -> Option<Promotion> {
    let prom = mv & BITMASK_PROMOTION;

    match prom {
        0 => None,
        BITMASK_PROM_KNIGHT => Some(Promotion::Knight),
        BITMASK_PROM_BISHOP => Some(Promotion::Bishop),
        BITMASK_PROM_ROOK => Some(Promotion::Rook),
        BITMASK_PROM_QUEEN => Some(Promotion::Queen),
        _ => panic!("INvalid promotion {:?}", prom),
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
