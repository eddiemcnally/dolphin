use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Colour {
    White,
    Black,
}

// Array offsets for each piece
#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackQueen,
    BlackKing,
}

pub const NUM_PIECES: usize = 12;
pub const NUM_COLOURS: usize = 2;

pub static PIECES: &[Piece] = &[
    Piece::WhitePawn,
    Piece::WhiteBishop,
    Piece::WhiteKnight,
    Piece::WhiteRook,
    Piece::WhiteQueen,
    Piece::WhiteKing,
    Piece::BlackPawn,
    Piece::BlackBishop,
    Piece::BlackKnight,
    Piece::BlackRook,
    Piece::BlackQueen,
    Piece::BlackKing,
];

// piece values from here:
// https://www.chessprogramming.org/Simplified_Evaluation_Function
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum PieceValue {
    Pawn = 100,
    Knight = 320,
    Bishop = 330,
    Rook = 500,
    Queen = 900,
    King = 20000,
}

impl Piece {
    pub const fn to_offset(&self) -> usize {
        return *self as usize;
    }

    pub fn from_offset(offset: u8) -> Piece {
        let pce = Piece::try_from(offset);
        match pce {
            Ok(pce) => return pce,
            _ => panic!("Invalid piece offset {}.", offset),
        }
    }

    pub fn is_king(self) -> bool {
        self == Piece::WhiteKing || self == Piece::BlackKing
    }
    pub fn is_rook(self) -> bool {
        self == Piece::WhiteRook || self == Piece::BlackRook
    }
    pub fn is_pawn(self) -> bool {
        self == Piece::WhitePawn || self == Piece::BlackPawn
    }

    pub const fn colour(self) -> Colour {
        match self {
            Piece::WhitePawn
            | Piece::WhiteBishop
            | Piece::WhiteKnight
            | Piece::WhiteRook
            | Piece::WhiteQueen
            | Piece::WhiteKing => Colour::White,
            Piece::BlackPawn
            | Piece::BlackBishop
            | Piece::BlackKnight
            | Piece::BlackRook
            | Piece::BlackQueen
            | Piece::BlackKing => Colour::Black,
        }
    }

    pub fn from_char(piece_char: char) -> Piece {
        match piece_char {
            'P' => Piece::WhitePawn,
            'B' => Piece::WhiteBishop,
            'N' => Piece::WhiteKnight,
            'R' => Piece::WhiteRook,
            'Q' => Piece::WhiteQueen,
            'K' => Piece::WhiteKing,
            'p' => Piece::BlackPawn,
            'b' => Piece::BlackBishop,
            'n' => Piece::BlackKnight,
            'r' => Piece::BlackRook,
            'q' => Piece::BlackQueen,
            'k' => Piece::BlackKing,
            _ => panic!("Invalid piece character {}.", piece_char),
        }
    }

    pub const fn value(self) -> u32 {
        match self {
            Piece::WhitePawn => PieceValue::Pawn as u32,
            Piece::WhiteBishop => PieceValue::Bishop as u32,
            Piece::WhiteKnight => PieceValue::Knight as u32,
            Piece::WhiteRook => PieceValue::Rook as u32,
            Piece::WhiteQueen => PieceValue::Queen as u32,
            Piece::WhiteKing => PieceValue::King as u32,
            Piece::BlackPawn => PieceValue::Pawn as u32,
            Piece::BlackBishop => PieceValue::Bishop as u32,
            Piece::BlackKnight => PieceValue::Knight as u32,
            Piece::BlackRook => PieceValue::Rook as u32,
            Piece::BlackQueen => PieceValue::Queen as u32,
            Piece::BlackKing => PieceValue::King as u32,
        }
    }

    pub fn to_label(self) -> char {
        match self {
            Piece::WhitePawn => 'P',
            Piece::WhiteBishop => 'B',
            Piece::WhiteKnight => 'N',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',
            Piece::BlackPawn => 'p',
            Piece::BlackBishop => 'b',
            Piece::BlackKnight => 'n',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        let label = self.to_label();
        debug_str.push_str(&format!("{:?}", label));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Colour {
    pub const fn flip_side(self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
    pub const fn to_offset(self) -> usize {
        self as usize
    }
    pub fn is_white(self) -> bool {
        self == Colour::White
    }
    pub fn is_black(self) -> bool {
        self == Colour::Black
    }
}

impl Default for Colour {
    fn default() -> Colour {
        Colour::White
    }
}

impl fmt::Debug for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::White => write!(f, "White"),
            Colour::Black => write!(f, "Black"),
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

pub fn get_all_pieces() -> Vec<Piece> {
    let mut list: Vec<Piece> = Vec::new();

    for p in PIECES {
        list.push(*p);
    }

    list
}

#[cfg(test)]
pub mod tests {

    use super::Colour;
    use super::Piece;

    #[test]
    pub fn flip_side_as_expected() {
        let c = Colour::default();
        assert!(c == Colour::White);

        let f = c.flip_side();
        assert!(f == Colour::Black);

        let o = f.flip_side();
        assert!(o == Colour::White);
    }

    #[test]
    pub fn default_colour() {
        let c = Colour::default();
        assert!(c == Colour::White);
    }

    #[test]
    pub fn piece_colour_flipped() {
        let pce = Piece::WhiteKnight;
        assert!(pce.colour().flip_side() == Colour::Black);
    }

    #[test]
    pub fn colour_as_expected() {
        assert_eq!(Colour::Black, Piece::BlackBishop.colour());
        assert_eq!(Colour::Black, Piece::BlackKing.colour());
        assert_eq!(Colour::Black, Piece::BlackKnight.colour());
        assert_eq!(Colour::Black, Piece::BlackPawn.colour());
        assert_eq!(Colour::Black, Piece::BlackQueen.colour());
        assert_eq!(Colour::Black, Piece::BlackKing.colour());

        assert_eq!(Colour::White, Piece::WhiteBishop.colour());
        assert_eq!(Colour::White, Piece::WhiteKing.colour());
        assert_eq!(Colour::White, Piece::WhiteKnight.colour());
        assert_eq!(Colour::White, Piece::WhitePawn.colour());
        assert_eq!(Colour::White, Piece::WhiteQueen.colour());
        assert_eq!(Colour::White, Piece::WhiteRook.colour());
    }

    #[test]
    pub fn offset_as_expected() {
        assert_eq!(Piece::WhitePawn.to_offset(), 0);
        assert_eq!(Piece::WhiteBishop.to_offset(), 1);
        assert_eq!(Piece::WhiteKnight.to_offset(), 2);
        assert_eq!(Piece::WhiteRook.to_offset(), 3);
        assert_eq!(Piece::WhiteQueen.to_offset(), 4);
        assert_eq!(Piece::WhiteKing.to_offset(), 5);

        assert_eq!(Piece::BlackPawn.to_offset(), 6);
        assert_eq!(Piece::BlackBishop.to_offset(), 7);
        assert_eq!(Piece::BlackKnight.to_offset(), 8);
        assert_eq!(Piece::BlackRook.to_offset(), 9);
        assert_eq!(Piece::BlackQueen.to_offset(), 10);
        assert_eq!(Piece::BlackKing.to_offset(), 11);
    }
}
