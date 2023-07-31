extern crate dolphin_core;
use dolphin_core::moves::move_gen::MoveGenerator;
use dolphin_core::moves::move_list::MoveList;
use dolphin_core::position::game_position::MoveLegality;
use dolphin_core::position::game_position::Position;

pub fn perft(depth: u8, position: &mut Position, move_generator: &MoveGenerator) -> u64 {
    let mut nodes = 0;
    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();

    move_generator.generate_moves(position, &mut move_list);

    for mv in move_list.iterator() {
        let move_legality = position.make_move(mv);

        if move_legality == MoveLegality::Legal {
            nodes += perft(depth - 1, position, move_generator);
        }

        position.take_move();
    }

    nodes
}

#[cfg(test)]
pub mod tests {

    use crate::perft_runner;
    use dolphin_core::board::occupancy_masks::OccupancyMasks;
    use dolphin_core::io::fen;
    use dolphin_core::moves::move_gen::MoveGenerator;
    use dolphin_core::position::attack_checker::AttackChecker;
    use dolphin_core::position::game_position::Position;
    use dolphin_core::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn sample_perft_1() {
        let depth = 5;
        let expected_move_count = 4865609;

        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281 ;D5 4865609 ;D6 119060324

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let mov_generator = MoveGenerator::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let num_moves = perft_runner::perft(depth, &mut pos, &mov_generator);

        assert_eq!(num_moves, expected_move_count);
    }

    #[test]
    pub fn sample_perft_2() {
        let depth = 6;
        let expected_move_count = 158065;

        // 8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1 ;D1 8 ;D2 61 ;D3 411 ;D4 3213 ;D5 21637 ;D6 158065

        let fen = "8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1 ";
        let mov_generator = MoveGenerator::new();
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let num_moves = perft_runner::perft(depth, &mut pos, &mov_generator);

        assert_eq!(num_moves, expected_move_count);
    }

    #[test]
    pub fn sample_perft_3() {
        let depth = 6;
        let expected_move_count = 22823890;

        // B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1 ;D1 17 ;D2 278 ;D3 4607 ;D4 76778 ;D5 1320507 ;D6 22823890

        let fen = "B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1";
        let mov_generator = MoveGenerator::new();
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let num_moves = perft_runner::perft(depth, &mut pos, &mov_generator);

        assert_eq!(num_moves, expected_move_count);
    }
}
