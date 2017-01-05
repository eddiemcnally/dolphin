#![allow(dead_code)]

mod board;
mod piece;
mod util;
mod bitboard;
mod occupancy_masks;
mod square;

use piece::Piece;
use square::Square;

fn main() {
    println!("pawn piece value {}", Piece::WQueen.value());


    let x: u64 = util::set_bit(0, 5);
    println!("x = {}", x);

    let y: u64 = occupancy_masks::get_occupancy_mask(piece::Piece::WQueen, Square::d1);
    println!("occ mask {}", y);




}
