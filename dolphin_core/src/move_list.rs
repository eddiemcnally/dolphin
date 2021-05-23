use crate::mov::Mov;

const MOVE_LIST_LEN: usize = 96;

pub struct MoveList {
    ml: [Mov; MOVE_LIST_LEN],
    count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            ml: [Mov::default(); MOVE_LIST_LEN],
            count: 0,
        }
    }

    pub fn push(&mut self, mov: Mov) {
        debug_assert!(
            self.count < MOVE_LIST_LEN,
            "Attempt to add past end of move list"
        );

        self.ml[self.count] = mov;
        self.count += 1;
    }

    pub fn contains(&self, mov: Mov) -> bool {
        self.ml[0..self.count].contains(&mov)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Mov> {
        self.ml[0..self.count].into_iter()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::mov::Mov;
    use crate::move_list::MoveList;
    use crate::square::Square;

    #[test]
    pub fn init_size_is_zero() {
        let mvl = MoveList::new();

        assert_eq!(mvl.len(), 0);
    }

    #[test]
    pub fn empty_list_iterator_as_expected() {
        let mut count = 0;
        let mvl = MoveList::new();

        for _ in mvl.iter() {
            count += 1;
        }
        assert_eq!(count, 0);
    }

    #[test]
    pub fn push_moves_contains_as_expected() {
        let mvs = [
            Mov::encode_move_quiet(Square::h7, Square::h5),
            Mov::encode_move_quiet(Square::b4, Square::c5),
            Mov::encode_move_quiet(Square::a3, Square::a2),
            Mov::encode_move_quiet(Square::d6, Square::e8),
            Mov::encode_move_quiet(Square::b6, Square::b7),
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
            Mov::encode_move_quiet(Square::h7, Square::h5),
            Mov::encode_move_quiet(Square::b4, Square::c5),
            Mov::encode_move_quiet(Square::a3, Square::a2),
            Mov::encode_move_quiet(Square::d6, Square::e8),
            Mov::encode_move_quiet(Square::b6, Square::b7),
        ];

        let mut ml = MoveList::new();
        for mv in mvs.iter() {
            ml.push(*mv);
        }

        let mut counter = 0;
        for mv in ml.iter() {
            counter += 1;
            assert!(mvs.contains(mv));
        }
        assert!(counter == mvs.len());
    }

    #[test]
    pub fn push_moves_len_as_expected() {
        let mvs = [
            Mov::encode_move_quiet(Square::h7, Square::h5),
            Mov::encode_move_quiet(Square::b4, Square::c5),
            Mov::encode_move_quiet(Square::a3, Square::a2),
            Mov::encode_move_quiet(Square::d6, Square::e8),
            Mov::encode_move_quiet(Square::b6, Square::b7),
        ];

        let mut ml = MoveList::new();
        for mv in mvs.iter() {
            ml.push(*mv);
        }
        assert_eq!(ml.len(), mvs.len());
    }
}
