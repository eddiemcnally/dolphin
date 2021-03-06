use crate::evaluate;
use crate::mov::Mov;
use crate::mov::MovPlusScore;
use crate::move_gen::MoveGenerator;
use crate::move_list::MoveList;
use crate::position::MoveLegality;
use crate::position::Position;
use crate::tt::TransTable;
use crate::tt::TransType;
use core::cmp::max;
use core::cmp::min;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Stats {
    enabled: bool,
    found_in_tt: u32,
    num_illegal_moves: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            enabled: false,
            found_in_tt: 0,
            num_illegal_moves: 0,
        }
    }
}
impl Stats {
    fn new(enable_stats: bool) -> Self {
        let mut stats = Stats::default();
        stats.enabled = enable_stats;

        return stats;
    }
}

pub struct Search {
    stats: Stats,
    tt: TransTable,
    max_depth: u8,
}

const POS_INFINITE: i32 = i32::MAX;
const NEG_INFINITE: i32 = i32::MIN;
const MAX_DEPTH: u8 = 6;

impl Search {
    pub fn new(enable_stats: bool, tt_capacity: usize, max_depth: u8) -> Self {
        Search {
            stats: Stats::new(enable_stats),
            tt: TransTable::new(tt_capacity, enable_stats),
            max_depth,
        }
    }

    pub fn start_search(
        &mut self,
        pos: &mut Position,
        alpha_start: i32,
        beta_start: i32,
    ) -> MovPlusScore {
        let mut alpha = alpha_start;
        let beta = beta_start;
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();

        // // iterative deepening
        // for depth in 1..self.max_depth{
        //     let best_score = self.alpha_beta(pos, depth, POS_INFINITE, NEG_INFINITE);

        // }

        let mut best_move_plus_score = MovPlusScore::default();

        move_gen.generate_moves(pos, &mut move_list);

        for mv in move_list.iter() {
            let mut score = 0;
            let legality = pos.make_move(*mv);
            if legality == MoveLegality::Legal {
                score = -self.alpha_beta(pos, self.max_depth, -beta, -alpha, &move_gen);
            }
            pos.take_move();

            if score > alpha {
                alpha = score;
                best_move_plus_score = MovPlusScore::new(*mv, score);
            }
        }

        return best_move_plus_score;
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

        let t: Option<(TransType, u8, i32)> = self.tt.get(hash);
        if t.is_some() {
            let tt_type = t.unwrap().0;
            let cached_depth = t.unwrap().1;
            let cached_score = t.unwrap().2;

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

        return alpha;
    }

    fn quiesence(&mut self, _pos: &mut Position, _depth: u8, _alpha: i32, _beta: i32) -> i32 {
        // let original_alpha = alpha;

        // if pos.move_counter().

        // let stand_pat = evaluate::evaluate_board(pos.board(), pos.side_to_move());

        // if stand_pat >= beta{
        //     return beta;
        // }

        // if original_alpha < stand_pat{
        //     alpha = stand_pat;
        // }

        // let mut move_list: Vec<Mov> = Vec::new();
        // move_gen::generate_moves(pos, &move_list);

        // for mv in move_list{
        //     if mv.is_capture(){
        //         pos.make_move(mv);

        //         let score = -self.quiesence(pos, depth, alpha, beta)

        //     }
        // }
        return 0;
    }

    //     int Quiesce( int alpha, int beta ) {
    //     int stand_pat = Evaluate();
    //     if( stand_pat >= beta )
    //         return beta;
    //     if( alpha < stand_pat )
    //         alpha = stand_pat;

    //     until( every_capture_has_been_examined )  {
    //         MakeCapture();
    //         score = -Quiesce( -beta, -alpha );
    //         TakeBackMove();

    //         if( score >= beta )
    //             return beta;
    //         if( score > alpha )
    //            alpha = score;
    //     }
    //     return alpha;
    // }

    // TODO
    fn sort_move_list(_move_list: &mut Vec<Mov>) {
        //move_list.sort_by(|a, b| b.age.cmp(&a.age));
    }
}
