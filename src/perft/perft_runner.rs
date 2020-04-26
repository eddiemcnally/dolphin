use engine::position::MoveLegality;
use engine::position::Position;
use moves::move_gen;

pub fn perft(depth: u8, position: &mut Position) -> u64 {
    let mut nodes = 0;
    if depth == 0 {
        return 1;
    }

    let mut move_list = Vec::new();

    move_gen::generate_moves(position, &mut move_list);

    for mv in &move_list {
        let move_legality = position.make_move(*mv);
        if move_legality == MoveLegality::Legal {
            nodes += perft(depth - 1, position);
        }
        position.take_move();
    }

    //println!("#Nodes: {}, Move List: {:?}", nodes, move_list);
    nodes
}

#[cfg(test)]
pub mod tests {
    use engine::position::Position;
    use input::fen;
    use perft::perft_runner;

    #[test]
    pub fn sample_perft_1() {
        let depth = 4;
        let expected_move_count = 197281;

        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281 ;D5 4865609 ;D6 119060324

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut position = Position::new(parsed_fen);

        let num_moves = perft_runner::perft(depth, &mut position);

        assert_eq!(num_moves, expected_move_count);
    }

    #[test]
    pub fn sample_perft_2() {
        let depth = 6;
        let expected_move_count = 158065;

        // 8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1 ;D1 8 ;D2 61 ;D3 411 ;D4 3213 ;D5 21637 ;D6 158065

        let fen = "8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1 ";
        let parsed_fen = fen::get_position(&fen);
        let mut position = Position::new(parsed_fen);

        let num_moves = perft_runner::perft(depth, &mut position);

        assert_eq!(num_moves, expected_move_count);
    }
}
