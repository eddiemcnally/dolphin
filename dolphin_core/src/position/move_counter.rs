use std::fmt;
#[derive(Default, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

impl MoveCounter {
    pub fn new(half_cntr: u16, full_cntr: u16) -> MoveCounter {
        MoveCounter {
            half_move: half_cntr,
            full_move: full_cntr,
        }
    }
    pub fn incr_half_move(&mut self) {
        self.half_move += 1;
    }
    pub fn incr_full_move(&mut self) {
        self.full_move += 1;
    }

    pub fn half_move(&self) -> u16 {
        self.half_move
    }
    pub fn full_move(&self) -> u16 {
        self.full_move
    }
}

impl fmt::Debug for MoveCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("HalfMove : {}, ", self.half_move));
        debug_str.push_str(&format!("FullMove : {} ", self.full_move));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for MoveCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::io::fen;

    #[test]
    pub fn move_counters_equality_as_expected() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 11 12";

        let (_, mc1, _, _, _) = fen::decompose_fen(fen);
        let (_, mc2, _, _, _) = fen::decompose_fen(fen);

        assert_eq!(mc1, mc2);
    }
}
