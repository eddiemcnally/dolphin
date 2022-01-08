use crate::board::colour::Colour;
use rand::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::board::colour::NUM_COLOURS;
use crate::board::piece::{Piece, NUM_PIECE_TYPES};
use crate::board::square::{Square, NUM_SQUARES};
use crate::board::types::ToInt;
use crate::position::castle_permissions;
use crate::position::castle_permissions::{CastlePermissionType, NUM_CASTLE_PERMS};

pub type ZobristHash = u64;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ZobristKeys {
    piece_keys: [[[ZobristHash; NUM_PIECE_TYPES]; NUM_SQUARES]; NUM_COLOURS],
    side_key: ZobristHash,
    castle_keys: [ZobristHash; NUM_CASTLE_PERMS],
    en_passant_sq_keys: [ZobristHash; NUM_SQUARES],
}

impl Default for ZobristKeys {
    fn default() -> Self {
        ZobristKeys {
            piece_keys: [[[0; NUM_PIECE_TYPES]; NUM_SQUARES]; NUM_COLOURS],
            side_key: 0,
            castle_keys: [0; NUM_CASTLE_PERMS],
            en_passant_sq_keys: [0; NUM_SQUARES],
        }
    }
}

impl ZobristKeys {
    pub fn new() -> Box<ZobristKeys> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);

        let piece_keys = init_piece_keys(&mut rng);
        let castle_keys = init_castle_keys(&mut rng);
        let en_passant_sq_keys = init_en_passant_keys(&mut rng);
        let side_key = rng.next_u64();

        let keys = ZobristKeys {
            piece_keys,
            castle_keys,
            en_passant_sq_keys,
            side_key,
        };

        Box::new(keys)
    }

    pub const fn side(&self) -> ZobristHash {
        self.side_key
    }

    pub fn piece_square(&self, piece: Piece, colour: Colour, square: Square) -> ZobristHash {
        let pce_offset = piece.to_usize();
        let sq_offset = square.to_usize();
        let col_offset = colour.to_usize();
        self.piece_keys[col_offset][sq_offset][pce_offset]
    }

    pub fn en_passant(&self, square: Square) -> ZobristHash {
        let sq_offset = square.to_usize();
        self.en_passant_sq_keys[sq_offset]
    }

    pub const fn castle_permission(&self, perm_type: CastlePermissionType) -> ZobristHash {
        let perm_offset = castle_permissions::to_offset(perm_type);
        self.castle_keys[perm_offset]
    }
}

fn init_piece_keys(
    rng: &mut Xoshiro256PlusPlus,
) -> [[[ZobristHash; NUM_PIECE_TYPES]; NUM_SQUARES]; NUM_COLOURS] {
    let mut retval = [[[0u64; NUM_PIECE_TYPES]; NUM_SQUARES]; NUM_COLOURS];
    for element in retval.iter_mut().flat_map(|r| r.iter_mut()) {
        for i in element {
            *i = rng.next_u64();
        }
    }
    retval
}
fn init_castle_keys(rng: &mut Xoshiro256PlusPlus) -> [ZobristHash; NUM_CASTLE_PERMS] {
    let mut retval = [0u64; NUM_CASTLE_PERMS];
    for item in retval.iter_mut().take(NUM_CASTLE_PERMS) {
        let seed = rng.next_u64();
        *item = seed;
    }
    retval
}
fn init_en_passant_keys(rng: &mut Xoshiro256PlusPlus) -> [ZobristHash; NUM_SQUARES] {
    let mut retval = [0u64; NUM_SQUARES];
    for item in retval.iter_mut().take(NUM_SQUARES) {
        let seed = rng.next_u64();
        *item = seed;
    }
    retval
}

#[cfg(test)]
pub mod tests {
    use super::ZobristHash;
    use super::ZobristKeys;
    use crate::position::castle_permissions::CastlePermissionType;

    #[test]
    pub fn piece_square_hashes_all_different() {
        let keys = ZobristKeys::new();
        let mut v: Vec<ZobristHash> = Vec::new();

        for pce in crate::board::piece::iterator() {
            for col in crate::board::colour::iterator() {
                for sq in crate::board::square::iterator() {
                    let hash = keys.piece_square(*pce, *col, *sq);
                    v.push(hash);
                }
            }
        }

        let mut found_cnt;
        for to_find in &v {
            found_cnt = 0;
            for hash in &v {
                if to_find == hash {
                    found_cnt += 1;
                }
            }
            assert!(found_cnt == 1);
        }
    }

    #[test]
    pub fn en_passant_hashes_all_different() {
        let keys = ZobristKeys::new();
        let mut v: Vec<ZobristHash> = Vec::new();

        for sq in crate::board::square::iterator() {
            let hash = keys.en_passant(*sq);
            v.push(hash);
        }

        let mut found_cnt;
        for to_find in &v {
            found_cnt = 0;
            for hash in &v {
                if to_find == hash {
                    found_cnt += 1;
                }
            }
            assert!(found_cnt == 1);
        }
    }

    #[test]
    pub fn castle_permissions_hashes_all_different() {
        let castle_types = [
            CastlePermissionType::WhiteKing,
            CastlePermissionType::WhiteQueen,
            CastlePermissionType::BlackKing,
            CastlePermissionType::BlackQueen,
        ];

        let keys = ZobristKeys::new();
        let mut v: Vec<ZobristHash> = Vec::new();

        for perm in castle_types.iter() {
            let hash = keys.castle_permission(*perm);
            v.push(hash);
        }

        let mut found_cnt;
        for to_find in &v {
            found_cnt = 0;
            for hash in &v {
                if to_find == hash {
                    found_cnt += 1;
                }
            }
            assert!(found_cnt == 1);
        }
    }

    #[test]
    pub fn side_hash_is_non_zero() {
        let keys = ZobristKeys::new();
        assert!(keys.side() != 0);
    }

    #[test]
    pub fn ensure_prng_is_reproduceable() {
        let keys1 = ZobristKeys::new();
        let keys2 = ZobristKeys::new();

        assert_eq!(keys1.side(), keys2.side());

        for col in crate::board::colour::iterator() {
            for pce in crate::board::piece::iterator() {
                for sq in crate::board::square::iterator() {
                    assert_eq!(
                        keys1.piece_square(*pce, *col, *sq),
                        keys2.piece_square(*pce, *col, *sq)
                    );
                }
            }
        }

        assert_eq!(
            keys1.castle_permission(CastlePermissionType::WhiteQueen),
            keys2.castle_permission(CastlePermissionType::WhiteQueen)
        );
    }
}
