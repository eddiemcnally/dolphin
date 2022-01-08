use crate::board::piece::Piece;
use crate::moves::mov::Mov;
use crate::position::game_position::GameState;
use std::fmt;

#[derive(Default, Eq, PartialEq, Copy, Clone)]
struct Item {
    game_state: GameState,
    mov: Mov,
    pce_moved: Piece,
    pce_captured: Option<Piece>,
}

#[derive(Eq, Copy, Clone)]
pub struct PositionHistory {
    count: u16,
    history: [Item; PositionHistory::MAX_MOVE_HISTORY],
}

impl Default for PositionHistory {
    fn default() -> Self {
        PositionHistory {
            count: 0,
            history: [Item::default(); PositionHistory::MAX_MOVE_HISTORY],
        }
    }
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
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("GameState : {}\n", self.game_state));
        debug_str.push_str(&format!("Move: : {}\n", self.mov));
        debug_str.push_str(&format!("Piece: : {}\n", self.pce_moved));

        if self.pce_captured.is_none() {
            debug_str.push_str(&"Captured Piece : -\n".to_string());
        } else {
            debug_str.push_str(&format!(
                "Captured Piece : {}\n",
                self.pce_captured.unwrap()
            ));
        }

        write!(f, "{}", debug_str)
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
            history: [Item::default(); PositionHistory::MAX_MOVE_HISTORY],
        })
    }

    // push
    pub fn push(
        &mut self,
        game_state: GameState,
        mv: Mov,
        piece: Piece,
        capt_piece: Option<Piece>,
    ) {
        debug_assert!(
            self.count <= (PositionHistory::MAX_MOVE_HISTORY - 1) as u16,
            "max length exceeded. {:?}",
            self.count
        );

        let item = Item {
            game_state,
            mov: mv,
            pce_moved: piece,
            pce_captured: capt_piece,
        };

        self.history[self.count as usize] = item;
        self.count += 1;
    }

    pub fn pop(&mut self) -> (GameState, Mov, Piece, Option<Piece>) {
        debug_assert!(self.count > 0, "attempt to pop, len = 0");

        self.count -= 1;

        (
            self.history[self.count as usize].game_state,
            self.history[self.count as usize].mov,
            self.history[self.count as usize].pce_moved,
            self.history[self.count as usize].pce_captured,
        )
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
