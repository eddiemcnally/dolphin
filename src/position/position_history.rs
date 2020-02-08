use board::board::Board;
use board::piece::Piece;
use board::square::Square;
use moves::mov::Mov;
use position::castle_permissions::CastlePermission;
use std::fmt;

#[derive(Clone, Copy)]
struct History {
    board: Board,
    position_hash: u64,
    mov: Mov,
    fifty_move_cntr: u8,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
    capt_piece: Option<Piece>,
}

impl fmt::Debug for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("PositionHash : {}\n", self.position_hash));
        debug_str.push_str(&format!("Mov : {}\n", self.mov));
        debug_str.push_str(&format!("FiftyMoveCntr : {}\n", self.fifty_move_cntr));
        if self.en_pass_sq.is_none() {
            debug_str.push_str(&format!("En pass Sq : -\n"));
        } else {
            debug_str.push_str(&format!("En pass Sq : {}\n", self.en_pass_sq.unwrap()));
        }

        debug_str.push_str("\n");

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl PartialEq for History {
    fn eq(&self, other: &Self) -> bool {
        if self.board != other.board {
            println!("POS: boards are different");
            return false;
        }

        if self.position_hash != other.position_hash {
            println!("POS: position hashes are different");
            return false;
        }

        if self.mov != other.mov {
            println!("POS: moves are different");
            return false;
        }

        if self.castle_perm != other.castle_perm {
            println!("POS: castle permissions are different");
            return false;
        }

        if self.fifty_move_cntr != other.fifty_move_cntr {
            println!("POS: 50-move counters are different");
            return false;
        }
        if self.en_pass_sq != other.en_pass_sq {
            println!("POS: en passant squares are different");
            return false;
        }

        return true;
    }
}

pub struct PositionHistory {
    max_hist_size: u16,
    history: Vec<History>,
}

impl PartialEq for PositionHistory {
    fn eq(&self, other: &Self) -> bool {
        if self.max_hist_size != other.max_hist_size {
            println!("POS: max sizes are different");
            return false;
        }

        if self.history != other.history {
            return false;
        }

        return true;
    }
}

impl fmt::Debug for PositionHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        if self.history.len() == 0 {
            debug_str.push_str(&format!("Hist : Empty\n"));
        } else {
            for h in &self.history {
                debug_str.push_str(&format!("Hist : {}\n", h));
            }
        }

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for PositionHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
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
        board: Board,
        position_hash: u64,
        mov: Mov,
        fifty_move_cntr: u8,
        en_pass_sq: Option<Square>,
        castle_perm: CastlePermission,
        capt_piece: Option<Piece>,
    ) {
        debug_assert!(
            self.history.len() <= self.max_hist_size as usize,
            "max length exceeded. {:?}",
            self.max_hist_size
        );

        let hist = History {
            board: board,
            position_hash: position_hash,
            mov: mov,
            fifty_move_cntr: fifty_move_cntr,
            en_pass_sq: en_pass_sq,
            castle_perm: castle_perm,
            capt_piece: capt_piece,
        };

        self.history.push(hist);
    }

    pub fn pop(
        &mut self,
    ) -> (
        Board,
        u64,
        Mov,
        u8,
        Option<Square>,
        CastlePermission,
        Option<Piece>,
    ) {
        debug_assert!(self.history.len() > 0, "attempt to pop, len = 0");

        let popped = self.history.pop();
        match popped {
            None => panic!("Nothing to pop from history"),
            Some(popped) => {
                let board = popped.board;
                let pos_key = popped.position_hash;
                let mov = popped.mov;
                let fifty_move_cntr = popped.fifty_move_cntr;
                let en_pass_sq = popped.en_pass_sq;
                let castle_perm = popped.castle_perm;
                let capt_piece = popped.capt_piece;
                (
                    board,
                    pos_key,
                    mov,
                    fifty_move_cntr,
                    en_pass_sq,
                    castle_perm,
                    capt_piece,
                )
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
    use board::board::Board;
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;
    use moves::mov::Mov;
    use position::castle_permissions;
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
            let board = Board::new();
            let pk = 1234;
            let mv = Mov::encode_move_castle_queenside_white();
            let enp = None;
            let fifty_move_cntr = i as u8;
            let castperm = castle_permissions::NO_CASTLE_PERMS;
            let capt_pce = Some(Piece::new(PieceRole::Bishop, Colour::Black));

            pos_hist.push(board, pk, mv, fifty_move_cntr, enp, castperm, capt_pce);
        }

        // pop and check the order
        for i in num_to_test..0 {
            let (_, _, _, fifty_cntr, _, _, _) = pos_hist.pop();
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
            let board = Board::new();
            let pk = 1234;
            let mv = Mov::encode_move_castle_queenside_white();
            let enp = None;
            let fifty_move_cntr = i as u8;
            let castperm = castle_permissions::NO_CASTLE_PERMS;

            pos_hist.push(board, pk, mv, fifty_move_cntr, enp, castperm, None);

            assert_eq!(pos_hist.len(), (i + 1) as usize);
        }
    }
}
