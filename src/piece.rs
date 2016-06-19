#[allow(dead_code)]
pub enum Piece {
    WPawn = 0,
    BPawn,
    WBishop,
    BBishop,
    WKnight,
    BKnight,
    WRook,
    BRook,
    WQueen,
    BQueen,
    WKing,
    BKing,
}

#[allow(dead_code)]
pub const NUM_PIECES: usize = 12;

#[allow(dead_code)]
pub enum Colour {
    White = 0,
    Black,
}
#[allow(dead_code)]
pub const NUM_COLOURS: usize = 2;


#[allow(dead_code)]
pub fn get_value(pce: Piece) -> u32 {
    match pce {
        Piece::WPawn => 300,
        Piece::BPawn => 300,
        Piece::WBishop => 550,
        Piece::BBishop => 550,
        Piece::WKnight => 550,
        Piece::BKnight => 550,
        Piece::WRook => 800,
        Piece::BRook => 800,
        Piece::WQueen => 1000,
        Piece::BQueen => 1000,
        Piece::WKing => 50000,
        Piece::BKing => 50000,
    }
}


#[allow(dead_code)]
pub fn get_label(pce: Piece) -> &'static str {
    match pce {
        Piece::WPawn => "P",
        Piece::BPawn => "p",
        Piece::WBishop => "B",
        Piece::BBishop => "b",
        Piece::WKnight => "N",
        Piece::BKnight => "n",
        Piece::WRook => "R",
        Piece::BRook => "r",
        Piece::WQueen => "Q",
        Piece::BQueen => "q",
        Piece::WKing => "K",
        Piece::BKing => "k",
    }
}
