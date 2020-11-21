extern crate rand;

use components::board::NUM_SQUARES;
use components::piece::Piece;
use components::piece::NUM_PIECES;
use components::square::Square;
use core::core_traits::ArrayAccessor;
use engine::castle_permissions;
use engine::castle_permissions::CastlePermissionType;
use engine::castle_permissions::NUM_CASTLE_PERMS;

pub type PositionHash = u64;

struct HashKeys {
    piece_keys: [[u64; NUM_PIECES]; NUM_SQUARES],
    side_key: u64,
    castle_keys: [u64; NUM_CASTLE_PERMS],
    en_passant_sq_keys: [u64; NUM_SQUARES],
}

lazy_static! {
    static ref KEYS: HashKeys = HashKeys {
        piece_keys: init_piece_keys(),
        side_key: rand::random::<u64>(),
        castle_keys: init_castle_keys(),
        en_passant_sq_keys: init_en_passant_keys(),
    };
}

pub fn update_side(pos_hash: PositionHash) -> PositionHash {
    return pos_hash ^ KEYS.side_key;
}

pub fn update_piece(pos_hash: PositionHash, piece: &Piece, square: Square) -> PositionHash {
    let pce_offset = piece.to_offset();
    let sq_offset = square.to_offset();
    let k = KEYS.piece_keys[sq_offset][pce_offset];

    return pos_hash ^ k;
}

pub fn update_en_passant(pos_hash: PositionHash, square: Square) -> PositionHash {
    let sq_offset = square.to_offset();
    let k = KEYS.en_passant_sq_keys[sq_offset];

    return pos_hash ^ k;
}

pub fn update_castle_permissions(
    pos_hash: PositionHash,
    perm_type: CastlePermissionType,
) -> PositionHash {
    let perm_offset = castle_permissions::to_offset(perm_type);
    let k = KEYS.castle_keys[perm_offset];
    return pos_hash ^ k;
}

fn init_piece_keys() -> [[u64; NUM_PIECES]; NUM_SQUARES] {
    let mut retval = [[0u64; NUM_PIECES]; NUM_SQUARES];
    for p in 0..NUM_SQUARES {
        for c in 0..NUM_PIECES {
            let seed = rand::random::<u64>();
            retval[p][c] = seed;
        }
    }
    retval
}

fn init_castle_keys() -> [u64; NUM_CASTLE_PERMS] {
    let mut retval = [0u64; NUM_CASTLE_PERMS];

    for p in 0..NUM_CASTLE_PERMS {
        let seed = rand::random::<u64>();
        retval[p] = seed;
    }

    retval
}

fn init_en_passant_keys() -> [u64; NUM_SQUARES] {
    let mut retval = [0u64; NUM_SQUARES];

    for p in 0..NUM_SQUARES {
        let seed = rand::random::<u64>();
        retval[p] = seed;
    }

    retval
}

#[cfg(test)]
pub mod tests {
    use engine::hash::PositionHash;
    use utils;

    #[test]
    pub fn hash_flip_side_result_as_expected() {
        let mut h: PositionHash = 0;

        let init_hash = h;

        h = super::update_side(h);

        assert!(init_hash != h);

        h = super::update_side(h);
        assert!(h == init_hash);
    }

    #[test]
    pub fn flip_piece_and_square_result_as_expected() {
        let mut h: PositionHash = 0;

        for pce in utils::get_all_pieces() {
            for sq in utils::get_ordered_square_list_by_file() {
                let init_hash = h;

                h = super::update_piece(h, &pce, sq);
                let after_hash = h;

                assert!(init_hash != after_hash);

                h = super::update_piece(h, &pce, sq);
                let after_second_hash = h;
                assert!(after_hash != after_second_hash);

                // after flip, back to the same
                assert!(init_hash == after_second_hash);

                // now flip again to seed the next iteration with something different
                h = super::update_piece(h, &pce, sq);
            }
        }
    }

    #[test]
    pub fn flip_en_passant_result_as_expected() {
        let mut h: PositionHash = 0;

        for sq in utils::get_ordered_square_list_by_file() {
            let init_hash = h;

            h = super::update_en_passant(h, sq);
            let after_hash = h;

            assert!(init_hash != after_hash);

            h = super::update_en_passant(h, sq);
            let after_second_hash = h;
            assert!(after_hash != after_second_hash);

            // after flip, back to the same
            assert!(init_hash == after_second_hash);

            // now flip again to seed the next iteration with something different
            h = super::update_en_passant(h, sq);
        }
    }

    #[test]
    pub fn flip_castle_permission_as_expected() {
        let mut h: PositionHash = 0;

        for cp in utils::get_all_castle_permissions() {
            let init_hash = h;

            h = super::update_castle_permissions(h, cp);
            let after_hash = h;

            assert!(init_hash != after_hash);

            h = super::update_castle_permissions(h, cp);
            let after_second_hash = h;
            assert!(after_hash != after_second_hash);

            // after flip, back to the same
            assert!(init_hash == after_second_hash);

            // now flip again to seed the next iteration with something different
            h = super::update_castle_permissions(h, cp);
        }
    }
}
