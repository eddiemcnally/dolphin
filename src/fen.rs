#[allow(dead_code)]

use board;
use piece::Piece;
use square::Rank;
use square::Square;
use square::File;
use std::mem::transmute;


/// parses a FEN string and populates the given board
///
/// Sample FEN:
///      rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
///
pub fn get_position(fen: &str, board: &mut board::Board) {

    let piece_pos: Vec<&str> = fen.split(' ').collect();

    let positions = populate_piece_positions(piece_pos[0]);

    // [0] = piece positions
    // [1] = piece to move
    // [2] = castle permissions
    // [3] = en passant square (or '-' if no en passant)
    // [4] = half-move clock
    // [5] = full move number


}

/// takes the list of ranks (starting at rank 8)
pub fn populate_piece_positions(pieces: &str) -> Vec<(Square, Piece)> {
    let ranks: Vec<_> = pieces.split('/').collect();
    let mut retval: Vec<(Square, Piece)> = Vec::new();
    for (rank, pieces) in ranks.iter().rev().enumerate() {
        let mut file: u8 = 0;

        for c in pieces.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    // it's a number, so incr the file
                    file = file + n as u8;
                }
                None => {
                    // not a number, so it's a piece
                    match Piece::from_char(c) {
                        Some(piece) => {
                            let r: Rank = unsafe { transmute(rank as u8) };
                            let f: File = unsafe { transmute(file as u8) };

                            let mut sq: Square = Square::get_square(r, f);
                            file += 1;
                            retval.push((sq as Square, piece as Piece));
                        }

                        None => panic!("Unexpected FEN piece"),
                    }
                }
            }
        }
    }
    return retval;
}




#[cfg(test)]
mod tests {
    use Square;
    use super::Rank;
    use super::File;

    #[test]
    pub fn test_piece_positions() {

        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";

        let piece_pos: Vec<&str> = fen.split(' ').collect();

        let sq_pce = populate_piece_positions(piece_pos[0]);

        for (square, pce) in sq_pce {
            println!("{:?}, {:?}", square, pce);
        }

        assert_eq!(false);
    }



}
