use super::castle_permissions::CastlePermission;
use crate::board::colour::Colour;
use crate::board::piece::Piece;
use crate::board::square::Square;
use rand::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

pub type ZobristHash = u64;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ZobristKeys {
    piece_keys: [[[ZobristHash; Piece::NUM_PIECE_TYPES]; Square::NUM_SQUARES]; Colour::NUM_COLOURS],
    side_key: ZobristHash,
    castle_keys: [ZobristHash; CastlePermission::NUM_CASTLE_PERMS],
    en_passant_sq_keys: [ZobristHash; Square::NUM_SQUARES],
}

impl Default for ZobristKeys {
    fn default() -> Self {
        ZobristKeys {
            piece_keys: [[[0; Piece::NUM_PIECE_TYPES]; Square::NUM_SQUARES]; Colour::NUM_COLOURS],
            side_key: 0,
            castle_keys: [0; CastlePermission::NUM_CASTLE_PERMS],
            en_passant_sq_keys: [0; Square::NUM_SQUARES],
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
        let pce_offset = piece.as_index();
        let sq_offset = square.as_index();
        let col_offset = colour.as_index();
        self.piece_keys[col_offset][sq_offset][pce_offset]
    }

    pub fn en_passant(&self, square: Square) -> ZobristHash {
        let sq_offset = square.as_index();
        self.en_passant_sq_keys[sq_offset]
    }

    pub const fn castle_permissions_white_king(&self) -> ZobristHash {
        self.castle_keys[CastlePermission::white_king_offset()]
    }
    pub const fn castle_permissions_white_queen(&self) -> ZobristHash {
        self.castle_keys[CastlePermission::white_queen_offset()]
    }
    pub const fn castle_permissions_black_king(&self) -> ZobristHash {
        self.castle_keys[CastlePermission::black_king_offset()]
    }
    pub const fn castle_permissions_black_queen(&self) -> ZobristHash {
        self.castle_keys[CastlePermission::black_queen_offset()]
    }
}

fn init_piece_keys(
    rng: &mut Xoshiro256PlusPlus,
) -> [[[ZobristHash; Piece::NUM_PIECE_TYPES]; Square::NUM_SQUARES]; Colour::NUM_COLOURS] {
    let mut retval = [[[0u64; Piece::NUM_PIECE_TYPES]; Square::NUM_SQUARES]; Colour::NUM_COLOURS];
    for element in retval.iter_mut().flat_map(|r| r.iter_mut()) {
        for i in element {
            *i = rng.next_u64();
        }
    }
    retval
}
fn init_castle_keys(
    rng: &mut Xoshiro256PlusPlus,
) -> [ZobristHash; CastlePermission::NUM_CASTLE_PERMS] {
    let mut retval = [0u64; CastlePermission::NUM_CASTLE_PERMS];
    for item in retval.iter_mut().take(CastlePermission::NUM_CASTLE_PERMS) {
        let seed = rng.next_u64();
        *item = seed;
    }
    retval
}
fn init_en_passant_keys(rng: &mut Xoshiro256PlusPlus) -> [ZobristHash; Square::NUM_SQUARES] {
    let mut retval = [0u64; Square::NUM_SQUARES];
    for item in retval.iter_mut().take(Square::NUM_SQUARES) {
        let seed = rng.next_u64();
        *item = seed;
    }
    retval
}

#[cfg(test)]
pub mod tests {
    use super::ZobristHash;
    use super::ZobristKeys;
    use crate::board::colour::Colour;
    use crate::position::zobrist_keys::Piece;
    use crate::position::zobrist_keys::Square;

    #[test]
    pub fn piece_square_hashes_all_different() {
        let keys = ZobristKeys::new();
        let mut v: Vec<ZobristHash> = Vec::new();

        let pieces = [
            Piece::Pawn,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];
        let colours = [Colour::White, Colour::Black];

        for pce in pieces.iter() {
            for col in colours.iter() {
                for sq in Square::iterator() {
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

        for sq in Square::iterator() {
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
    pub fn side_hash_is_non_zero() {
        let keys = ZobristKeys::new();
        assert!(keys.side() != 0);
    }
}
