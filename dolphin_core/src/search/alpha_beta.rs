use crate::moves::mov::Move;
use crate::moves::mov::MoveTrait;
use crate::moves::move_gen::MoveGenerator;
use crate::moves::move_list::MoveList;
use crate::position::game_position::MoveLegality;
use crate::position::game_position::Position;
use crate::search::evaluate;
use crate::search::tt::TransTable;
use crate::search::tt::TransType;
use core::cmp::max;
use core::cmp::min;

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Stats {
    enabled: bool,
    found_in_tt: u32,
    num_illegal_moves: u32,
}

pub struct Search {
    tt: TransTable,
    max_depth: u8,
}

impl Search {
    pub fn new(tt_capacity: usize, max_depth: u8) -> Self {
        Search {
            tt: TransTable::new(tt_capacity),
            max_depth,
        }
    }

    pub fn start_search(&mut self, pos: &mut Position, alpha_start: i32, beta_start: i32) -> Move {
        let mut alpha = alpha_start;
        let beta = beta_start;
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();

        // // iterative deepening
        // for depth in 1..self.max_depth{
        //     let best_score = self.alpha_beta(pos, depth, POS_INFINITE, NEG_INFINITE);

        // }

        move_gen.generate_moves(pos, &mut move_list);
        let mut best_move = Move::default();

        for mv in move_list.iter() {
            let mut score = 0;
            let legality = pos.make_move(*mv);
            if legality == MoveLegality::Legal {
                score = -self.alpha_beta(pos, self.max_depth, -beta, -alpha, &move_gen);
            }
            pos.take_move();

            if score > alpha {
                alpha = score;

                best_move = *mv;
                best_move.set_score(score);
            }
        }

        best_move
    }

    fn alpha_beta(
        &mut self,
        pos: &mut Position,
        depth: u8,
        mut alpha: i32,
        beta: i32,
        move_gen: &MoveGenerator,
    ) -> i32 {
        let original_alpha = alpha;

        let hash = pos.position_hash();

        if let Some(t) = self.tt.get(hash) {
            // let t: Option<(TransType, u8, i32)> = self.tt.get(hash);
            // if t.is_some() {
            let tt_type = t.0;
            let cached_depth = t.1;
            let cached_score = t.2;

            if cached_depth >= depth {
                match tt_type {
                    TransType::Exact => return cached_score,
                    TransType::Lower => return max(alpha, cached_score),
                    TransType::Upper => return min(beta, cached_score),
                }
            }
            if alpha > beta {
                return cached_score;
            }
        }

        if depth == 0 {
            return evaluate::evaluate_board(pos.board(), pos.side_to_move());
        }

        let mut move_list = MoveList::new();
        move_gen.generate_moves(pos, &mut move_list);

        let mut score = -1_000_000;

        for mv in move_list.iter() {
            let legality = pos.make_move(*mv);
            if legality == MoveLegality::Legal {
                score = -self.alpha_beta(pos, depth - 1, -beta, -alpha, move_gen);
            }
            pos.take_move();

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        let tt_type = if score <= original_alpha {
            TransType::Upper
        } else if score >= beta {
            TransType::Lower
        } else {
            TransType::Exact
        };

        self.tt.add(tt_type, depth, score, hash);

        alpha
    }
}
