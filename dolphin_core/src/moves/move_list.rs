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
}

#[cfg(test)]
pub mod tests {
    use crate::board::square::*;
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
            Move::encode_move_quiet(SQUARE_H7, SQUARE_H5),
            Move::encode_move_quiet(SQUARE_B4, SQUARE_C5),
            Move::encode_move_quiet(SQUARE_A3, SQUARE_A2),
            Move::encode_move_quiet(SQUARE_D6, SQUARE_E8),
            Move::encode_move_quiet(SQUARE_B6, SQUARE_B7),
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
            Move::encode_move_quiet(SQUARE_H7, SQUARE_H5),
            Move::encode_move_quiet(SQUARE_B4, SQUARE_C5),
            Move::encode_move_quiet(SQUARE_A3, SQUARE_A2),
            Move::encode_move_quiet(SQUARE_D6, SQUARE_E8),
            Move::encode_move_quiet(SQUARE_B6, SQUARE_B7),
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
            Move::encode_move_quiet(SQUARE_H7, SQUARE_H5),
            Move::encode_move_quiet(SQUARE_B4, SQUARE_C5),
            Move::encode_move_quiet(SQUARE_A3, SQUARE_A2),
            Move::encode_move_quiet(SQUARE_D6, SQUARE_E8),
            Move::encode_move_quiet(SQUARE_B6, SQUARE_B7),
        ];

        let mut ml = MoveList::new();
        for mv in mvs.iter() {
            ml.push(*mv);
        }
        assert_eq!(ml.len(), mvs.len());
    }
}
