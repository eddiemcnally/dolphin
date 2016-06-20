mod board;
mod piece;

fn main() {
#![allow(dead_code)]


    println!("pawn piece value {}",
             piece::get_value(piece::Piece::WQueen));


}
