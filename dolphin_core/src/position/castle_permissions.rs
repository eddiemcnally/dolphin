use std::ops::{BitAnd, BitOr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CastlePermission(u8);

// Bit fields for CastlePermission
// ---- ---X    White King
// ---- --X-    White Queen
// ---- -X--    Black King
// ---- X---    Black Queen
//
#[rustfmt::skip]
const MASK_WHITE_KING: u8 = 0b0000_0001;
const MASK_WHITE_QUEEN: u8 = 0b0000_0010;
const MASK_BLACK_KING: u8 = 0b0000_0100;
const MASK_BLACK_QUEEN: u8 = 0b0000_1000;

const MASK_WHITE: u8 = 0b0000_0011;
const MASK_BLACK: u8 = 0b0000_1100;

enum AsOffset {
    WhiteKing,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}

impl CastlePermission {
    pub const NUM_CASTLE_PERMS: usize = 4;

    pub const NO_CASTLE_PERMS_AVAIL: CastlePermission = CastlePermission(0);

    pub fn has_castle_permission(&self) -> bool {
        *self != CastlePermission::NO_CASTLE_PERMS_AVAIL
    }

    pub fn set_black_king(&mut self) {
        self.0 |= MASK_BLACK_KING;
    }

    pub fn set_white_king(&mut self) {
        self.0 |= MASK_WHITE_KING;
    }

    pub fn clear_white_king_and_queen(&mut self) {
        self.0 &= !MASK_WHITE;
    }

    pub fn clear_black_king_and_queen(&mut self) {
        self.0 &= !MASK_BLACK;
    }

    pub fn clear_king_black(&mut self) {
        self.0 &= !MASK_BLACK_KING;
    }

    pub fn clear_king_white(&mut self) {
        self.0 &= !MASK_WHITE_KING;
    }

    pub const fn is_white_king_set(&self) -> bool {
        self.0 & MASK_WHITE_KING != 0
    }

    pub const fn is_black_king_set(&self) -> bool {
        self.0 & MASK_BLACK_KING != 0
    }

    pub fn set_white_queen(&mut self) {
        self.0 |= MASK_WHITE_QUEEN
    }

    pub fn set_black_queen(&mut self) {
        self.0 |= MASK_BLACK_QUEEN
    }

    pub const fn has_white_castle_permission(&self) -> bool {
        self.0 & MASK_WHITE != 0
    }

    pub const fn has_black_castle_permission(&self) -> bool {
        self.0 & MASK_BLACK != 0
    }

    pub fn clear_queen_black(&mut self) {
        self.0 &= !MASK_BLACK_QUEEN
    }

    pub fn clear_queen_white(&mut self) {
        self.0 &= !MASK_WHITE_QUEEN
    }

    pub const fn is_white_queen_set(&self) -> bool {
        self.0 & MASK_WHITE_QUEEN != 0
    }

    pub const fn is_black_queen_set(&self) -> bool {
        self.0 & MASK_BLACK_QUEEN != 0
    }

    pub const fn white_king_offset() -> usize {
        AsOffset::WhiteKing as usize
    }

    pub const fn white_queen_offset() -> usize {
        AsOffset::WhiteQueen as usize
    }

    pub const fn black_king_offset() -> usize {
        AsOffset::BlackKing as usize
    }

    pub const fn black_queen_offset() -> usize {
        AsOffset::BlackQueen as usize
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
    use crate::position::castle_permissions::CastlePermission;

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
