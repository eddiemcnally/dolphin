#[allow(dead_code)]

use board::square::Square;
use std::mem::transmute;
use std::ops::Shl;
use std::ops::Shr;

 
pub type Move = u16;

#[derive(Eq, PartialEq, Hash)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Promotion {
    Knight,
    Bishop,
    Rook,
    Queen,
}

// Bit representation as follows
// 		0000 0000 0011 1111		From square
//		0000 1111 1100 0000		To square
//		0001 0000 0000 0000		Promote to Knight
//		0010 0000 0000 0000		Promote to Bishop
//		0100 0000 0000 0000		Promote to Rook
//		1000 0000 0000 0000		Promote to Queen

const BITMASK_FROM_SQ: u16 = 0x003F;
const OFFSET_FROM_SQ: u16 = 0;
const BITMASK_TO_SQ: u16 = 0x0FC0;
const OFFSET_TO_SQ: u16 = 6;
const BITMASK_PROM_KNIGHT: u16 = 0x1000;
const BITMASK_PROM_BISHOP: u16 = 0x2000;
const BITMASK_PROM_ROOK: u16 = 0x4000;
const BITMASK_PROM_QUEEN: u16 = 0x8000;
const BITMASK_PROMOTION: u16 = 0xF000;

pub fn set_move(from_sq: Square, to_sq: Square, mv: &mut Move) {
    let mut m: u16 = 0;
    m = m | (from_sq as u16);
    m = m | (to_sq as u16).shl(OFFSET_TO_SQ);
    *mv = m;
}

pub fn set_move_with_promotion(
    from_sq: Square,
    to_sq: Square,
    promotion: Promotion,
    mut mv: &mut Move,
) {
    set_move(from_sq, to_sq, &mut mv);

    match promotion {
        Promotion::Knight => *mv = *mv | BITMASK_PROM_KNIGHT,
        Promotion::Bishop => *mv = *mv | BITMASK_PROM_BISHOP,
        Promotion::Rook => *mv = *mv | BITMASK_PROM_ROOK,
        Promotion::Queen => *mv = *mv | BITMASK_PROM_QUEEN,		
    }
}

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
}



#[cfg(test)]
mod tests {
    use super::set_move;
    use super::extract_from_sq;
    use super::extract_to_sq;
    use super::set_move_with_promotion;
    use super::extract_promotion;
    use super::Move;
    use square::Square;
    use super::Promotion;

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
