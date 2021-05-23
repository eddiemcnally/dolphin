use crate::castle_permissions;
use crate::castle_permissions::CastlePermission;
use crate::mov::Mov;
use crate::piece::Piece;
use crate::square::Square;
use crate::zobrist_keys::ZobristHash;
use std::fmt;

#[derive(Clone, Copy)]
pub struct GameState {
    position_hash: ZobristHash,
    mov: Mov,
    fifty_move_cntr: u8,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
    piece_being_moved: Piece,
    capt_piece: Option<Piece>,
}

impl GameState {
    pub fn new(
        position_hash: ZobristHash,
        mov: Mov,
        fifty_move_cntr: u8,
        en_pass_sq: Option<Square>,
        castle_perm: CastlePermission,
        piece_being_moved: Piece,
        capt_piece: Option<Piece>,
    ) -> GameState {
        GameState {
            position_hash,
            mov,
            fifty_move_cntr,
            en_pass_sq,
            castle_perm,
            piece_being_moved,
            capt_piece,
        }
    }

    pub fn position_hash(&self) -> ZobristHash {
        self.position_hash
    }

    pub fn mov(&self) -> Mov {
        self.mov
    }

    pub fn fifty_move_cntr(&self) -> u8 {
        self.fifty_move_cntr
    }

    pub fn en_pass_sq(&self) -> Option<Square> {
        self.en_pass_sq
    }

    pub fn castle_permissions(&self) -> CastlePermission {
        self.castle_perm
    }

    pub fn piece_being_moved(&self) -> Piece {
        self.piece_being_moved
    }
    pub fn captured_piece(&self) -> Option<Piece> {
        self.capt_piece
    }
}

impl fmt::Debug for GameState {
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

impl Default for GameState {
    fn default() -> Self {
        GameState {
            position_hash: 0,
            mov: Mov::encode_move_quiet(Square::a1, Square::a2),
            fifty_move_cntr: 0,
            en_pass_sq: None,
            castle_perm: castle_permissions::NO_CASTLE_PERMS_AVAIL,
            piece_being_moved: Piece::WhitePawn,
            capt_piece: None,
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
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
    history: [GameState; PositionHistory::MAX_MOVE_HISTORY],
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
    pub fn new() -> Box<PositionHistory> {
        Box::new(PositionHistory {
            count: 0,
            history: [GameState::default(); PositionHistory::MAX_MOVE_HISTORY],
        })
    }

    // push
    pub fn push(&mut self, game_state: &GameState) {
        debug_assert!(
            self.count <= (PositionHistory::MAX_MOVE_HISTORY - 1) as u16,
            "max length exceeded. {:?}",
            self.count
        );

        self.history[self.count as usize] = *game_state;
        self.count += 1;
    }

    pub fn pop(&mut self) -> GameState {
        debug_assert!(self.count > 0, "attempt to pop, len = 0");

        self.count -= 1;
        return self.history[self.count as usize];
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::castle_permissions;
    use crate::mov::Mov;
    use crate::piece::Piece;
    use crate::position_history::GameState;
    use crate::position_history::PositionHistory;
    use crate::square::Square;

    #[test]
    pub fn posh_pop_element_order_as_expected() {
        let num_to_test = 50;

        let mut pos_hist = PositionHistory::new();

        // push multiple positions
        for i in 0..num_to_test {
            let state = GameState::new(
                1234,
                Mov::encode_move_castle_queenside_white(),
                i,
                Some(Square::a3),
                castle_permissions::NO_CASTLE_PERMS_AVAIL,
                Piece::BlackQueen,
                Some(Piece::BlackBishop),
            );
            pos_hist.push(&state);
        }

        // pop and check the order
        for i in num_to_test..0 {
            let state = pos_hist.pop();
            assert_eq!(state.fifty_move_cntr, i as u8);
        }
    }

    #[test]
    pub fn posh_pop_len_as_expected() {
        let num_to_test = 50;

        let mut pos_hist = PositionHistory::new();

        assert_eq!(pos_hist.len(), 0);

        // push multiple positions
        for i in 0..num_to_test {
            let state = GameState::new(
                1234,
                Mov::encode_move_castle_queenside_white(),
                i,
                Some(Square::a3),
                castle_permissions::NO_CASTLE_PERMS_AVAIL,
                Piece::BlackQueen,
                Some(Piece::BlackBishop),
            );
            pos_hist.push(&state);
            assert_eq!(pos_hist.len(), (i + 1) as usize);
        }
    }
}
