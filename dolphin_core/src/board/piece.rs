use super::{colour::Colour, types::ToInt};
use std::{fmt, slice::Iter};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl ToInt for Piece {
    fn to_u8(&self) -> u8 {
        *self as u8
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl Piece {
    pub const fn value(&self) -> u32 {
        match self {
            Piece::Pawn => PieceValue::Pawn as u32,
            Piece::Bishop => PieceValue::Bishop as u32,
            Piece::Knight => PieceValue::Knight as u32,
            Piece::Rook => PieceValue::Rook as u32,
            Piece::Queen => PieceValue::Queen as u32,
            Piece::King => PieceValue::King as u32,
        }
    }
    pub fn is_king(&self) -> bool {
        *self == Piece::King
    }
    pub fn is_pawn(&self) -> bool {
        *self == Piece::Pawn
    }
    pub fn is_rook(&self) -> bool {
        *self == Piece::Rook
    }

    pub fn from_char(piece_char: char) -> (Piece, Colour) {
        match piece_char {
            'P' => (Piece::Pawn, Colour::White),
            'B' => (Piece::Bishop, Colour::White),
            'N' => (Piece::Knight, Colour::White),
            'R' => (Piece::Rook, Colour::White),
            'Q' => (Piece::Queen, Colour::White),
            'K' => (Piece::King, Colour::White),
            'p' => (Piece::Pawn, Colour::Black),
            'b' => (Piece::Bishop, Colour::Black),
            'n' => (Piece::Knight, Colour::Black),
            'r' => (Piece::Rook, Colour::Black),
            'q' => (Piece::Queen, Colour::Black),
            'k' => (Piece::King, Colour::Black),

            _ => panic!("Invalid piece character {}.", piece_char),
        }
    }
}

pub fn iterator() -> Iter<'static, Piece> {
    static PIECES: [Piece; NUM_PIECE_TYPES] = [
        Piece::Pawn,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
    PIECES.iter()
}

pub fn label(piece: Piece, colour: Colour) -> char {
    let c = match piece {
        Piece::Pawn => 'P',
        Piece::Bishop => 'B',
        Piece::Knight => 'N',
        Piece::Rook => 'R',
        Piece::Queen => 'Q',
        Piece::King => 'K',
    };

    if colour == Colour::White {
        return c;
    }
    c.to_ascii_lowercase()
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        let st = match self {
            Piece::Pawn => "Pawn",
            Piece::Bishop => "Bishop",
            Piece::Knight => "Knight",
            Piece::Rook => "Rook",
            Piece::Queen => "Queen",
            Piece::King => "King",
        };

        debug_str.push_str(&st.to_string());

        write!(f, "{}", debug_str)
    }
}

pub const NUM_PIECES: usize = 32;
pub const NUM_PIECE_TYPES: usize = 6;

// piece values from here:
// https://www.chessprogramming.org/Simplified_Evaluation_Function
#[rustfmt::skip]
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum PieceValue {
    Pawn    = 100,
    Knight  = 320,
    Bishop  = 330,
    Rook    = 500,
    Queen   = 900,
    King    = 20000,
}

impl Default for Piece {
    fn default() -> Piece {
        Piece::Pawn
    }
}
