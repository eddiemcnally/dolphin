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
    let depth = 1;
    let expected_move_count = 2;

    // 6kq/8/8/8/8/8/8/7K w - - 0 1 ;D1 2 ;D2 36 ;D3 143 ;D4 3637 ;D5 14893 ;D6 391507

    let fen = "6kq/8/8/8/8/8/8/7K w - - 0 1 ";
    let parsed_fen = fen::get_position(&fen);
    let mut position = Position::new(parsed_fen);

    let num_moves = perft_runner::perft(depth, &mut position);

    println!(
        "#ExpectedMoves={}, #ActualMoves={}",
        expected_move_count, num_moves
    );
}
