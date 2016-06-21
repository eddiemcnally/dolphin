mod board;
mod piece;
mod util;
mod occupancy_masks;

fn main() {
#![allow(dead_code)]


    println!("pawn piece value {}",
             piece::get_value(piece::Piece::WQueen));


    let x: u64 = util::set_bit(0, 5);
    println!("x = {}", x);

    let y:u64 = occupancy_masks::get_occupancy_mask(piece::Piece::WQueen, board::Square::d1);
    println!("occ mask {}", y);


}
