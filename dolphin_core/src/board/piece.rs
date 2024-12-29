use crate::board::colour::Colour;
use crate::moves::mov::Score;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub enum Piece {
    #[default]
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const NUM_PIECE_TYPES: usize = 6;

    #[inline(always)]
    pub const fn as_index(&self) -> usize {
        *self as usize
    }

    pub const fn value(&self) -> Score {
        match self {
            Piece::Pawn => PieceValue::Pawn as Score,
            Piece::Bishop => PieceValue::Bishop as Score,
            Piece::Knight => PieceValue::Knight as Score,
            Piece::Rook => PieceValue::Rook as Score,
            Piece::Queen => PieceValue::Queen as Score,
            Piece::King => PieceValue::King as Score,
        }
    }

    pub fn from_char(piece_char: char) -> Option<(Piece, Colour)> {
        match piece_char {
            'P' => Some((Piece::Pawn, Colour::White)),
            'B' => Some((Piece::Bishop, Colour::White)),
            'N' => Some((Piece::Knight, Colour::White)),
            'R' => Some((Piece::Rook, Colour::White)),
            'Q' => Some((Piece::Queen, Colour::White)),
            'K' => Some((Piece::King, Colour::White)),

            'p' => Some((Piece::Pawn, Colour::Black)),
            'b' => Some((Piece::Bishop, Colour::Black)),
            'n' => Some((Piece::Knight, Colour::Black)),
            'r' => Some((Piece::Rook, Colour::Black)),
            'q' => Some((Piece::Queen, Colour::Black)),
            'k' => Some((Piece::King, Colour::Black)),
            _ => None,
        }
    }

    pub fn label(piece: &Piece, colour: &Colour) -> char {
        let c = match piece {
            Piece::Pawn => 'P',
            Piece::Bishop => 'B',
            Piece::Knight => 'N',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        };

        match colour {
            Colour::White => c,
            Colour::Black => c.to_ascii_lowercase(),
        }
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

#[cfg(test)]
pub mod tests {
    use crate::{
        board::{
            colour::Colour,
            piece::{Piece, PieceValue},
        },
        moves::mov::Score,
    };

    #[test]
    pub fn piece_values_as_expected() {
        assert_eq!(Piece::Pawn.value(), PieceValue::Pawn as Score);
        assert_eq!(Piece::Bishop.value(), PieceValue::Bishop as Score);
        assert_eq!(Piece::Knight.value(), PieceValue::Knight as Score);
        assert_eq!(Piece::Rook.value(), PieceValue::Rook as Score);
        assert_eq!(Piece::Queen.value(), PieceValue::Queen as Score);
        assert_eq!(Piece::King.value(), PieceValue::King as Score);
    }

    #[test]
    pub fn from_char() {
        // white
        assert_eq!(Piece::from_char('P'), Some((Piece::Pawn, Colour::White)));
        assert_eq!(Piece::from_char('B'), Some((Piece::Bishop, Colour::White)));
        assert_eq!(Piece::from_char('N'), Some((Piece::Knight, Colour::White)));
        assert_eq!(Piece::from_char('R'), Some((Piece::Rook, Colour::White)));
        assert_eq!(Piece::from_char('Q'), Some((Piece::Queen, Colour::White)));
        assert_eq!(Piece::from_char('K'), Some((Piece::King, Colour::White)));

        // black
        assert_eq!(Piece::from_char('p'), Some((Piece::Pawn, Colour::Black)));
        assert_eq!(Piece::from_char('b'), Some((Piece::Bishop, Colour::Black)));
        assert_eq!(Piece::from_char('n'), Some((Piece::Knight, Colour::Black)));
        assert_eq!(Piece::from_char('r'), Some((Piece::Rook, Colour::Black)));
        assert_eq!(Piece::from_char('q'), Some((Piece::Queen, Colour::Black)));
        assert_eq!(Piece::from_char('k'), Some((Piece::King, Colour::Black)));
    }

    #[test]
    pub fn label() {
        // white
        assert_eq!(Piece::label(&Piece::Pawn, &Colour::White), 'P');
        assert_eq!(Piece::label(&Piece::Bishop, &Colour::White), 'B');
        assert_eq!(Piece::label(&Piece::Knight, &Colour::White), 'N');
        assert_eq!(Piece::label(&Piece::Rook, &Colour::White), 'R');
        assert_eq!(Piece::label(&Piece::Queen, &Colour::White), 'Q');
        assert_eq!(Piece::label(&Piece::King, &Colour::White), 'K');

        // black
        assert_eq!(Piece::label(&Piece::Pawn, &Colour::Black), 'p');
        assert_eq!(Piece::label(&Piece::Bishop, &Colour::Black), 'b');
        assert_eq!(Piece::label(&Piece::Knight, &Colour::Black), 'n');
        assert_eq!(Piece::label(&Piece::Rook, &Colour::Black), 'r');
        assert_eq!(Piece::label(&Piece::Queen, &Colour::Black), 'q');
        assert_eq!(Piece::label(&Piece::King, &Colour::Black), 'k');
    }
}
