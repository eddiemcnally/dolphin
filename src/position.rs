use board::board::Board;
use board::piece::Colour;
use board::square::Square;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum CastlePermissionBitMap {
    WK = 0x01,
    WQ = 0x02,
    BK = 0x04,
    BQ = 0x08,
}
pub type CastlePermission = u8;

impl CastlePermissionBitMap {
    pub fn set_perm(perm: CastlePermissionBitMap, perm_map: CastlePermission) -> CastlePermission {
        return perm as CastlePermission | perm_map;
    }

    pub fn clear_perm(
        perm: CastlePermissionBitMap,
        perm_map: CastlePermission,
    ) -> CastlePermission {
        return !(perm as CastlePermission) & perm_map;
    }

    pub fn is_perm_set(perm: CastlePermissionBitMap, perm_map: CastlePermission) -> bool {
        return perm as CastlePermission & perm_map > 0;
    }
}

struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

pub struct Position {
    // pieces and squares
    pub board: Board,
    // side to move
    side_to_move: Colour,
    // the en passant square
    en_pass_sq: Option<Square>,
    // castle permissions
    castle_perm: Option<CastlePermission>,

    move_cntr: MoveCounter,
}

impl Position {
    pub fn set_board(&mut self, brd: Board) {
        self.board = brd;
    }
    pub fn get_side_to_move(&self) -> Colour {
        self.side_to_move
    }
}

#[cfg(test)]
mod tests {}
