use components::board::Board;
use components::piece::Piece;
use components::square::Square;
use engine::castle_permissions;
use engine::castle_permissions::CastlePermission;
use engine::hash::PositionHash;
use moves::mov::Mov;
use std::fmt;

#[derive(Clone, Copy)]
struct HistoryItem {
    board: Board,
    position_hash: PositionHash,
    mov: Mov,
    fifty_move_cntr: u8,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
    capt_piece: Option<&'static Piece>,
}

impl fmt::Debug for HistoryItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("PositionHash : {}\n", self.position_hash));
        debug_str.push_str(&format!("Mov : {}\n", self.mov));
        debug_str.push_str(&format!("FiftyMoveCntr : {}\n", self.fifty_move_cntr));
        if self.en_pass_sq.is_none() {
            debug_str.push_str(&"En pass Sq : -\n".to_string());
        } else {
            debug_str.push_str(&format!("En pass Sq : {}\n", self.en_pass_sq.unwrap()));
        }

        debug_str.push_str("\n");

        write!(f, "{}", debug_str)
    }
}

impl Default for HistoryItem {
    fn default() -> Self {
        HistoryItem {
            board: Board::default(),
            position_hash: PositionHash::default(),
            mov: Mov::encode_move_quiet(Square::a1, Square::a2),
            fifty_move_cntr: 0,
            en_pass_sq: None,
            castle_perm: castle_permissions::NO_CASTLE_PERMS,
            capt_piece: None,
        }
    }
}

impl fmt::Display for HistoryItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl PartialEq for HistoryItem {
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

        true
    }
}

pub struct PositionHistory {
    count: u16,
    history: [HistoryItem; PositionHistory::MAX_MOVE_HISTORY],
}

impl PartialEq for PositionHistory {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            println!("POS: max sizes are different");
            return false;
        }

        for i in 0..self.count {
            if self.history[i as usize] != other.history[i as usize] {
                return false;
            }
        }
        true
    }
}

impl fmt::Debug for PositionHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        if self.history.is_empty() {
            debug_str.push_str(&"Hist : Empty\n".to_string());
        } else {
            for i in 0..self.count {
                debug_str.push_str(&format!("Hist : {}\n", self.history[i as usize]));
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
    const MAX_MOVE_HISTORY: usize = 1024;

    // new
    pub fn new() -> PositionHistory {
        PositionHistory {
            count: 0,
            history: [HistoryItem::default(); PositionHistory::MAX_MOVE_HISTORY],
        }
    }

    // push
    pub fn push(
        &mut self,
        board: &Board,
        position_hash: PositionHash,
        mov: Mov,
        fifty_move_cntr: u8,
        en_pass_sq: Option<Square>,
        castle_perm: CastlePermission,
        capt_piece: Option<&'static Piece>,
    ) {
        debug_assert!(
            self.count <= (PositionHistory::MAX_MOVE_HISTORY - 1) as u16,
            "max length exceeded. {:?}",
            self.count
        );

        self.history[self.count as usize].board = *board;
        self.history[self.count as usize].position_hash = position_hash;
        self.history[self.count as usize].mov = mov;
        self.history[self.count as usize].fifty_move_cntr = fifty_move_cntr;
        self.history[self.count as usize].en_pass_sq = en_pass_sq;
        self.history[self.count as usize].castle_perm = castle_perm;
        self.history[self.count as usize].capt_piece = capt_piece;

        self.count += 1;
    }

    pub fn pop(
        &mut self,
    ) -> (
        Board,
        PositionHash,
        Mov,
        u8,
        Option<Square>,
        CastlePermission,
        Option<&'static Piece>,
    ) {
        debug_assert!(self.count > 0, "attempt to pop, len = 0");

        self.count -= 1;

        (
            self.history[self.count as usize].board,
            self.history[self.count as usize].position_hash,
            self.history[self.count as usize].mov,
            self.history[self.count as usize].fifty_move_cntr,
            self.history[self.count as usize].en_pass_sq,
            self.history[self.count as usize].castle_perm,
            self.history[self.count as usize].capt_piece,
        )
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }
}

#[cfg(test)]
mod tests {
    use components::board::Board;
    use components::piece::Colour;
    use components::piece::Piece;
    use components::piece::PieceRole;
    use engine::castle_permissions;
    use engine::hash::PositionHash;
    use engine::position_history::PositionHistory;
    use moves::mov::Mov;

    #[test]
    pub fn posh_pop_element_order_as_expected() {
        let num_to_test = 50;

        let mut pos_hist = PositionHistory::new();

        // push multiple positions
        for i in 0..num_to_test {
            let board = Board::new();
            let pk = PositionHash::new(1234);
            let mv = Mov::encode_move_castle_queenside_white();
            let enp = None;
            let fifty_move_cntr = i as u8;
            let castperm = castle_permissions::NO_CASTLE_PERMS;
            let capt_pce = Some(Piece::new(PieceRole::Bishop, Colour::Black));

            pos_hist.push(&board, pk, mv, fifty_move_cntr, enp, castperm, capt_pce);
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

        let mut pos_hist = PositionHistory::new();

        assert_eq!(pos_hist.len(), 0);

        // push multiple positions
        for i in 0..num_to_test {
            let board = Board::new();
            let pk = PositionHash::new(1234);
            let mv = Mov::encode_move_castle_queenside_white();
            let enp = None;
            let fifty_move_cntr = i as u8;
            let castperm = castle_permissions::NO_CASTLE_PERMS;

            pos_hist.push(&board, pk, mv, fifty_move_cntr, enp, castperm, None);

            assert_eq!(pos_hist.len(), (i + 1) as usize);
        }
    }
}
