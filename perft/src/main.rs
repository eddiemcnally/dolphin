extern crate core_affinity;
extern crate dolphin_core;
extern crate num_enum;

use dolphin_core::fen;
use dolphin_core::occupancy_masks::OccupancyMasks;
use dolphin_core::piece::Piece;
use dolphin_core::position::Position;
use dolphin_core::zobrist_keys::ZobristKeys;
use std::time::Instant;

mod epd_parser;
mod perft_runner;

fn main() {
    let x = Piece::BlackKing;

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
    let parsed_fen = fen::get_position(&fen);
    let zobrist_keys = ZobristKeys::new();
    let occ_masks = OccupancyMasks::new();

    let mut position = Position::new(&zobrist_keys, &occ_masks, parsed_fen);

    let now = Instant::now();
    let num_moves = perft_runner::perft(depth, &mut position);
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
