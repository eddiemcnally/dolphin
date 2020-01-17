use board::piece::Colour;

pub type CastlePermission = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CastlePermissionType {
    WhiteKing = 0x01,
    WhiteQueen = 0x02,
    BlackKing = 0x04,
    BlackQueen = 0x08,
}

pub const NUM_CASTLE_PERMS: usize = 4;

pub const NO_CASTLE_PERMS: u8 = 0;

pub fn has_castle_permission(perm: CastlePermission) -> bool {
    return perm != NO_CASTLE_PERMS;
}

pub fn set_king(perm: &mut CastlePermission, colour: Colour) {
    let mut cp = *perm;
    match colour {
        Colour::White => cp = cp | CastlePermissionType::WhiteKing as u8,
        Colour::Black => cp = cp | CastlePermissionType::BlackKing as u8,
    }
    *perm = cp;
}

pub fn clear_king(perm: &mut CastlePermission, colour: Colour) {
    let mut cp = *perm;
    match colour {
        Colour::White => cp = cp & !(CastlePermissionType::WhiteKing as u8),
        Colour::Black => cp = cp & !(CastlePermissionType::BlackKing as u8),
    }
    *perm = cp;
}

pub fn is_king_set(perm: CastlePermission, colour: Colour) -> bool {
    match colour {
        Colour::White => return perm & CastlePermissionType::WhiteKing as u8 != 0,
        Colour::Black => return perm & CastlePermissionType::BlackKing as u8 != 0,
    }
}

pub fn set_queen(perm: &mut CastlePermission, colour: Colour) {
    let mut cp = *perm;
    match colour {
        Colour::White => cp = cp | CastlePermissionType::WhiteQueen as u8,
        Colour::Black => cp = cp | CastlePermissionType::BlackQueen as u8,
    }
    *perm = cp;
}

pub fn clear_queen(perm: &mut CastlePermission, colour: Colour) {
    let mut cp = *perm;
    match colour {
        Colour::White => cp = cp & !(CastlePermissionType::WhiteQueen as u8),
        Colour::Black => cp = cp & !(CastlePermissionType::BlackQueen as u8),
    }
    *perm = cp;
}

pub fn is_queen_set(perm: CastlePermission, colour: Colour) -> bool {
    match colour {
        Colour::White => return perm & CastlePermissionType::WhiteQueen as u8 != 0,
        Colour::Black => return perm & CastlePermissionType::BlackQueen as u8 != 0,
    }
}

pub fn to_offset(perm_type: CastlePermissionType) -> usize {
    match perm_type {
        CastlePermissionType::WhiteQueen => return 0,
        CastlePermissionType::WhiteKing => return 1,
        CastlePermissionType::BlackQueen => return 2,
        CastlePermissionType::BlackKing => return 3,
    }
}

#[cfg(test)]
pub mod tests {
    use board::piece::Colour;
    use position::castle_permissions;
    use position::castle_permissions::CastlePermissionType;

    #[test]
    pub fn default_castle_permisisons_none_set() {
        let cp = castle_permissions::NO_CASTLE_PERMS;

        assert!(castle_permissions::is_king_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::Black) == false);
        assert!(castle_permissions::has_castle_permission(cp) == false);
    }

    #[test]
    pub fn castle_permisison_offsets() {
        assert!(castle_permissions::to_offset(CastlePermissionType::WhiteQueen) == 0);
        assert!(castle_permissions::to_offset(CastlePermissionType::WhiteKing) == 1);
        assert!(castle_permissions::to_offset(CastlePermissionType::BlackQueen) == 2);
        assert!(castle_permissions::to_offset(CastlePermissionType::BlackKing) == 3);
    }

    #[test]
    pub fn castle_permission_white_king_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_king(&mut cp, Colour::White);
        assert!(castle_permissions::is_king_set(cp, Colour::White) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::Black) == false);
    }

    #[test]
    pub fn castle_permission_black_king_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_king(&mut cp, Colour::Black);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_king_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::Black) == false);
    }

    #[test]
    pub fn castle_permission_white_queen_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_queen(&mut cp, Colour::White);
        assert!(castle_permissions::is_queen_set(cp, Colour::White) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::Black) == false);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
    }

    #[test]
    pub fn castle_permission_black_queen_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_queen(&mut cp, Colour::Black);
        assert!(castle_permissions::is_queen_set(cp, Colour::Black) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
    }
}
