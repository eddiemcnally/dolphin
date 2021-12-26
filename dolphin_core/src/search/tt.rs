use crate::position::zobrist_keys::ZobristHash;
use std::boxed::Box;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TransType {
    Exact,
    Upper,
    Lower,
}

impl fmt::Display for TransType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct TransEntry {
    trans_type: TransType,
    score: i32,
    depth: u8,
    in_use: bool,
}
impl Default for TransEntry {
    fn default() -> Self {
        TransEntry {
            trans_type: TransType::Exact,
            score: 0,
            depth: 0,
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

    pub fn add(&mut self, tt_type: TransType, depth: u8, score: i32, hash: ZobristHash) {
        let offset = self.convert_hash_to_offset(hash, self.capacity);

        let tte = TransEntry {
            trans_type: tt_type,
            depth,
            score,
            in_use: true,
        };

        self.entries[offset] = tte;
    }

    pub fn get(&mut self, hash: ZobristHash) -> Option<(TransType, u8, i32)> {
        let offset = self.convert_hash_to_offset(hash, self.capacity);
        if self.entries[offset].in_use {
            let tte = self.entries[offset];
            let t = (tte.trans_type, tte.depth, tte.score);
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
    pub fn get_num_trans_type_upper(&self) -> u32 {
        self.count_tt_types(TransType::Upper)
    }
    pub fn get_num_trans_type_lower(&self) -> u32 {
        self.count_tt_types(TransType::Lower)
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
    use crate::position::zobrist_keys::ZobristHash;

    #[test]
    pub fn add_and_get_multiple_no_collisions_verify_contents_as_expected() {
        const NUM_TO_TEST: usize = 1_000_000;
        const DEPTH: u8 = 5;
        const TT_ENTRY_TYPE: TransType = TransType::Upper;

        let mut tt = TransTable::new(NUM_TO_TEST);
        // add to TT
        for i in 0..NUM_TO_TEST {
            let score = i as i32;
            let depth = DEPTH;
            let trans_type = TT_ENTRY_TYPE;

            tt.add(trans_type, depth, score, i as ZobristHash);
        }
        assert!(tt.get_num_used() == NUM_TO_TEST as u32);

        // retrieve and verify
        for i in 0..NUM_TO_TEST {
            let tte: Option<(TransType, u8, i32)> = tt.get(i as ZobristHash);

            assert!(tte.is_some() == true);
            let trans_type = tte.unwrap().0;
            let depth = tte.unwrap().1;
            let score = tte.unwrap().2;

            assert!(score == i as i32);
            assert!(depth == DEPTH);
            assert!(trans_type == TT_ENTRY_TYPE);
        }
    }

    #[test]
    pub fn add_and_get_multiple_with_collisions_verify_contents_as_expected() {
        const NUM_TO_TEST: usize = 1_000_000;
        const TT_SIZE: usize = 100_000;
        const EXPECTED_NUM_COLLISIONS: usize = 900_000;
        const DEPTH: u8 = 5;
        const TT_ENTRY_TYPE: TransType = TransType::Upper;

        let mut tt = TransTable::new(TT_SIZE);
        // add to TT
        for i in 0..NUM_TO_TEST {
            let score = i as i32;
            let depth = DEPTH;
            let trans_type = TT_ENTRY_TYPE;

            tt.add(trans_type, depth, score, i as ZobristHash);
        }
        assert!(tt.get_num_used() == TT_SIZE as u32);

        // elements upo to EXPECTED_NUM_COLLISIONS are overwritten
        for i in EXPECTED_NUM_COLLISIONS..NUM_TO_TEST {
            let tte: Option<(TransType, u8, i32)> = tt.get(i as ZobristHash);

            assert!(tte.is_some() == true);
            let trans_type = tte.unwrap().0;
            let depth = tte.unwrap().1;
            let score = tte.unwrap().2;

            assert!(score == i as i32);
            assert!(depth == DEPTH);
            assert!(trans_type == TT_ENTRY_TYPE);
        }
    }
}
