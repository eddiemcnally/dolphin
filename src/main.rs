#![allow(dead_code)]

mod board;
mod utils;
mod moves;
mod position;
mod input;

use board::piece::Piece;
use input::fen;

fn main() {
    println!("pawn piece value {}", Piece::WQueen.value());

    let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";

    let piece_pos: Vec<&str> = fen.split(' ').collect();

    let sq_pce = fen::extract_piece_locations(piece_pos[0]);

    for (square, pce) in sq_pce {
        println!("{:?}, {:?}", square, pce);
    }
}
