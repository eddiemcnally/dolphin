use crate::moves::mov::Move;
use crate::moves::mov::Score;
use crate::position::zobrist_keys::ZobristHash;
use std::boxed::Box;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TransType {
    Exact,
    Alpha,
    Beta,
}

impl fmt::Display for TransType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
impl Default for TransType {
    fn default() -> Self {
        TransType::Exact
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct TransEntry {
    trans_type: TransType,
    score: Score,
    depth: u8,
    mv: Move,
    in_use: bool,
}
impl Default for TransEntry {
    fn default() -> Self {
        TransEntry {
            trans_type: TransType::Exact,
            score: 0,
            depth: 0,
            mv: Move::default(),
            in_use: false,
        }
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Stats {
    enabled: bool,
    num_collisions: u32,
    num_misses: u32,
    num_used: u32,
    num_trans_type_exact: u32,
    num_trans_type_upper: u32,
    num_trans_type_lower: u32,
}

pub struct TransTable {
    entries: Box<[TransEntry]>,
    capacity: usize,
}

impl Default for TransTable {
    fn default() -> Self {
        Self {
            entries: Box::new([TransEntry::default(); 1]),
            capacity: 1,
        }
    }
}

impl TransTable {
    pub fn new(capacity: usize) -> Self {
        let array = vec![TransEntry::default(); capacity].into_boxed_slice();

        TransTable {
            entries: array,
            capacity,
        }
    }

    pub fn add(
        &mut self,
        tt_type: TransType,
        depth: u8,
        score: Score,
        hash: ZobristHash,
        mv: Move,
    ) {
        let offset = self.convert_hash_to_offset(hash, self.capacity);

        let tte = TransEntry {
            trans_type: tt_type,
            depth,
            score,
            mv,
            in_use: true,
        };

        self.entries[offset] = tte;
    }

    pub fn contains_position_hash(&self, hash: ZobristHash) -> bool {
        let offset = self.convert_hash_to_offset(hash, self.capacity);

        if !self.entries[offset].in_use {
            return true;
        }
        false
    }

    pub fn get_move_for_position_hash(&self, hash: ZobristHash) -> Option<Move> {
        let offset = self.convert_hash_to_offset(hash, self.capacity);

        if self.entries[offset].in_use {
            return Some(self.entries[offset].mv);
        }
        None
    }

    pub fn probe(
        &self,
        hash: ZobristHash,
        depth: u8,
        alpha: Score,
        beta: Score,
    ) -> Option<(TransType, Score)> {
        let offset = self.convert_hash_to_offset(hash, self.capacity);

        let entry = self.entries[offset];
        if !entry.in_use {
            return None;
        }

        if entry.depth >= depth {
            if entry.trans_type == TransType::Exact {
                return Some((entry.trans_type, entry.score));
            }

            if entry.trans_type == TransType::Alpha && entry.score <= alpha {
                return Some((entry.trans_type, alpha));
            }

            if entry.trans_type == TransType::Beta && entry.score >= beta {
                return Some((entry.trans_type, beta));
            }
        }

        None
    }

    pub fn get(&mut self, hash: ZobristHash) -> Option<(TransType, u8, Score, Move)> {
        let offset = self.convert_hash_to_offset(hash, self.capacity);
        if self.entries[offset].in_use {
            let tte = self.entries[offset];
            let t = (tte.trans_type, tte.depth, tte.score, tte.mv);
            return Some(t);
        }
        None
    }

    pub fn get_num_used(&self) -> u32 {
        self.entries.iter().filter(|n| n.in_use).count() as u32
    }
    pub fn get_num_trans_type_exact(&self) -> u32 {
        self.count_tt_types(TransType::Exact)
    }
    pub fn get_num_trans_type_alpha(&self) -> u32 {
        self.count_tt_types(TransType::Alpha)
    }
    pub fn get_num_trans_type_beta(&self) -> u32 {
        self.count_tt_types(TransType::Beta)
    }

    fn count_tt_types(&self, tt_type: TransType) -> u32 {
        self.entries
            .iter()
            .filter(|n| n.trans_type == tt_type)
            .count() as u32
    }

    #[inline]
    fn convert_hash_to_offset(&self, hash: ZobristHash, capacity: usize) -> usize {
        (hash % capacity as u64) as usize
    }
}

#[cfg(test)]
pub mod tests {
    use super::TransTable;
    use super::TransType;
    use crate::board::square::Square;
    use crate::moves::mov::Move;
    use crate::position::zobrist_keys::ZobristHash;
    use crate::search_engine::tt::Score;

    #[test]
    pub fn add_and_get_multiple_no_collisions_verify_contents_as_expected() {
        const NUM_TO_TEST: usize = 30000;
        const DEPTH: u8 = 5;
        const TT_ENTRY_TYPE: TransType = TransType::Alpha;

        let target_move = Move::encode_move_quiet(Square::A1, Square::A2);

        let mut tt = TransTable::new(NUM_TO_TEST);
        // add to TT
        for i in 0..NUM_TO_TEST {
            let score = i as Score;
            let depth = DEPTH;
            let trans_type = TT_ENTRY_TYPE;

            tt.add(trans_type, depth, score, i as ZobristHash, target_move);
        }
        assert!(tt.get_num_used() == NUM_TO_TEST as u32);

        // retrieve and verify
        for i in 0..NUM_TO_TEST {
            let tte: Option<(TransType, u8, Score, Move)> = tt.get(i as ZobristHash);

            assert!(tte.is_some());
            let trans_type = tte.unwrap().0;
            let depth = tte.unwrap().1;
            let score = tte.unwrap().2;
            let mv = tte.unwrap().3;

            assert!(score == i as Score);
            assert!(depth == DEPTH);
            assert!(trans_type == TT_ENTRY_TYPE);
            assert!(mv == target_move);
        }
    }
}
