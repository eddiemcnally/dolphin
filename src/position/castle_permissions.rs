use board::piece::Colour;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct CastlePermission {
    has_white_king: bool,
    has_white_queen: bool,
    has_black_king: bool,
    has_black_queen: bool,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CastlePermissionType {
    WhiteKing = 0,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}

pub const NUM_CASTLE_PERMS: usize = 4;

impl CastlePermission {
    pub fn new() -> CastlePermission {
        CastlePermission {
            has_white_king: false,
            has_white_queen: false,
            has_black_king: false,
            has_black_queen: false,
        }
    }

    pub fn has_castle_permission(&self) -> bool {
        return self.has_black_king == true
            || self.has_black_queen == true
            || self.has_white_king == true
            || self.has_white_queen == true;
    }

    pub fn set_king(&mut self, colour: Colour, state: bool) {
        match colour {
            Colour::White => self.has_white_king = state,
            Colour::Black => self.has_black_king = state,
        }
    }
    pub fn is_king_set(&self, colour: Colour) -> bool {
        match colour {
            Colour::White => self.has_white_king == true,
            Colour::Black => self.has_black_king == true,
        }
    }

    pub fn set_queen(&mut self, colour: Colour, state: bool) {
        match colour {
            Colour::White => self.has_white_queen = state,
            Colour::Black => self.has_black_queen = state,
        }
    }
    pub fn is_queen_set(&self, colour: Colour) -> bool {
        match colour {
            Colour::White => self.has_white_queen == true,
            Colour::Black => self.has_black_queen == true,
        }
    }

    pub fn offset(castle_perm_type: CastlePermissionType) -> usize {
        match castle_perm_type {
            CastlePermissionType::WhiteKing => 0,
            CastlePermissionType::WhiteQueen => 1,
            CastlePermissionType::BlackKing => 2,
            CastlePermissionType::BlackQueen => 3,
        }
    }
}
