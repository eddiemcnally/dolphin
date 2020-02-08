use moves::move_gen;
use position::position::MoveLegality;
use position::position::Position;

pub fn perft(depth: u8, position: &mut Position) -> u64 {
    let mut move_cntr = 0;
    if depth == 0 {
        return 1;
    }

    let mut move_list = Vec::new();

    move_gen::generate_moves(position, &mut move_list);

    for mv in &move_list {
        let move_legality = position.make_move(*mv);
        if move_legality == MoveLegality::Legal {
            move_cntr = move_cntr + perft(depth - 1, position);
        }
        position.take_move();
    }

    return move_cntr;
}

#[cfg(test)]
pub mod tests {
    use input::fen;
    use perft::perft_runner;
    use position::position::Position;

    #[test]
    pub fn sample_perft() {
        let depth = 2;
        let expected_move_count = 36;

        // 6kq/8/8/8/8/8/8/7K w - - 0 1 ;D1 2 ;D2 36 ;D3 143 ;D4 3637 ;D5 14893 ;D6 391507

        let fen = "6kq/8/8/8/8/8/8/7K w - - 0 1 ";
        let parsed_fen = fen::get_position(&fen);
        let mut position = Position::new(parsed_fen);

        let num_moves = perft_runner::perft(depth, &mut position);

        assert_eq!(num_moves, expected_move_count);
    }
}
