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
    // let epd_rows = perft::epd_parser::extract_epd("../../src/resources/perftsuite.epd".to_string());

    // for depth in 1..6 {
    //     for epd in &epd_rows {
    //         process_row(epd, depth);
    //     }
    // }

        let epd = "r3k3/1K6/8/8/8/8/8/8 w q - 0 1 ;D1 4 ;D2 49 ;D3 243 ;D4 3991 ;D5 20780 ;D6 367724";
        let row = perft::epd_parser::extract_row(epd.to_string());
        process_row(&row, 2);
}

fn process_row(row: &perft::epd_parser::EpdRow, depth: u8) {
    let fen = &row.fen;

    println!("Depth: {}, Testing FEN '{}'", depth, fen);

    let expected_moves = &row.depth_map[&depth];
    let parsed_fen = fen::get_position(&fen);
    let mut position = Position::new(parsed_fen);

    //println!("{}", position.board());

    let num_moves = perft_runner::perft(depth, &mut position);

    if *expected_moves != num_moves {
        println!("**************** problem ***************************")
    }
    println!("Expected: {}, found: {}", expected_moves, num_moves);
}
