use board::piece::Colour;

pub type CastlePermission = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Offset {
    WhiteKing = 0b00000000,
    WhiteQueen = 0b00010000,
    BlackKing = 0b00100000,
    BlackQueen = 0b00110000,
}

const OFFSET_MASK: u8 = 0b11110000;
const OFFSET_SHIFT: u8 = 4;
const PERM_MASK: u8 = !OFFSET_MASK;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CastlePermissionType {
    WhiteKing = (0b00000001 | Offset::WhiteKing as u8),
    WhiteQueen = (0b00000010 | Offset::WhiteQueen as u8),
    BlackKing = (0b00000100 | Offset::BlackKing as u8),
    BlackQueen = (0b00001000 | Offset::BlackQueen as u8),
}

pub const NUM_CASTLE_PERMS: usize = 4;

pub const NO_CASTLE_PERMS: u8 = 0;

pub fn has_castle_permission(perm: CastlePermission) -> bool {
    return (perm & PERM_MASK) != NO_CASTLE_PERMS;
}

pub fn set_king(perm: &mut CastlePermission, colour: Colour) {
    let mut cp = *perm;
    match colour {
        Colour::White => cp = (cp & PERM_MASK) | CastlePermissionType::WhiteKing as u8,
        Colour::Black => cp = (cp & PERM_MASK) | CastlePermissionType::BlackKing as u8,
    }
    *perm = cp;
}

pub fn clear_king_and_queen(perm: &mut CastlePermission, colour: Colour) {
    let mut cp = *perm;
    match colour {
        Colour::White => {
            cp = cp & !(CastlePermissionType::WhiteKing as u8);
            cp = cp & !(CastlePermissionType::WhiteQueen as u8);
        }

        Colour::Black => {
            cp = cp & !(CastlePermissionType::BlackKing as u8);
            cp = cp & !(CastlePermissionType::BlackQueen as u8);
        }
    }
    *perm = cp;
}

pub fn clear_king_black(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp = cp & !(CastlePermissionType::BlackKing as u8);
    *perm = cp;
}

pub fn clear_king_white(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp = cp & !(CastlePermissionType::WhiteKing as u8);
    *perm = cp;
}

pub fn is_king_set(perm: CastlePermission, colour: Colour) -> bool {
    match colour {
        Colour::White => return (perm & PERM_MASK) & CastlePermissionType::WhiteKing as u8 != 0,
        Colour::Black => return (perm & PERM_MASK) & CastlePermissionType::BlackKing as u8 != 0,
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

pub fn has_white_castle_permission(perm: CastlePermission) -> bool {
    return is_king_set(perm, Colour::White) || is_queen_set(perm, Colour::White);
}

pub fn has_black_castle_permission(perm: CastlePermission) -> bool {
    return is_king_set(perm, Colour::Black) || is_queen_set(perm, Colour::Black);
}

pub fn clear_queen_black(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp = cp & !(CastlePermissionType::BlackQueen as u8);
    *perm = cp;
}

pub fn clear_queen_white(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp = cp & !(CastlePermissionType::WhiteQueen as u8);
    *perm = cp;
}

pub fn is_queen_set(perm: CastlePermission, colour: Colour) -> bool {
    match colour {
        Colour::White => return (perm & PERM_MASK) & CastlePermissionType::WhiteQueen as u8 != 0,
        Colour::Black => return (perm & PERM_MASK) & CastlePermissionType::BlackQueen as u8 != 0,
    }
}

pub fn to_offset(perm_type: CastlePermissionType) -> usize {
    return (((perm_type as u8) & OFFSET_MASK) >> OFFSET_SHIFT) as usize;
}

#[cfg(test)]
pub mod tests {
    use board::piece::Colour;
    use position::castle_permissions;
    use position::castle_permissions::CastlePermissionType;

    #[test]
    pub fn default_castle_permissisons_none_set() {
        let cp = castle_permissions::NO_CASTLE_PERMS;

        assert!(castle_permissions::is_king_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_king_set(cp, Colour::Black) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::White) == false);
        assert!(castle_permissions::is_queen_set(cp, Colour::Black) == false);
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
