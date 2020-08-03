pub type CastlePermission = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Offset {
    WhiteKing = 0b0000_0000,
    WhiteQueen = 0b0001_0000,
    BlackKing = 0b0010_0000,
    BlackQueen = 0b0011_0000,
}

const OFFSET_MASK: u8 = 0b1111_0000;
const OFFSET_SHIFT: u8 = 4;
const PERM_MASK: u8 = !OFFSET_MASK;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CastlePermissionType {
    WhiteKing = 0b0000_0001 | Offset::WhiteKing as u8,
    WhiteQueen = 0b0000_0010 | Offset::WhiteQueen as u8,
    BlackKing = 0b0000_0100 | Offset::BlackKing as u8,
    BlackQueen = 0b0000_1000 | Offset::BlackQueen as u8,
}

pub const NUM_CASTLE_PERMS: usize = 4;

pub const NO_CASTLE_PERMS: u8 = 0;

pub fn has_castle_permission(perm: CastlePermission) -> bool {
    (perm & PERM_MASK) != NO_CASTLE_PERMS
}

pub fn set_black_king(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp = (cp & PERM_MASK) | CastlePermissionType::BlackKing as u8;
    *perm = cp;
}

pub fn set_white_king(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp = (cp & PERM_MASK) | CastlePermissionType::WhiteKing as u8;
    *perm = cp;
}

pub fn clear_white_king_and_queen(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp &= !(CastlePermissionType::WhiteKing as u8);
    cp &= !(CastlePermissionType::WhiteQueen as u8);
    *perm = cp;
}

pub fn clear_black_king_and_queen(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp &= !(CastlePermissionType::BlackKing as u8);
    cp &= !(CastlePermissionType::BlackQueen as u8);
    *perm = cp;
}

pub fn clear_king_black(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp &= !(CastlePermissionType::BlackKing as u8);
    *perm = cp;
}

pub fn clear_king_white(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp &= !(CastlePermissionType::WhiteKing as u8);
    *perm = cp;
}

pub fn is_white_king_set(perm: CastlePermission) -> bool {
    (perm & PERM_MASK) & CastlePermissionType::WhiteKing as u8 != 0
}

pub fn is_black_king_set(perm: CastlePermission) -> bool {
    (perm & PERM_MASK) & CastlePermissionType::BlackKing as u8 != 0
}

pub fn set_white_queen(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp |= CastlePermissionType::WhiteQueen as u8;
    *perm = cp;
}

pub fn set_black_queen(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp |= CastlePermissionType::BlackQueen as u8;
    *perm = cp;
}

pub fn has_white_castle_permission(perm: CastlePermission) -> bool {
    is_white_king_set(perm) || is_white_queen_set(perm)
}

pub fn has_black_castle_permission(perm: CastlePermission) -> bool {
    is_black_king_set(perm) || is_black_queen_set(perm)
}

pub fn clear_queen_black(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp &= !(CastlePermissionType::BlackQueen as u8);
    *perm = cp;
}

pub fn clear_queen_white(perm: &mut CastlePermission) {
    let mut cp = *perm;
    cp &= !(CastlePermissionType::WhiteQueen as u8);
    *perm = cp;
}

pub fn is_white_queen_set(perm: CastlePermission) -> bool {
    (perm & PERM_MASK) & CastlePermissionType::WhiteQueen as u8 != 0
}

pub fn is_black_queen_set(perm: CastlePermission) -> bool {
    (perm & PERM_MASK) & CastlePermissionType::BlackQueen as u8 != 0
}

pub fn to_offset(perm_type: CastlePermissionType) -> usize {
    (((perm_type as u8) & OFFSET_MASK) >> OFFSET_SHIFT) as usize
}

#[cfg(test)]
pub mod tests {
    use engine::castle_permissions;
    use engine::castle_permissions::CastlePermissionType;

    #[test]
    pub fn default_castle_permissisons_none_set() {
        let cp = castle_permissions::NO_CASTLE_PERMS;

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
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_white_king(&mut cp);
        assert!(castle_permissions::is_white_king_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
    }

    #[test]
    pub fn castle_permission_black_king_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_black_king(&mut cp);
        assert!(castle_permissions::is_black_king_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_white_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
    }

    #[test]
    pub fn castle_permission_white_queen_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_white_queen(&mut cp);
        assert!(castle_permissions::is_white_queen_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_black_queen_set(cp) == false);
        assert!(castle_permissions::is_black_king_set(cp) == false);
    }

    #[test]
    pub fn castle_permission_black_queen_set_get_as_expected() {
        let mut cp = castle_permissions::NO_CASTLE_PERMS;

        // init condition
        assert!(castle_permissions::has_castle_permission(cp) == false);

        castle_permissions::set_black_queen(&mut cp);
        assert!(castle_permissions::is_black_queen_set(cp) == true);
        assert!(castle_permissions::has_castle_permission(cp) == true);
        assert!(castle_permissions::is_black_king_set(cp) == false);
        assert!(castle_permissions::is_white_queen_set(cp) == false);
        assert!(castle_permissions::is_black_king_set(cp) == false);
    }
}
