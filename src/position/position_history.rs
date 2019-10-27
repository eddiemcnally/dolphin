use board::square::Square;
use moves::mov::Mov;
use position::castle_permissions::CastlePermission;
use position::hash::PositionHash;

struct History {
    position_key: PositionHash,
    mov: Mov,
    fifty_move_cntr: u8,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
}

pub struct PositionHistory {
    max_hist_size: u16,
    history: Vec<History>,
}

impl PositionHistory {
    // new
    pub fn new(max_hist_size: u16) -> PositionHistory {
        let mut hist: Vec<History> = Vec::new();
        hist.reserve(max_hist_size as usize);

        PositionHistory {
            max_hist_size: max_hist_size,
            history: hist,
        }
    }

    // push
    pub fn push(
        &mut self,
        pos_key: PositionHash,
        mov: Mov,
        fifty_move_cntr: u8,
        en_pass_sq: Option<Square>,
        castle_perm: CastlePermission,
    ) {
        debug_assert!(
            self.history.len() <= self.max_hist_size as usize,
            "max length exceeded. {:?}",
            self.max_hist_size
        );

        let hist = History {
            position_key: pos_key,
            mov: mov,
            fifty_move_cntr: fifty_move_cntr,
            en_pass_sq: en_pass_sq,
            castle_perm: castle_perm,
        };

        self.history.push(hist);
    }

    pub fn pop(&mut self) -> (PositionHash, Mov, u8, Option<Square>, CastlePermission) {
        debug_assert!(self.history.len() > 0, "attempt to pop, len = 0");

        let popped = self.history.pop();
        match popped {
            None => panic!(""),
            Some(popped) => {
                let pos_key = popped.position_key;
                let mov = popped.mov;
                let fifty_move_cntr = popped.fifty_move_cntr;
                let en_pass_sq = popped.en_pass_sq;
                let castle_perm = popped.castle_perm;

                (pos_key, mov, fifty_move_cntr, en_pass_sq, castle_perm)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn capacity(&self) -> usize {
        self.history.capacity()
    }
}

#[cfg(test)]
mod tests {
    use moves::mov::Mov;
    use position::castle_permissions::CastlePermission;
    use position::hash::PositionHash;
    use position::position_history::PositionHistory;

    #[test]
    pub fn ensure_init_capacity_is_max_size() {
        let mut pos_hist = PositionHistory::new(10);
        assert_eq!(pos_hist.capacity(), 10);
        pos_hist = PositionHistory::new(2048);
        assert_eq!(pos_hist.capacity(), 2048);
    }

    #[test]
    pub fn posh_pop_element_order_as_expected() {
        let num_to_test = 50;

        let mut pos_hist = PositionHistory::new(num_to_test);

        // push multiple positions
        for i in 0..num_to_test {
            let pk = PositionHash::new();
            let mv = Mov::encode_move_castle_queenside_white();
            let enp = None;
            let fifty_move_cntr = i as u8;
            let castperm = CastlePermission::new();

            pos_hist.push(pk, mv, fifty_move_cntr, enp, castperm);
        }

        // pop and check the order
        for i in num_to_test..0 {
            let (_, _, fifty_cntr, _, _) = pos_hist.pop();
            assert_eq!(fifty_cntr, i as u8);
        }
    }

    #[test]
    pub fn posh_pop_len_as_expected() {
        let num_to_test = 50;

        let mut pos_hist = PositionHistory::new(num_to_test);

        assert_eq!(pos_hist.len(), 0);

        // push multiple positions
        for i in 0..num_to_test {
            let pk = PositionHash::new();
            let mv = Mov::encode_move_castle_queenside_white();
            let enp = None;
            let fifty_move_cntr = i as u8;
            let castperm = CastlePermission::new();

            pos_hist.push(pk, mv, fifty_move_cntr, enp, castperm);

            assert_eq!(pos_hist.len(), (i + 1) as usize);
        }
    }
}
