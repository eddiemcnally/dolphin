use board::piece::Colour;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct CastlePermission {
    has_white_king: bool,
    has_white_queen: bool,
    has_black_king: bool,
    has_black_queen: bool,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CastlePermissionType {
    WhiteKing,
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

#[cfg(test)]
pub mod tests {
    use board::piece::Colour;
    use position::castle_permissions::CastlePermission;
    use position::castle_permissions::CastlePermissionType;

    #[test]
    pub fn test_default_castle_permisisons_none_set() {
        let cp = CastlePermission::new();

        assert!(cp.is_king_set(Colour::White) == false);
        assert!(cp.is_king_set(Colour::Black) == false);
        assert!(cp.is_queen_set(Colour::White) == false);
        assert!(cp.is_queen_set(Colour::Black) == false);
        assert!(cp.has_castle_permission() == false);
    }

    #[test]
    pub fn test_castle_permisison_offsets() {
        assert!(CastlePermission::offset(CastlePermissionType::WhiteQueen) == 1);
        assert!(CastlePermission::offset(CastlePermissionType::WhiteKing) == 0);
        assert!(CastlePermission::offset(CastlePermissionType::BlackQueen) == 3);
        assert!(CastlePermission::offset(CastlePermissionType::BlackKing) == 2);
    }

    #[test]
    pub fn test_castle_permission_white_king_set_get_as_expected() {
        let mut cp = CastlePermission::new();

        // init condition
        assert!(cp.has_castle_permission() == false);

        cp.set_king(Colour::White, true);
        assert!(cp.is_king_set(Colour::White) == true);
        assert!(cp.has_castle_permission() == true);
        assert!(cp.is_king_set(Colour::Black) == false);
        assert!(cp.is_queen_set(Colour::White) == false);
        assert!(cp.is_queen_set(Colour::Black) == false);
    }

    #[test]
    pub fn test_castle_permission_black_king_set_get_as_expected() {
        let mut cp = CastlePermission::new();

        // init condition
        assert!(cp.has_castle_permission() == false);

        cp.set_king(Colour::Black, true);
        assert!(cp.is_king_set(Colour::Black) == true);
        assert!(cp.has_castle_permission() == true);
        assert!(cp.is_king_set(Colour::White) == false);
        assert!(cp.is_queen_set(Colour::White) == false);
        assert!(cp.is_queen_set(Colour::Black) == false);
    }

    #[test]
    pub fn test_castle_permission_white_queen_set_get_as_expected() {
        let mut cp = CastlePermission::new();

        // init condition
        assert!(cp.has_castle_permission() == false);

        cp.set_queen(Colour::White, true);
        assert!(cp.is_queen_set(Colour::White) == true);
        assert!(cp.has_castle_permission() == true);
        assert!(cp.is_king_set(Colour::Black) == false);
        assert!(cp.is_queen_set(Colour::Black) == false);
        assert!(cp.is_king_set(Colour::Black) == false);
    }

    #[test]
    pub fn test_castle_permission_black_queen_set_get_as_expected() {
        let mut cp = CastlePermission::new();

        // init condition
        assert!(cp.has_castle_permission() == false);

        cp.set_queen(Colour::Black, true);
        assert!(cp.is_queen_set(Colour::Black) == true);
        assert!(cp.has_castle_permission() == true);
        assert!(cp.is_king_set(Colour::Black) == false);
        assert!(cp.is_queen_set(Colour::White) == false);
        assert!(cp.is_king_set(Colour::Black) == false);
    }

}
