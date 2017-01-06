use board;
use piece;
use square::Rank;
use square::File;




pub fn construct_position(fen: &str, board: &mut board::Board){

    let mut rank = Rank::Rank8;
    let mut file = File::FileA;
    for c in fen.chars(){
        let mut pce_to_add:piece::Piece;
        let mut count = 0;
        match c{
            'p' => pce_to_add = piece::Piece::BPawn,
            'P' => pce_to_add = piece::Piece::WPawn,
            'r' => pce_to_add = piece::Piece::BRook,
            'R' => pce_to_add = piece::Piece::WRook,
            'b' => pce_to_add = piece::Piece::BBishop,
            'B' => pce_to_add = piece::Piece::WBishop,
            'n' => pce_to_add = piece::Piece::BKnight,
            'N' => pce_to_add = piece::Piece::WKnight,
            'q' => pce_to_add = piece::Piece::BQueen,
            'Q' => pce_to_add = piece::Piece::WQueen,
            'k' => pce_to_add = piece::Piece::BKing,
            'K' => pce_to_add = piece::Piece::BPawn,
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
            _ => panic()
        }




    }



}
