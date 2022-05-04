use crate::moves::mov::Move;
use crate::moves::move_gen::MoveGenerator;
use crate::moves::move_list::MoveList;
use crate::position::game_position::MoveLegality;
use crate::position::game_position::Position;
use crate::search_engine::evaluate::evaluate_board;
use crate::search_engine::tt::TransTable;
use crate::search_engine::tt::TransType;

const SCORE_INFINITE: i32 = 30000;
const SCORE_MATE: i32 = 29000;

#[derive(Default)]
pub struct Search {
    // input to search
    max_depth: u8,

    // runtime info
    tt: TransTable,
}

impl Search {
    const MOVE_ORDER_WEIGHT_PV_MOVE: i32 = 2000000;

    pub fn new(tt_capacity: usize, max_depth: u8) -> Self {
        Search {
            tt: TransTable::new(tt_capacity),
            max_depth,
        }
    }

    pub fn search(&mut self, pos: &mut Position) {
        // iterative deepening
        for depth in 1..self.max_depth {
            self.alpha_beta(pos, -SCORE_INFINITE, SCORE_INFINITE, depth);

            let pv_line = self.get_pv_line(pos, depth);

            //let best_move = pv_line[0];

            println!("SEARCH: depth : {}, PV Line : ", depth);
            for m in pv_line.iter() {
                println!("{}   ", *m);
            }
        }
    }

    fn get_pv_line(&mut self, pos: &mut Position, depth: u8) -> Vec<Move> {
        let mut retval = Vec::<Move>::new();

        let mut mv = self.tt.get_move_for_position_hash(pos.position_hash());
        let mut i = 0u8;

        while mv.is_some() && i < depth {
            pos.make_move(mv.unwrap());
            retval.push(mv.unwrap());
            i += 1;
            mv = self.tt.get_move_for_position_hash(pos.position_hash());
        }

        for _ in 0..i {
            pos.take_move();
        }

        retval
    }

    fn alpha_beta(&mut self, pos: &mut Position, mut alpha: i32, beta: i32, depth: u8) -> i32 {
        if depth == 0 {
            return self.quiesence(pos, alpha, beta);
        }

        let mut num_legal_moves = 0;

        // TODO: check if timer expired
        // TODO: check for repetition
        // TODO: check for 50 move counter

        let old_alpha = alpha;

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::default();

        move_gen.generate_moves(pos, &mut move_list);

        // check to see if current position is in transposition table
        // and if it is, set the score so we can prioritise it
        if let Some((_, _, _, mv)) = self.tt.get(pos.position_hash()) {
            if let Some(offset) = move_list.get_offset_for_move(mv) {
                move_list.set_score_for_move_at(offset, Search::MOVE_ORDER_WEIGHT_PV_MOVE);
            } else {
                panic!("Cant find move in list, but is in TT");
            }
        }

        let mut best_move: Move = Move::default();

        for i in 0..move_list.len() {
            // sort to bring highest score to the top
            move_list.sort_by_score(i);

            let mv = move_list.get_move_at_offset(i);

            let move_legality = pos.make_move(mv);
            if move_legality == MoveLegality::Illegal {
                pos.take_move();
                continue;
            }
            num_legal_moves += 1;

            // note: alpha/beta are swapped, and sign is reversed
            let score = -self.alpha_beta(pos, -beta, -alpha, depth - 1);
            pos.take_move();

            if score > alpha {
                if score > beta {
                    self.tt
                        .add(TransType::Beta, depth, score, pos.position_hash(), mv);
                    return beta;
                }
                best_move = mv;

                alpha = score;
                self.tt
                    .add(TransType::Alpha, depth, score, pos.position_hash(), mv);
            }
        }

        // check for mate
        if num_legal_moves == 0 {
            if pos.is_king_sq_attacked() {
                return -SCORE_MATE + pos.move_counter().half_move() as i32;
            } else {
                return 0_i32;
            }
        }

        if alpha != old_alpha {
            self.tt.add(
                TransType::Exact,
                depth,
                best_move.get_score(),
                pos.position_hash(),
                best_move,
            );
        }
        alpha
    }

    fn quiesence(&mut self, pos: &mut Position, mut alpha: i32, beta: i32) -> i32 {
        // TODO check repetition
        // TODO checkl 50 move counter
        // TODO check max depth

        // stand pat
        let stand_pat_score = evaluate_board(pos.board(), pos.side_to_move());
        if stand_pat_score >= beta {
            return beta;
        }
        if stand_pat_score > alpha {
            alpha = stand_pat_score;
        }

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::default();

        move_gen.generate_moves(pos, &mut move_list);

        for i in 0..move_list.len() {
            // sort to bring highest score to the top
            move_list.sort_by_score(i);

            let mv = move_list.get_move_at_offset(i);

            let move_legality = pos.make_move(mv);
            if move_legality == MoveLegality::Illegal {
                pos.take_move();
                continue;
            }

            // note: alpha/beta are swapped, and sign is reversed
            let score = -self.quiesence(pos, -beta, -alpha);
            pos.take_move();

            if score > alpha {
                if score > beta {
                    return beta;
                }
                alpha = score;
            }
        }

        alpha
    }
}
