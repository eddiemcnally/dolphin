#[allow(dead_code)]

use square::Square;
use std::mem::transmute;
use std::ops::Shl;
use std::ops::Shr;


pub type Move = u16;

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

pub fn get_from_sq(mv: Move) -> Square {
    let fsq = mv & BITMASK_FROM_SQ;
    let sq: Square = unsafe { transmute(fsq as u8) };
    return sq;
}

pub fn get_to_sq(mv: Move) -> Square {
    let mut tsq = mv & BITMASK_TO_SQ;
    tsq = tsq.shr(OFFSET_TO_SQ);
    let sq: Square = unsafe { transmute(tsq as u8) };
    return sq;
}




#[cfg(test)]
mod tests {}
