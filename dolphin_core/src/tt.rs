#[warn(unused_attributes)]
use crate::zobrist_keys::ZobristHash;
use std::boxed::Box;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum TransType {
    Exact,
    Upper,
    Lower,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct TransEntry {
    trans_type: TransType,
    score: i32,
    depth: u8,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Stats {
    enabled: bool,
    num_collisions: u32,
    num_used: u32,
    num_trans_type_exact: u32,
    num_trans_type_upper: u32,
    num_trans_type_lower: u32,
}
impl Default for Stats {
    fn default() -> Self {
        Stats {
            num_collisions: 0,
            num_used: 0,
            enabled: false,
            num_trans_type_exact: 0,
            num_trans_type_upper: 0,
            num_trans_type_lower: 0,
        }
    }
}
impl Stats {
    fn new(enable_stats: bool) -> Self {
        let mut stats = Stats::default();
        stats.enabled = enable_stats;

        return stats;
    }
}

impl Default for TransEntry {
    fn default() -> Self {
        TransEntry {
            trans_type: TransType::Exact,
            score: 0,
            depth: 0,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Entry {
    trans_entry: TransEntry,
    is_empty: bool,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            trans_entry: TransEntry::default(),
            is_empty: true,
        }
    }
}

pub struct TransTable {
    entries: Box<[Entry]>,
    capacity: usize,
    stats: Stats,
}

impl Default for TransTable {
    fn default() -> Self {
        Self {
            entries: Box::new([Entry::default(); 1]),
            capacity: 1,
            stats: Stats::default(),
        }
    }
}

impl TransTable {
    fn new(capacity: usize, enable_stats: bool) -> Self {
        let array = vec![Entry::default(); capacity].into_boxed_slice();

        TransTable {
            entries: array,
            capacity,
            stats: Stats::new(enable_stats),
        }
    }

    pub fn add(&mut self, trans_entry: &TransEntry, hash: ZobristHash) {
        let offset = TransTable::get_offset(hash, self.capacity);

        let entry = Entry {
            trans_entry: *trans_entry,
            is_empty: false,
        };

        if self.stats.enabled {
            if self.entries[offset].is_empty == false {
                self.stats.num_collisions += 1;
            }
            self.stats.num_used += 1;

            match entry.trans_entry.trans_type {
                TransType::Exact => self.stats.num_trans_type_exact += 1,
                TransType::Upper => self.stats.num_trans_type_upper += 1,
                TransType::Lower => self.stats.num_trans_type_lower += 1,
            }
        }

        self.entries[offset] = entry;
    }

    pub fn is_present(&self, hash: ZobristHash) -> bool {
        let offset = TransTable::get_offset(hash, self.capacity);
        self.entries[offset].is_empty == false
    }

    pub fn get(&self, hash: ZobristHash) -> Option<TransEntry> {
        let offset = TransTable::get_offset(hash, self.capacity);

        match self.entries[offset].is_empty {
            true => None,
            false => Some(self.entries[offset].trans_entry),
        }
    }

    pub fn display_stats(&self) {
        let percent_used = self.stats.num_used.checked_div(self.capacity as u32);
        println!(
            "TT Stats: \
            Capacity : {:?}, \
            Num Used : {:?}, \
            % used : {:?}, \
            Num Collisions : {:?}, \
            Num EXACT Types : {:?}, \
            Num UPPER Types : {:?}, \
            Num LOWER Types : {:?}",
            self.capacity,
            self.stats.num_used,
            percent_used,
            self.stats.num_collisions,
            self.stats.num_trans_type_exact,
            self.stats.num_trans_type_upper,
            self.stats.num_trans_type_lower
        );
    }

    fn get_offset(hash: ZobristHash, capacity: usize) -> usize {
        return (hash % capacity as u64) as usize;
    }
}

#[cfg(test)]
pub mod tests {
    use super::TransEntry;
    use super::TransTable;
    use super::TransType;

    #[test]
    pub fn add_and_get_multiple_verify_contents_as_expected() {
        const NUM_TO_TEST: usize = 100_000_000;
        const DEPTH: u8 = 5;
        const TT_ENTRY_TYPE: TransType = TransType::Upper;

        let mut entry = TransEntry::default();
        let mut tt = TransTable::new(NUM_TO_TEST, false);

        // add to TT
        for i in 0..NUM_TO_TEST {
            entry.score = i as i32;
            entry.depth = DEPTH;
            entry.trans_type = TT_ENTRY_TYPE;

            tt.add(&entry, i as u64);
        }

        // retrieve and verify
        for i in 0..NUM_TO_TEST {
            let tte = tt.get(i as u64);
            assert!(tte.is_some());

            assert!(tte.unwrap().score == i as i32);
            assert!(tte.unwrap().depth == DEPTH);
            assert!(tte.unwrap().trans_type == TT_ENTRY_TYPE);
        }
    }

    #[test]
    pub fn add_multiple_verify_is_present_as_expected() {
        const NUM_TO_TEST: usize = 100_000_000;
        const DEPTH: u8 = 5;
        const TT_ENTRY_TYPE: TransType = TransType::Upper;

        let mut entry = TransEntry::default();
        let mut tt = TransTable::new(NUM_TO_TEST, false);

        // add to TT
        for i in 0..NUM_TO_TEST {
            entry.score = i as i32;
            entry.depth = DEPTH;
            entry.trans_type = TT_ENTRY_TYPE;

            tt.add(&entry, i as u64);
        }

        // check is_present
        for i in 0..NUM_TO_TEST {
            let tte = tt.is_present(i as u64);
            assert!(tte);
        }
    }
}
