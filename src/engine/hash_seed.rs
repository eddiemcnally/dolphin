extern crate rand;

use components::board::NUM_SQUARES;
use components::piece::Piece;
use components::piece::NUM_PIECES;
use components::square::Square;
use engine::castle_permissions;
use engine::castle_permissions::CastlePermissionType;
use engine::castle_permissions::NUM_CASTLE_PERMS;

pub struct HashSeed {
    piece_keys: [[u64; NUM_PIECES]; NUM_SQUARES],
    side_key: u64,
    castle_keys: [u64; NUM_CASTLE_PERMS],
    en_passant_sq_keys: [u64; NUM_SQUARES],
}

lazy_static! {
    // the order of these is important....bishop uses the diag and anti-diag masks, the queen uses both rook and bishop masks
    static ref SEEDS: HashSeed = HashSeed {
        piece_keys: init_piece_keys(),
        side_key: rand::random::<u64>(),
        castle_keys: init_castle_keys(),
        en_passant_sq_keys: init_en_passant_keys(),
    };
}

impl HashSeed {
    pub fn new() -> &'static HashSeed {
        return &SEEDS;
    }

    pub const fn get_side_seed(&'static self) -> u64 {
        self.side_key
    }

    pub const fn get_piece_square_seed(&'static self, piece: &Piece, square: Square) -> u64 {
        let pce_offset = piece.to_offset();
        let sq_offset = square.to_offset();
        self.piece_keys[sq_offset][pce_offset]
    }

    pub const fn get_en_passant_seed(&'static self, square: Square) -> u64 {
        let sq_offset = square.to_offset();
        self.en_passant_sq_keys[sq_offset]
    }

    pub const fn get_castle_permission_seed(&'static self, perm_type: CastlePermissionType) -> u64 {
        let perm_offset = castle_permissions::to_offset(perm_type);
        self.castle_keys[perm_offset]
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
