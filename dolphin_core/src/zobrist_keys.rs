extern crate rand;

use crate::castle_permissions;
use crate::castle_permissions::{CastlePermissionType, NUM_CASTLE_PERMS};
use crate::piece::{Piece, NUM_PIECES};
use crate::square::{Square, NUM_SQUARES};

pub type ZobristHash = u64;

pub struct ZobristKeys {
    piece_keys: [[ZobristHash; NUM_PIECES]; NUM_SQUARES],
    side_key: ZobristHash,
    castle_keys: [ZobristHash; NUM_CASTLE_PERMS],
    en_passant_sq_keys: [ZobristHash; NUM_SQUARES],
}

impl ZobristKeys {
    pub fn new() -> ZobristKeys {
        let piece_keys = init_piece_keys();
        let side_key = rand::random::<ZobristHash>();
        let castle_keys = init_castle_keys();
        let en_passant_sq_keys = init_en_passant_keys();

        ZobristKeys {
            piece_keys,
            side_key,
            castle_keys,
            en_passant_sq_keys,
        }
    }

    pub const fn side(&self) -> ZobristHash {
        self.side_key
    }

    pub const fn piece_square(&self, piece: Piece, square: Square) -> ZobristHash {
        let pce_offset = piece.to_offset();
        let sq_offset = square.to_offset();
        self.piece_keys[sq_offset][pce_offset]
    }

    pub const fn en_passant(&self, square: Square) -> ZobristHash {
        let sq_offset = square.to_offset();
        self.en_passant_sq_keys[sq_offset]
    }

    pub const fn castle_permission(&self, perm_type: CastlePermissionType) -> ZobristHash {
        let perm_offset = castle_permissions::to_offset(perm_type);
        self.castle_keys[perm_offset]
    }
}

fn init_piece_keys() -> [[ZobristHash; NUM_PIECES]; NUM_SQUARES] {
    let mut retval = [[0u64; NUM_PIECES]; NUM_SQUARES];
    for p in 0..NUM_SQUARES {
        for c in 0..NUM_PIECES {
            let seed = rand::random::<u64>();
            retval[p][c] = seed;
        }
    }
    retval
}

fn init_castle_keys() -> [ZobristHash; NUM_CASTLE_PERMS] {
    let mut retval = [0u64; NUM_CASTLE_PERMS];

    for p in 0..NUM_CASTLE_PERMS {
        let seed = rand::random::<u64>();
        retval[p] = seed;
    }

    retval
}

fn init_en_passant_keys() -> [ZobristHash; NUM_SQUARES] {
    let mut retval = [0u64; NUM_SQUARES];

    for p in 0..NUM_SQUARES {
        let seed = rand::random::<u64>();
        retval[p] = seed;
    }

    retval
}
