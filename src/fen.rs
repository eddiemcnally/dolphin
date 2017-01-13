use board;
use piece;
use square::Rank;
use square::File;


/// parses a FEN string and populates the given board
///
/// Sample FEN:
///      rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
///
pub fn get_position(fen: &str, board: &mut board::Board) {
    let v: Vec<&str> = fen.split(' ').collect();

    let positions = populate_piece_positions(v[0]);

    // [0] = piece positions
    // [1] = piece to move
    // [2] = castle permissions
    // [3] = en passant square (or '-' if no en passant)
    // [4] = half-move clock
    // [5] = full move number


}

/// takes the list of ranks (starting at rank 8)
fn populate_piece_positions(pieces: &str) -> Vec<(Square, Piece)> {
    let ranks: Vec<_> = placement_str.split('/').collect();
    let mut retval: Vec<(Square, Piece)>;
    for (rank, pieces) in ranks.iter().rev().enumerate() {
        let mut file = 0;

        for c in pieces.chars() {
            match piece_char.to_digit(10) {
                Some(n) => {
                    // it's a number, so incr the file
                    file = file + n;
                }
                None => {
                    // not a number, so it's a piece
                    match Piece::from_char(piece_char) {
                        Some(piece) => {
                            let mut sq = rank * 8 + file;
                            file += 1;
                            retval.append(sq, piece);
                        }

                        None => panic_abort!(),
                    }
                }
            }
        }
    }
    return retval;
}
