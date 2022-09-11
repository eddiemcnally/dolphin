use std::ops::{BitAnd, BitOr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CastlePermission(u8);

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[rustfmt::skip]
pub enum CastlePermissionType {
    WhiteKing   = 0x01,
    WhiteQueen  = 0x01 << 1,
    BlackKing   = 0x01 << 2,
    BlackQueen  = 0x01 << 3,
}

impl CastlePermissionType {
    pub const fn to_offset(self) -> usize {
        self as usize
    }
}

impl CastlePermission {
    pub const NUM_CASTLE_PERMS: usize = 4;

    pub const NO_CASTLE_PERMS_AVAIL: CastlePermission = CastlePermission(0);

    pub fn has_castle_permission(&self) -> bool {
        *self != CastlePermission::NO_CASTLE_PERMS_AVAIL
    }

    pub fn set_black_king(&mut self) {
        self.0 |= CastlePermissionType::BlackKing.to_offset() as u8;
    }

    pub fn set_white_king(&mut self) {
        self.0 |= CastlePermissionType::WhiteKing.to_offset() as u8;
    }

    pub fn clear_white_king_and_queen(&mut self) {
        self.0 &= !CastlePermissionType::WhiteKing.to_offset() as u8;
        self.0 &= !CastlePermissionType::WhiteQueen.to_offset() as u8;
    }

    pub fn clear_black_king_and_queen(&mut self) {
        self.0 &= !CastlePermissionType::BlackKing.to_offset() as u8;
        self.0 &= !CastlePermissionType::BlackQueen.to_offset() as u8;
    }

    pub fn clear_king_black(&mut self) {
        self.0 &= !CastlePermissionType::BlackKing.to_offset() as u8;
    }

    pub fn clear_king_white(&mut self) {
        self.0 &= !CastlePermissionType::WhiteKing.to_offset() as u8;
    }

    pub fn is_white_king_set(&self) -> bool {
        self.0 & CastlePermissionType::WhiteKing.to_offset() as u8 != 0
    }

    pub fn is_black_king_set(&self) -> bool {
        self.0 & CastlePermissionType::BlackKing.to_offset() as u8 != 0
    }

    pub fn set_white_queen(&mut self) {
        self.0 |= CastlePermissionType::WhiteQueen.to_offset() as u8
    }

    pub fn set_black_queen(&mut self) {
        self.0 |= CastlePermissionType::BlackQueen.to_offset() as u8
    }

    pub fn has_white_castle_permission(&self) -> bool {
        self.is_white_king_set() || self.is_white_queen_set()
    }

    pub fn has_black_castle_permission(&self) -> bool {
        self.is_black_king_set() || self.is_black_queen_set()
    }

    pub fn clear_queen_black(&mut self) {
        self.0 &= !CastlePermissionType::BlackQueen.to_offset() as u8
    }

    pub fn clear_queen_white(&mut self) {
        self.0 &= !CastlePermissionType::WhiteQueen.to_offset() as u8
    }

    pub fn is_white_queen_set(&self) -> bool {
        self.0 & CastlePermissionType::WhiteQueen.to_offset() as u8 != 0
    }

    pub fn is_black_queen_set(&self) -> bool {
        self.0 & CastlePermissionType::BlackQueen.to_offset() as u8 != 0
    }

    pub const fn to_offset(perm_type: CastlePermissionType) -> usize {
        match perm_type {
            CastlePermissionType::WhiteKing => 0,
            CastlePermissionType::WhiteQueen => 1,
            CastlePermissionType::BlackKing => 2,
            CastlePermissionType::BlackQueen => 3,
        }
    }
}

impl BitAnd for CastlePermission {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        CastlePermission(self.0 & other.0)
    }
}

impl BitOr for CastlePermission {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        CastlePermission(self.0 | other.0)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::position::castle_permissions::{CastlePermission, CastlePermissionType};

    #[test]
    pub fn default_castle_permissisons_none_set() {
        let cp = CastlePermission::NO_CASTLE_PERMS_AVAIL;

        assert!(!cp.is_white_king_set());
        assert!(!cp.is_black_king_set());
        assert!(!cp.is_white_queen_set());
        assert!(!cp.is_black_queen_set());
        assert!(!cp.has_castle_permission());
    }

    #[test]
    pub fn castle_permissison_offsets() {
        assert!(CastlePermission::to_offset(CastlePermissionType::WhiteKing) == 0);
        assert!(CastlePermission::to_offset(CastlePermissionType::WhiteQueen) == 1);
        assert!(CastlePermission::to_offset(CastlePermissionType::BlackKing) == 2);
        assert!(CastlePermission::to_offset(CastlePermissionType::BlackQueen) == 3);
    }

    #[test]
    pub fn castle_permission_white_king_set_get_as_expected() {
        let mut cp = CastlePermission::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(!cp.has_castle_permission());

        cp.set_white_king();
        assert!(cp.is_white_king_set());
        assert!(cp.has_castle_permission());
        assert!(!cp.is_black_king_set());
        assert!(!cp.is_white_queen_set());
        assert!(!cp.is_black_queen_set());
    }

    #[test]
    pub fn castle_permission_black_king_set_get_as_expected() {
        let mut cp = CastlePermission::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(!cp.has_castle_permission());

        cp.set_black_king();
        assert!(cp.is_black_king_set());
        assert!(cp.has_castle_permission());
        assert!(!cp.is_white_king_set());
        assert!(!cp.is_white_queen_set());
        assert!(!cp.is_black_queen_set());
    }

    #[test]
    pub fn castle_permission_white_queen_set_get_as_expected() {
        let mut cp = CastlePermission::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(!cp.has_castle_permission());

        cp.set_white_queen();
        assert!(cp.is_white_queen_set());
        assert!(cp.has_castle_permission());
        assert!(!cp.is_black_king_set());
        assert!(!cp.is_black_queen_set());
        assert!(!cp.is_black_king_set());
    }

    #[test]
    pub fn castle_permission_black_queen_set_get_as_expected() {
        let mut cp = CastlePermission::NO_CASTLE_PERMS_AVAIL;

        // init condition
        assert!(!cp.has_castle_permission());

        cp.set_black_queen();
        assert!(cp.is_black_queen_set());
        assert!(cp.has_castle_permission());
        assert!(!cp.is_black_king_set());
        assert!(!cp.is_white_queen_set());
        assert!(!cp.is_black_king_set());
    }
}
