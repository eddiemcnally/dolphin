#[allow(dead_code)]
#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
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

impl Piece {
    pub fn value(self) -> u32 {
        match self {
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

    pub fn from_char(piece_char: char) -> Option<Piece> {
        let pce = match piece_char {
            'P' => Piece::WPawn,
            'B' => Piece::WBishop,
            'N' => Piece::WKnight,
            'R' => Piece::WRook,
            'Q' => Piece::WQueen,
            'K' => Piece::WKing,
            'p' => Piece::BPawn,
            'b' => Piece::BBishop,
            'n' => Piece::BKnight,
            'r' => Piece::BRook,
            'q' => Piece::BQueen,
            'k' => Piece::BKing,
            _ => return None,
        };
        Some(pce)
    }

    pub fn label(self) -> &'static str {
        match self {
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

    pub fn colour(self) -> Colour {
        match self {
            Piece::WPawn => Colour::White,
            Piece::BPawn => Colour::Black,
            Piece::WBishop => Colour::White,
            Piece::BBishop => Colour::Black,
            Piece::WKnight => Colour::White,
            Piece::BKnight => Colour::Black,
            Piece::WRook => Colour::White,
            Piece::BRook => Colour::Black,
            Piece::WQueen => Colour::White,
            Piece::BQueen => Colour::Black,
            Piece::WKing => Colour::White,
            Piece::BKing => Colour::Black,
        }
    }
}


#[allow(dead_code)]
pub const NUM_PIECES: usize = 12;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Colour {
    White = 0,
    Black,
}
impl Default for Colour {
    fn default() -> Colour {
        Colour::White
    }
}



#[allow(dead_code)]
pub const NUM_COLOURS: usize = 2;


#[cfg(test)]
mod tests {
    use super::Piece;
    use super::Colour;

    #[test]
    pub fn test_piece_value() {
        let mut pce = Piece::BBishop;
        assert_eq!(pce.value(), 550);

        pce = Piece::WRook;
        assert_eq!(pce.value(), 800);
    }

    #[test]
    pub fn test_piece_label() {
        let mut pce = Piece::BBishop;
        assert_eq!(pce.label(), "b");

        pce = Piece::WRook;
        assert_eq!(pce.label(), "R");
    }

    #[test]
    pub fn test_piece_colour() {
        let mut pce = Piece::BBishop;
        assert_eq!(pce.colour(), Colour::Black);

        pce = Piece::WRook;
        assert_eq!(pce.colour(), Colour::White);
    }




}
