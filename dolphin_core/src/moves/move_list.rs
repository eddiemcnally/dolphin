use crate::moves::mov::Move;

const MOVE_LIST_LEN: usize = 96;

pub struct MoveList {
    ml: [Move; MOVE_LIST_LEN],
    count: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            ml: [Move::default(); MOVE_LIST_LEN],
            count: 0,
        }
    }

    pub fn push(&mut self, mov: Move) {
        debug_assert!(
            self.count < MOVE_LIST_LEN,
            "Attempt to add past end of move list"
        );

        self.ml[self.count] = mov;
        self.count += 1;
    }

    pub fn contains(&self, mov: Move) -> bool {
        self.ml[0..self.count].contains(&mov)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get_move_at_offset(&self, offset: usize) -> Move {
        self.ml[offset]
    }

    pub fn set_score_for_move_at(&mut self, offset: usize, score: i32) {
        let mut mv = self.get_move_at_offset(offset);
        mv.set_score(score);
    }

    pub fn get_offset_for_move(&self, mv: Move) -> Option<usize> {
        for i in 0..self.len() {
            if self.ml[i] == mv {
                return Some(i);
            }
        }
        None
    }

    pub fn iterator(&self) -> std::slice::Iter<'_, Move> {
        self.ml[0..self.count].iter()
    }

    pub fn sort_by_score(&mut self, sort_from_offset: usize) {
        if self.count == 0 {
            return;
        }

        if sort_from_offset == self.count - 1 {
            // starting at the end, nothing to sort
            return;
        }

        let mut high = sort_from_offset;

        // find entry with highest score
        for i in (sort_from_offset + 1)..self.count {
            if self.ml[i].get_score() > self.ml[high].get_score() {
                high = i;
            }
        }
        self.ml.swap(high, sort_from_offset);
    }

    pub fn print(&self) {
        for mov in self.iterator() {
            mov.print_move();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::square::Square;
    use crate::moves::mov::Move;
    use crate::moves::move_list::MoveList;

    #[test]
    pub fn init_size_is_zero() {
        let mvl = MoveList::new();

        assert_eq!(mvl.len(), 0);
    }

    #[test]
    pub fn empty_list_iterator_as_expected() {
        let mut count = 0;
        let mvl = MoveList::new();

        for _ in mvl.iterator() {
            count += 1;
        }
        assert_eq!(count, 0);
    }

    #[test]
    pub fn push_moves_contains_as_expected() {
        let mvs = [
            Move::encode_move_quiet(Square::H7, Square::H5),
            Move::encode_move_quiet(Square::B4, Square::C5),
            Move::encode_move_quiet(Square::A3, Square::A2),
            Move::encode_move_quiet(Square::D6, Square::E8),
            Move::encode_move_quiet(Square::B6, Square::B7),
        ];

        let mut ml = MoveList::new();
        for mv in mvs.iter() {
            ml.push(*mv);
        }

        for mv in mvs.iter() {
            assert!(ml.contains(*mv));
        }
    }

    #[test]
    pub fn push_moves_iterator_as_expected() {
        let mvs = [
            Move::encode_move_quiet(Square::H7, Square::H5),
            Move::encode_move_quiet(Square::B4, Square::C5),
            Move::encode_move_quiet(Square::A3, Square::A2),
            Move::encode_move_quiet(Square::D6, Square::E8),
            Move::encode_move_quiet(Square::B6, Square::B7),
        ];

        let mut ml = MoveList::new();
        for mv in mvs.iter() {
            ml.push(*mv);
        }

        let mut counter = 0;
        for mv in ml.iterator() {
            counter += 1;
            assert!(mvs.contains(mv));
        }
        assert!(counter == mvs.len());
    }

    #[test]
    pub fn push_moves_len_as_expected() {
        let mvs = [
            Move::encode_move_quiet(Square::H7, Square::H5),
            Move::encode_move_quiet(Square::B4, Square::C5),
            Move::encode_move_quiet(Square::A3, Square::A2),
            Move::encode_move_quiet(Square::D6, Square::E8),
            Move::encode_move_quiet(Square::B6, Square::B7),
        ];

        let mut ml = MoveList::new();
        for mv in mvs.iter() {
            ml.push(*mv);
        }
        assert_eq!(ml.len(), mvs.len());
    }

    #[test]
    pub fn sort_move_by_score_highest_brought_to_top_sort_from_start() {
        let mut mv1 = Move::encode_move_quiet(Square::H7, Square::H5);
        let mut mv2 = Move::encode_move_quiet(Square::B4, Square::C5);
        let mut mv3 = Move::encode_move_quiet(Square::A3, Square::A2);
        let mut mv4 = Move::encode_move_quiet(Square::D6, Square::E8);
        let mut mv5 = Move::encode_move_quiet(Square::B6, Square::B7);

        mv1.set_score(1);
        mv2.set_score(2);
        mv3.set_score(3);
        mv4.set_score(4);
        mv5.set_score(5);

        let mut ml = MoveList::new();
        ml.push(mv1);
        ml.push(mv2);
        ml.push(mv3);
        ml.push(mv4);
        ml.push(mv5);

        // check sorting before operation
        assert!(ml.get_move_at_offset(0) == mv1);
        assert!(ml.get_move_at_offset(1) == mv2);
        assert!(ml.get_move_at_offset(2) == mv3);
        assert!(ml.get_move_at_offset(3) == mv4);
        assert!(ml.get_move_at_offset(4) == mv5);

        ml.sort_by_score(0); // sort from start

        assert!(ml.get_move_at_offset(0) == mv5);
        assert!(ml.get_move_at_offset(1) == mv2);
        assert!(ml.get_move_at_offset(2) == mv3);
        assert!(ml.get_move_at_offset(3) == mv4);
        assert!(ml.get_move_at_offset(4) == mv1);
    }

    #[test]
    pub fn sort_move_by_score_highest_brought_to_top_sort_from_mid_array() {
        let mut mv1 = Move::encode_move_quiet(Square::H7, Square::H5);
        let mut mv2 = Move::encode_move_quiet(Square::B4, Square::C5);
        let mut mv3 = Move::encode_move_quiet(Square::A3, Square::A2);
        let mut mv4 = Move::encode_move_quiet(Square::D6, Square::E8);
        let mut mv5 = Move::encode_move_quiet(Square::B6, Square::B7);

        mv1.set_score(1);
        mv2.set_score(2);
        mv3.set_score(3);
        mv4.set_score(4);
        mv5.set_score(5);

        let mut ml = MoveList::new();
        ml.push(mv1);
        ml.push(mv2);
        ml.push(mv3);
        ml.push(mv4);
        ml.push(mv5);

        // check sorting before operation
        assert!(ml.get_move_at_offset(0) == mv1);
        assert!(ml.get_move_at_offset(1) == mv2);
        assert!(ml.get_move_at_offset(2) == mv3);
        assert!(ml.get_move_at_offset(3) == mv4);
        assert!(ml.get_move_at_offset(4) == mv5);

        ml.sort_by_score(2);

        assert!(ml.get_move_at_offset(0) == mv1);
        assert!(ml.get_move_at_offset(1) == mv2);
        assert!(ml.get_move_at_offset(2) == mv5);
        assert!(ml.get_move_at_offset(3) == mv4);
        assert!(ml.get_move_at_offset(4) == mv3);
    }
    #[test]
    pub fn sort_move_by_score_highest_brought_to_top_sort_from_last_entry() {
        let mut mv1 = Move::encode_move_quiet(Square::H7, Square::H5);
        let mut mv2 = Move::encode_move_quiet(Square::B4, Square::C5);
        let mut mv3 = Move::encode_move_quiet(Square::A3, Square::A2);
        let mut mv4 = Move::encode_move_quiet(Square::D6, Square::E8);
        let mut mv5 = Move::encode_move_quiet(Square::B6, Square::B7);

        mv1.set_score(1);
        mv2.set_score(2);
        mv3.set_score(3);
        mv4.set_score(4);
        mv5.set_score(5);

        let mut ml = MoveList::new();
        ml.push(mv1);
        ml.push(mv2);
        ml.push(mv3);
        ml.push(mv4);
        ml.push(mv5);

        // check sorting before operation
        assert!(ml.get_move_at_offset(0) == mv1);
        assert!(ml.get_move_at_offset(1) == mv2);
        assert!(ml.get_move_at_offset(2) == mv3);
        assert!(ml.get_move_at_offset(3) == mv4);
        assert!(ml.get_move_at_offset(4) == mv5);

        ml.sort_by_score(4); // sort from last entry

        // no sort performed
        assert!(ml.get_move_at_offset(0) == mv1);
        assert!(ml.get_move_at_offset(1) == mv2);
        assert!(ml.get_move_at_offset(2) == mv3);
        assert!(ml.get_move_at_offset(3) == mv4);
        assert!(ml.get_move_at_offset(4) == mv5);
    }
}
