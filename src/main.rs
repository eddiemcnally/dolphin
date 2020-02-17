#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod board;
mod input;
mod moves;
mod perft;
mod position;
mod utils;
use input::fen;
use perft::perft_runner;
use position::position::Position;

fn main() {
    let epd_rows = perft::epd_parser::extract_epd("../../src/resources/perftsuite.epd".to_string());

    for epd in &epd_rows {
        println!("Testing FEN '{}'", epd.fen);

        for depth in 1..7 {
            process_row(epd, depth);
        }
    }

    // let epd = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ;D1 48 ;D2 2039 ;D3 97862 ;D4 4085603 ;D5 193690690 ;D6 8031647685";
    // let row = perft::epd_parser::extract_row(epd.to_string());
    // process_row(&row, 3);
}

fn process_row(row: &perft::epd_parser::EpdRow, depth: u8) {
    let fen = &row.fen;

    let expected_moves = &row.depth_map[&depth];
    let parsed_fen = fen::get_position(&fen);
    let mut position = Position::new(parsed_fen);

    //println!("{}", position.board());

    let num_moves = perft_runner::perft(depth, &mut position);

    if *expected_moves != num_moves {
        println!(
            "Depth: {}, #Expected: {}, #found: {}",
            depth, expected_moves, num_moves
        );
        panic!("**************** problem ***************************");
    }
    println!(
        "Depth: {}, #Expected: {}, #found: {}",
        depth, expected_moves, num_moves
    );
}
