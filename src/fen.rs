use board;
use piece;
use square::Rank;
use square::File;


/// parses a FEN string and populates the given board
///
/// Sample FEN:
///      rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
///
pub fn get_position(fen: &str, board: &mut board::Board){
    let v: Vec<&str> = fen.split(' ').collect();

    // [0] = piece positions
    // [1] = piece to move
    // [2] = castle permissions
    // [3] = en passant square (or '-' if no en passant)
    // [4] = half-move clock
    // [5] = full move number






}

/// takes the list of ranks (starting at rank 8)
populate_piece_positions(rank_list Vec<&str>, board: &mut board::Board){

    let ranks = vec![Rank::Rank8, Rank::Rank7, Rank::Rank6, Rank::Rank5, Rank::Rank4, Rank::Rank3, Rank::Rank2, Rank::Rank1];




}




pub fn aaa__construct_position(fen: &str, board: &mut board::Board){

    let mut rank = Rank::Rank8;
    let mut file = File::FileA;
    for c in fen.chars(){
        let mut pce_to_add:piece::Piece;
        let mut count = 0;
        let pce = try_get_piece(&c);
        match c{
            Some(),
            None(){
                '1' => count => 1,
                '2' => count => 2,
                '3' => count => 3,
                '4' => count => 4,
                '5' => count => 5,
                '6' => count => 6,
                '7' => count => 7,
                '8' => count => 8,
                '/' | ' ' =>{
                    rank = rank.decr();
                    file = File::FileA;
                },
            }
            _ => panic()
        }
    }

    fn gtry_get_piece(fen_char &str) -> Option<piece::Piece>{
        match *fen_char {
            'p' => pce_to_add = Some(piece::Piece::BPawn),
            'P' => pce_to_add = Some(piece::Piece::WPawn),
            'r' => pce_to_add = Some(piece::Piece::BRook),
            'R' => pce_to_add = Some(piece::Piece::WRook),
            'b' => pce_to_add = Some(piece::Piece::BBishop),
            'B' => pce_to_add = Some(piece::Piece::WBishop),
            'n' => pce_to_add = Some(piece::Piece::BKnight),
            'N' => pce_to_add = Some(piece::Piece::WKnight),
            'q' => pce_to_add = Some(piece::Piece::BQueen),
            'Q' => pce_to_add = Some(piece::Piece::WQueen),
            'k' => pce_to_add = Some(piece::Piece::BKing),
            'K' => pce_to_add = Some(piece::Piece::BPawn),
            _ => None
        }
    }
}
