use board::Board;
use piece::Colour;
use square::Square;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Debug)]
pub enum CastlePermissionBitMap {
    None = 0,
    WK = 0x01,
    WQ = 0x02,
    BK = 0x04,
    BQ = 0x08,
}
//pub type CastlePermission = u8;


impl CastlePermissionBitMap {
    pub fn set_perm(perm: CastlePermissionBitMap, perm_map: u8) -> u8 {
        return perm as u8 | perm_map;
    }

    pub fn clear_perm(perm: CastlePermissionBitMap, perm_map: u8) -> u8 {
        return !(perm as u8) & perm_map;
    }

    pub fn is_perm_set(perm: CastlePermissionBitMap, perm_map: u8) -> bool {
        return perm as u8 & perm_map > 0;
    }
}



struct MoveCounter {
    half_move: u16,
    full_move: u16,
}


pub struct Position {
    // pieces and squares
    board: Board,
    // side to move
    side_to_move: Colour,
    // the en passant square
    en_pass_sq: Square,
    // castle permissions
    castle_perm: u8,
}

#[cfg(test)]
mod tests {}
