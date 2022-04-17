use crate::board::colour::Colour;
use crate::core::types::ToInt;
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
    pub const NUM_PIECES: usize = 32;
    pub const NUM_PIECE_TYPES: usize = 6;

    const VALUES: [u32; Piece::NUM_PIECE_TYPES] = [
        PieceValue::Pawn as u32,
        PieceValue::Bishop as u32,
        PieceValue::Knight as u32,
        PieceValue::Rook as u32,
        PieceValue::Queen as u32,
        PieceValue::King as u32,
    ];

    pub fn value(&self) -> u32 {
        Piece::VALUES[self.to_usize()]
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

    pub fn iterator() -> Iter<'static, Piece> {
        static PIECES: [Piece; Piece::NUM_PIECE_TYPES] = [
            Piece::Pawn,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];
        PIECES.iter()
    }
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

        debug_str.push_str(st);

        write!(f, "{}", debug_str)
    }
}

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

#[cfg(test)]
pub mod tests {
    use crate::board::{
        colour::Colour,
        piece::{Piece, PieceValue},
    };

    #[test]
    pub fn piece_values_as_expected() {
        assert_eq!(Piece::Pawn.value(), PieceValue::Pawn as u32);
        assert_eq!(Piece::Bishop.value(), PieceValue::Bishop as u32);
        assert_eq!(Piece::Knight.value(), PieceValue::Knight as u32);
        assert_eq!(Piece::Rook.value(), PieceValue::Rook as u32);
        assert_eq!(Piece::Queen.value(), PieceValue::Queen as u32);
        assert_eq!(Piece::King.value(), PieceValue::King as u32);
    }

    #[test]
    pub fn from_char() {
        // white
        assert_eq!(Piece::from_char('P'), (Piece::Pawn, Colour::White));
        assert_eq!(Piece::from_char('B'), (Piece::Bishop, Colour::White));
        assert_eq!(Piece::from_char('N'), (Piece::Knight, Colour::White));
        assert_eq!(Piece::from_char('R'), (Piece::Rook, Colour::White));
        assert_eq!(Piece::from_char('Q'), (Piece::Queen, Colour::White));
        assert_eq!(Piece::from_char('K'), (Piece::King, Colour::White));

        // black
        assert_eq!(Piece::from_char('p'), (Piece::Pawn, Colour::Black));
        assert_eq!(Piece::from_char('b'), (Piece::Bishop, Colour::Black));
        assert_eq!(Piece::from_char('n'), (Piece::Knight, Colour::Black));
        assert_eq!(Piece::from_char('r'), (Piece::Rook, Colour::Black));
        assert_eq!(Piece::from_char('q'), (Piece::Queen, Colour::Black));
        assert_eq!(Piece::from_char('k'), (Piece::King, Colour::Black));
    }

    #[test]
    pub fn label() {
        // white
        assert_eq!(Piece::label(Piece::Pawn, Colour::White), 'P');
        assert_eq!(Piece::label(Piece::Bishop, Colour::White), 'B');
        assert_eq!(Piece::label(Piece::Knight, Colour::White), 'N');
        assert_eq!(Piece::label(Piece::Rook, Colour::White), 'R');
        assert_eq!(Piece::label(Piece::Queen, Colour::White), 'Q');
        assert_eq!(Piece::label(Piece::King, Colour::White), 'K');

        // black
        assert_eq!(Piece::label(Piece::Pawn, Colour::Black), 'p');
        assert_eq!(Piece::label(Piece::Bishop, Colour::Black), 'b');
        assert_eq!(Piece::label(Piece::Knight, Colour::Black), 'n');
        assert_eq!(Piece::label(Piece::Rook, Colour::Black), 'r');
        assert_eq!(Piece::label(Piece::Queen, Colour::Black), 'q');
        assert_eq!(Piece::label(Piece::King, Colour::Black), 'k');
    }
}
