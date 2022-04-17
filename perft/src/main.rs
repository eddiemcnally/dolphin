extern crate core_affinity;
extern crate dolphin_core;

use dolphin_core::board::occupancy_masks::OccupancyMasks;
use dolphin_core::board::piece::Piece;
use dolphin_core::io::fen;
use dolphin_core::moves::move_gen::MoveGenerator;
use dolphin_core::position::attack_checker::AttackChecker;
use dolphin_core::position::game_position::Position;
use dolphin_core::position::zobrist_keys::ZobristKeys;
use std::time::Instant;

mod epd_parser;
mod perft_runner;

fn main() {
    let x = Piece::King;

    print!("val {}", x);
    // Pin current thread to a core
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[0]);

    let epd_rows = epd_parser::extract_epd(
        "/Users/eddiemcnally/dev/rust/dolphin/perft/resources/perftsuite.epd".to_string(),
    );

    for epd in &epd_rows {
        println!("Testing FEN '{}'", epd.fen);

        for depth in 1..7 {
            process_row(epd, depth);
        }
    }
}

fn process_row(row: &epd_parser::EpdRow, depth: u8) {
    let fen = &row.fen;

    let expected_moves = &row.depth_map[&depth];
    let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) = fen::decompose_fen(fen);

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
    let mov_generator = MoveGenerator::new();

    let now = Instant::now();
    let num_moves = perft_runner::perft(depth, &mut pos, &mov_generator);
    let elapsed_in_secs = now.elapsed().as_secs_f64();
    let nodes_per_sec = (num_moves as f64 / elapsed_in_secs) as u64;

    if *expected_moves != num_moves {
        println!(
            "Depth: {}, #Expected: {}, #found: {}",
            depth, expected_moves, num_moves
        );
        panic!("**************** problem ***************************");
    }
    println!(
        "#Nodes/Sec: {}, Depth: {}, #Expected: {}, #found: {}",
        nodes_per_sec, depth, expected_moves, num_moves
    );
}
