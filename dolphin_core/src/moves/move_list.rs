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
        self.len() == 0
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Move> {
        self.ml[0..self.count].iter()
    }

    pub fn print(&self) {
        for mov in self.iter() {
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

        for _ in mvl.iter() {
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
        for mv in ml.iter() {
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
}
