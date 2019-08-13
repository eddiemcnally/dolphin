#![allow(dead_code)]
#![feature(reverse_bits)]


mod board;
mod input;
mod moves;
mod position;
mod utils;
use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use input::fen;

fn main() {
    let pce = Piece::new(PieceRole::Queen, Colour::White);
    println!("pawn piece value {}", pce.value());

    let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";

    let piece_pos: Vec<&str> = fen.split(' ').collect();

    let sq_pce = fen::extract_piece_locations(piece_pos[0]);

    for (square, pce) in sq_pce {
        println!("{:?}, {:?}", square, pce);
    }
}
