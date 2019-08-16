extern crate rand;

use board::board::NUM_SQUARES;
use board::piece::Piece;
use board::piece::NUM_PIECES;
use board::square::Square;
use position::castle_permissions::CastlePermission;
use position::castle_permissions::CastlePermissionType;
use position::castle_permissions::NUM_CASTLE_PERMS;

pub struct PositionHash {
    hash: u64,
    keys: Keys,
}

struct Keys {
    piece_keys: [[u64; NUM_PIECES]; NUM_SQUARES],
    side_key: u64,
    castle_keys: [u64; NUM_CASTLE_PERMS],
    en_passant_sq_keys: [u64; NUM_SQUARES],
}

impl PositionHash {
    pub fn new() -> PositionHash {
        let pkeys = init_piece_keys();
        let ckeys = init_castle_keys();
        let ekeys = init_en_passant_keys();
        let side_key = rand::random::<u64>();

        let key_struct = Keys {
            piece_keys: pkeys,
            side_key: side_key,
            castle_keys: ckeys,
            en_passant_sq_keys: ekeys,
        };

        PositionHash {
            hash: 0,
            keys: key_struct,
        }
    }

    pub fn update_side(&mut self) {
        self.hash = self.hash ^ self.keys.side_key;
    }

    pub fn update_piece(&mut self, piece: Piece, square: Square) {
        let pce_offset = piece.offset();
        let sq_offset = square.to_offset();
        let k = self.keys.piece_keys[sq_offset][pce_offset];

        self.hash = self.hash ^ k;
    }

    pub fn update_en_passant(&mut self, square: Square) {
        let sq_offset = square.to_offset();
        let k = self.keys.en_passant_sq_keys[sq_offset];

        self.hash = self.hash ^ k;
    }

    pub fn update_castle_permissions(&mut self, perm_type: CastlePermissionType) {
        let perm_offset = CastlePermission::offset(perm_type);
        let k = self.keys.castle_keys[perm_offset];
        self.hash = self.hash ^ k;
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }
}

fn init_piece_keys() -> [[u64; NUM_PIECES]; NUM_SQUARES] {
    let mut retval = [[0u64; NUM_PIECES]; NUM_SQUARES];
    for p in 0..NUM_SQUARES {
        for c in 0..NUM_PIECES {
            let seed = rand::random::<u64>();
            retval[p][c] = seed;
        }
    }
    return retval;
}

fn init_castle_keys() -> [u64; NUM_CASTLE_PERMS] {
    let mut retval = [0u64; NUM_CASTLE_PERMS];

    for p in 0..NUM_CASTLE_PERMS {
        let seed = rand::random::<u64>();
        retval[p] = seed;
    }

    return retval;
}

fn init_en_passant_keys() -> [u64; NUM_SQUARES] {
    let mut retval = [0u64; NUM_SQUARES];

    for p in 0..NUM_SQUARES {
        let seed = rand::random::<u64>();
        retval[p] = seed;
    }

    return retval;
}

#[cfg(test)]
pub mod tests {
    use position::hash::PositionHash;
    use utils;

    #[test]
    pub fn hash_init_as_zero() {
        let h = PositionHash::new();

        assert_eq!(h.get_hash(), 0);
    }

    #[test]
    pub fn hash_flip_side_result_as_expected() {
        let mut h = PositionHash::new();

        let init_hash = h.get_hash();

        h.update_side();
        let flip_1 = h.get_hash();

        assert!(init_hash != flip_1);

        h.update_side();
        let flip_2 = h.get_hash();

        assert!(flip_2 != flip_1);

        assert!(flip_2 == init_hash);
    }

    #[test]
    pub fn flip_piece_and_square_result_as_expected() {
        let mut h = PositionHash::new();

        for pce in utils::get_all_pieces() {
            for sq in utils::get_ordered_square_list_by_file() {
                let init_hash = h.get_hash();

                h.update_piece(pce, sq);
                let after_hash = h.get_hash();

                assert!(init_hash != after_hash);

                h.update_piece(pce, sq);
                let after_second_hash = h.get_hash();
                assert!(after_hash != after_second_hash);

                // after flip, back to the same
                assert!(init_hash == after_second_hash);

                // now flip again to seed the next iteration with something different
                h.update_piece(pce, sq);
            }
        }
    }

    #[test]
    pub fn flip_en_passant_result_as_expected() {
        let mut h = PositionHash::new();

        for sq in utils::get_ordered_square_list_by_file() {
            let init_hash = h.get_hash();

            h.update_en_passant(sq);
            let after_hash = h.get_hash();

            assert!(init_hash != after_hash);

            h.update_en_passant(sq);
            let after_second_hash = h.get_hash();
            assert!(after_hash != after_second_hash);

            // after flip, back to the same
            assert!(init_hash == after_second_hash);

            // now flip again to seed the next iteration with something different
            h.update_en_passant(sq);
        }
    }

    #[test]
    pub fn flip_castle_permission_as_expected() {
        let mut h = PositionHash::new();

        for cp in utils::get_all_castle_permissions() {
            let init_hash = h.get_hash();

            h.update_castle_permissions(cp);
            let after_hash = h.get_hash();

            assert!(init_hash != after_hash);

            h.update_castle_permissions(cp);
            let after_second_hash = h.get_hash();
            assert!(after_hash != after_second_hash);

            // after flip, back to the same
            assert!(init_hash == after_second_hash);

            // now flip again to seed the next iteration with something different
            h.update_castle_permissions(cp);
        }
    }

}
