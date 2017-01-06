#![allow(dead_code)]

mod board;
mod piece;
mod bitboard;
mod occupancy_masks;
mod square;

use piece::Piece;
use square::Square;

fn main() {
    println!("pawn piece value {}", Piece::WQueen.value());


    let y: u64 = occupancy_masks::get_occupancy_mask(piece::Piece::WQueen, Square::d1);
    println!("occ mask {}", y);




}
