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
        let k = self.keys.piece_keys[pce_offset][sq_offset];

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
}

fn init_piece_keys() -> [[u64; NUM_PIECES]; NUM_SQUARES] {
    let mut retval = [[0u64; NUM_PIECES]; NUM_SQUARES];

    for p in 0..NUM_PIECES {
        for c in 0..NUM_SQUARES {
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
