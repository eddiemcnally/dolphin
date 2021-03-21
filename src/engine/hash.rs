extern crate rand;

use components::piece::Piece;
use components::square::Square;
use engine::castle_permissions::CastlePermissionType;
use engine::hash_seed::HashSeed;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PositionHash {
    hash: u64,
}

impl fmt::Display for PositionHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Default for PositionHash {
    fn default() -> Self {
        PositionHash { hash: 0 }
    }
}
impl PositionHash {
    pub fn new(val: u64) -> PositionHash {
        PositionHash { hash: val }
    }
    pub const fn update_side(&self, hash_seeds: &'static HashSeed) -> PositionHash {
        let new_hash = self.hash ^ hash_seeds.get_side_seed();
        PositionHash { hash: new_hash }
    }

    pub const fn update_piece(
        &self,
        hash_seeds: &'static HashSeed,
        piece: &Piece,
        square: Square,
    ) -> PositionHash {
        let new_hash = self.hash ^ hash_seeds.get_piece_square_seed(piece, square);
        PositionHash { hash: new_hash }
    }

    pub const fn update_en_passant(&self, hash_seeds: &'static HashSeed, square: Square) -> PositionHash {
        let new_hash = self.hash ^ hash_seeds.get_en_passant_seed(square);
        PositionHash { hash: new_hash }
    }

    pub const fn update_castle_permissions(
        &self,
        hash_seeds: &'static HashSeed,
        perm_type: CastlePermissionType,
    ) -> PositionHash {
        let new_hash = self.hash ^ hash_seeds.get_castle_permission_seed(perm_type);
        PositionHash { hash: new_hash }
    }
}

#[cfg(test)]
pub mod tests {
    use engine::hash::PositionHash;
    use engine::hash_seed::HashSeed;
    use utils;

    #[test]
    pub fn hash_flip_side_result_as_expected() {
        let mut hash: PositionHash = PositionHash::default();
        let seeds = HashSeed::new();
        let init_hash = hash;

        hash = hash.update_side(seeds);

        assert!(init_hash != hash);

        hash = hash.update_side(seeds);
        assert!(hash == init_hash);
    }

    #[test]
    pub fn flip_piece_and_square_result_as_expected() {
        let mut h: PositionHash = PositionHash::default();
        let seeds = HashSeed::new();

        for pce in utils::get_all_pieces() {
            for sq in utils::get_ordered_square_list_by_file() {
                let init_hash = h;

                h = h.update_piece(seeds, &pce, sq);
                let after_hash = h;

                assert!(init_hash != after_hash);

                h = h.update_piece(seeds, &pce, sq);
                let after_second_hash = h;
                assert!(after_hash != after_second_hash);

                // after flip, back to the same
                assert!(init_hash == after_second_hash);

                // now flip again to seed the next iteration with something different
                h = h.update_piece(seeds, &pce, sq);
            }
        }
    }

    #[test]
    pub fn flip_en_passant_result_as_expected() {
        let mut h: PositionHash = PositionHash::default();
        let seeds = HashSeed::new();

        for sq in utils::get_ordered_square_list_by_file() {
            let init_hash = h;

            h = h.update_en_passant(seeds, sq);
            let after_hash = h;

            assert!(init_hash != after_hash);

            h = h.update_en_passant(seeds, sq);
            let after_second_hash = h;
            assert!(after_hash != after_second_hash);

            // after flip, back to the same
            assert!(init_hash == after_second_hash);

            // now flip again to seed the next iteration with something different
            h = h.update_en_passant(seeds, sq);
        }
    }

    #[test]
    pub fn flip_castle_permission_as_expected() {
        let mut h: PositionHash = PositionHash::default();
        let seeds = HashSeed::new();

        for cp in utils::get_all_castle_permissions() {
            let init_hash = h;

            h = h.update_castle_permissions(seeds, cp);
            let after_hash = h;

            assert!(init_hash != after_hash);

            h = h.update_castle_permissions(seeds, cp);
            let after_second_hash = h;
            assert!(after_hash != after_second_hash);

            // after flip, back to the same
            assert!(init_hash == after_second_hash);

            // now flip again to seed the next iteration with something different
            h = h.update_castle_permissions(seeds, cp);
        }
    }
}
