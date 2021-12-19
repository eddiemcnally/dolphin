pub type CastlePermission = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[rustfmt::skip]
pub enum CastlePermissionType {
    WhiteKing   = 0x01,
    WhiteQueen  = 0x01 << 1,
    BlackKing   = 0x01 << 2,
    BlackQueen  = 0x01 << 3,
}

pub const NUM_CASTLE_PERMS: usize = 4;

pub const NO_CASTLE_PERMS_AVAIL: u8 = 0;

pub fn has_castle_permission(perm: CastlePermission) -> bool {
    perm != NO_CASTLE_PERMS_AVAIL
}

pub fn set_black_king(perm: CastlePermission) -> CastlePermission {
    (perm as u8 | CastlePermissionType::BlackKing as u8) as CastlePermission
}

pub fn set_white_king(perm: CastlePermission) -> CastlePermission {
    (perm as u8 | CastlePermissionType::WhiteKing as u8) as CastlePermission
}

pub fn clear_white_king_and_queen(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp &= !(CastlePermissionType::WhiteKing as u8);
    tp &= !(CastlePermissionType::WhiteQueen as u8);
    tp
}

pub fn clear_black_king_and_queen(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp &= !(CastlePermissionType::BlackKing as u8);
    tp &= !(CastlePermissionType::BlackQueen as u8);
    tp
}

pub fn clear_king_black(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp &= !(CastlePermissionType::BlackKing as u8);
    tp
}

pub fn clear_king_white(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp &= !(CastlePermissionType::WhiteKing as u8);
    tp
}

pub fn is_white_king_set(perm: CastlePermission) -> bool {
    perm as u8 & CastlePermissionType::WhiteKing as u8 != 0
}

pub fn is_black_king_set(perm: CastlePermission) -> bool {
    perm as u8 & CastlePermissionType::BlackKing as u8 != 0
}

pub fn set_white_queen(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp |= CastlePermissionType::WhiteQueen as u8;
    tp
}

pub fn set_black_queen(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp |= CastlePermissionType::BlackQueen as u8;
    tp
}

pub fn has_white_castle_permission(perm: CastlePermission) -> bool {
    is_white_king_set(perm) || is_white_queen_set(perm)
}

pub fn has_black_castle_permission(perm: CastlePermission) -> bool {
    is_black_king_set(perm) || is_black_queen_set(perm)
}

pub fn clear_queen_black(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp &= !(CastlePermissionType::BlackQueen as u8);
    tp
}

pub fn clear_queen_white(perm: CastlePermission) -> CastlePermission {
    let mut tp: CastlePermission = perm;
    tp &= !(CastlePermissionType::WhiteQueen as u8);
    tp
}

pub fn is_white_queen_set(perm: CastlePermission) -> bool {
    perm as u8 & CastlePermissionType::WhiteQueen as u8 != 0
}

pub fn is_black_queen_set(perm: CastlePermission) -> bool {
    perm as u8 & CastlePermissionType::BlackQueen as u8 != 0
}

pub const fn to_offset(perm_type: CastlePermissionType) -> usize {
    match perm_type {
        CastlePermissionType::WhiteKing => 0,
        CastlePermissionType::WhiteQueen => 1,
        CastlePermissionType::BlackKing => 2,
        CastlePermissionType::BlackQueen => 3,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::castle_permissions;
    use crate::castle_permissions::CastlePermissionType;

    #[test]
    pub fn default_castle_permissisons_none_set() {
        let cp = castle_permissions::NO_CASTLE_PERMS_AVAIL;

        assert!(castle_permissions::is_white_king_set(cp) == false);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
        assert!(castle_permissions::has_castle_permission(cp) == false);
    }

    #[test]
    pub fn castle_permissison_offsets() {
        assert!(castle_permissions::to_offset(CastlePermissionType::WhiteKing) == 0);
        assert!(castle_permissions::to_offset(CastlePermissionType::WhiteQueen) == 1);
        assert!(castle_permissions::to_offset(CastlePermissionType::BlackKing) == 2);
        assert!(castle_permissions::to_offset(CastlePermissionType::BlackQueen) == 3);
    }

    #[test]
    pub fn castle_permission_white_king_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        cp = castle_permissions::set_white_king(cp);
        assert!(castle_permissions::is_white_king_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
    }

    #[test]
    pub fn castle_permission_black_king_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        cp = castle_permissions::set_black_king(cp);
        assert!(castle_permissions::is_black_king_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_white_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
    }

    #[test]
    pub fn castle_permission_white_queen_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        cp = castle_permissions::set_white_queen(cp);
        assert!(castle_permissions::is_white_queen_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
        assert!(castle_permissions::is_black_king_set(cp) == false);
    }

    #[test]
    pub fn castle_permission_black_queen_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        cp = castle_permissions::set_black_queen(cp);
        assert!(castle_permissions::is_black_queen_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_king_set(cp) == false);
    }
}
