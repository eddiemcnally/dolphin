use moves::mov::Mov;
use std::vec::Vec;

const MAX_MOVE_BUF_SZ: usize = 256;

#[derive(Clone, Copy)]
pub struct MoveList {
    moves: [Mov; MAX_MOVE_BUF_SZ],
    count: usize,
    iter_count: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            moves: [Mov::default(); MAX_MOVE_BUF_SZ],
            count: 0,
            iter_count: 0,
        }
    }
}

impl Iterator for MoveList {
    type Item = Mov;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_count < self.count {
            let retval = self.moves[self.iter_count];
            self.iter_count = self.iter_count + 1;
            Some(retval)
        } else {
            None
        }
    }
}

impl MoveList {
    pub fn push(&mut self, mov: Mov) {
        if self.count >= MAX_MOVE_BUF_SZ - 1 {
            panic!("Move List is full.");
        }

        self.moves[self.count] = mov;
        self.count += 1;
    }

    pub fn contains(&self, mov: Mov) -> bool {
        return self.moves.contains(&mov);
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get_moves(&self) -> Vec<Mov> {
        let mut retval = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            retval.push(self.moves[i]);
        }
        return retval;
    }
}

#[cfg(test)]
pub mod tests {
    use components::square::Square;
    use moves::mov::Mov;
    use moves::move_list::MoveList;
    use std::collections::HashSet;

    #[test]
    pub fn move_list_defaults_to_empty() {
        let movelist = MoveList::default();

        assert!(movelist.len() == 0);
        assert!(movelist.get_moves().len() == 0);
    }

    #[test]
    pub fn move_list_push_elements_number_as_expected() {
        const NUM_ELEMS: usize = 20;
        let test_move = Mov::default();

        let mut movelist = MoveList::default();
        for _ in 0..NUM_ELEMS {
            movelist.push(test_move);
        }

        assert!(movelist.len() == NUM_ELEMS);
        assert!(movelist.get_moves().len() == NUM_ELEMS);

        for mv in &movelist.get_moves() {
            assert!(*mv == test_move);
        }
    }

    #[test]
    pub fn move_list_contains_as_expected() {
        let mv1 = Mov::encode_move_castle_kingside_white();
        let mv2 = Mov::encode_move_castle_kingside_black();
        let mv3 = Mov::encode_move_quiet(Square::a1, Square::a2);
        let mv4 = Mov::encode_move_quiet(Square::h1, Square::h2);

        let mut movelist = MoveList::default();
        movelist.push(mv1);
        movelist.push(mv2);
        movelist.push(mv3);

        assert!(movelist.contains(mv1));
        assert!(movelist.contains(mv2));
        assert!(movelist.contains(mv3));
        assert!(movelist.contains(mv4) == false);
    }

    #[test]
    pub fn move_list_get_moves_as_expected() {
        let mv1 = Mov::encode_move_castle_kingside_white();
        let mv2 = Mov::encode_move_castle_kingside_black();
        let mv3 = Mov::encode_move_quiet(Square::a1, Square::a2);

        let mut movelist = MoveList::default();
        movelist.push(mv1);
        movelist.push(mv2);
        movelist.push(mv3);

        let moves = movelist.get_moves();
        assert!(moves.len() == 3);

        assert!(moves.contains(&mv1));
        assert!(moves.contains(&mv2));
        assert!(moves.contains(&mv3));
    }

    #[test]
    pub fn move_list_iterator() {
        let mut movelist = MoveList::default();

        let mut test_moves: HashSet<Mov> = HashSet::new();
        test_moves.insert(Mov::encode_move_castle_kingside_white());
        test_moves.insert(Mov::encode_move_castle_kingside_black());
        test_moves.insert(Mov::encode_move_quiet(Square::a1, Square::a2));

        for mv in test_moves.iter() {
            movelist.push(*mv);
        }
        assert!(movelist.len() == 3);

        let mut cntr = 0;
        for test_mv in movelist {
            cntr += 1;
            assert!(test_moves.contains(&test_mv));
        }
        assert!(cntr == 3);
    }
}
