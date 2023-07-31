use crate::board::colour::Colour;
use crate::moves::mov::Score;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
#[repr(u8)]
pub enum Role {
    #[default]
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl Role {
    pub const fn as_index(&self) -> usize {
        *self as usize
    }
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Debug for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        let st = match self {
            Role::Pawn => "Pawn",
            Role::Bishop => "Bishop",
            Role::Knight => "Knight",
            Role::Rook => "Rook",
            Role::Queen => "Queen",
            Role::King => "King",
        };

        debug_str.push_str(st);

        write!(f, "{}", debug_str)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct Piece {
    role: Role,
    colour: Colour,
}

pub const WHITE_PAWN: Piece = Piece::new(Role::Pawn, Colour::White);
pub const WHITE_BISHOP: Piece = Piece::new(Role::Bishop, Colour::White);
pub const WHITE_KNIGHT: Piece = Piece::new(Role::Knight, Colour::White);
pub const WHITE_ROOK: Piece = Piece::new(Role::Rook, Colour::White);
pub const WHITE_QUEEN: Piece = Piece::new(Role::Queen, Colour::White);
pub const WHITE_KING: Piece = Piece::new(Role::King, Colour::White);

pub const BLACK_PAWN: Piece = Piece::new(Role::Pawn, Colour::Black);
pub const BLACK_BISHOP: Piece = Piece::new(Role::Bishop, Colour::Black);
pub const BLACK_KNIGHT: Piece = Piece::new(Role::Knight, Colour::Black);
pub const BLACK_ROOK: Piece = Piece::new(Role::Rook, Colour::Black);
pub const BLACK_QUEEN: Piece = Piece::new(Role::Queen, Colour::Black);
pub const BLACK_KING: Piece = Piece::new(Role::King, Colour::Black);

impl Piece {
    pub const NUM_PIECES: usize = 32;
    pub const NUM_PIECE_TYPES: usize = 6;

    pub const fn new(role: Role, colour: Colour) -> Piece {
        Piece { role, colour }
    }

    pub const fn role(&self) -> Role {
        self.role
    }

    pub const fn colour(&self) -> Colour {
        self.colour
    }

    pub fn value(self) -> Score {
        match self.role {
            Role::Pawn => PieceValue::Pawn as Score,
            Role::Bishop => PieceValue::Bishop as Score,
            Role::Knight => PieceValue::Knight as Score,
            Role::Rook => PieceValue::Rook as Score,
            Role::Queen => PieceValue::Queen as Score,
            Role::King => PieceValue::King as Score,
        }
    }

    pub fn from_char(piece_char: char) -> Piece {
        match piece_char {
            'P' => WHITE_PAWN,
            'B' => WHITE_BISHOP,
            'N' => WHITE_KNIGHT,
            'R' => WHITE_ROOK,
            'Q' => WHITE_QUEEN,
            'K' => WHITE_KING,
            'p' => BLACK_PAWN,
            'b' => BLACK_BISHOP,
            'n' => BLACK_KNIGHT,
            'r' => BLACK_ROOK,
            'q' => BLACK_QUEEN,
            'k' => BLACK_KING,

            _ => panic!("Invalid piece character {}.", piece_char),
        }
    }

    pub fn label(piece: Piece) -> char {
        let c = match piece.role {
            Role::Pawn => 'P',
            Role::Bishop => 'B',
            Role::Knight => 'N',
            Role::Rook => 'R',
            Role::Queen => 'Q',
            Role::King => 'K',
        };

        if piece.colour == Colour::White {
            return c;
        }
        c.to_ascii_lowercase()
    }

    pub fn iterator() -> Iter<'static, Piece> {
        #[rustfmt::skip]
        static PIECES : [Piece; Piece::NUM_PIECE_TYPES * Colour::NUM_COLOURS] = [
            WHITE_PAWN, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK, WHITE_QUEEN, WHITE_KING,
            BLACK_PAWN, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK, BLACK_QUEEN, BLACK_KING];
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

        let st = match self.role {
            Role::Pawn => "Pawn",
            Role::Bishop => "Bishop",
            Role::Knight => "Knight",
            Role::Rook => "Rook",
            Role::Queen => "Queen",
            Role::King => "King",
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
    use crate::board::piece::{
        Piece, PieceValue, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN,
        BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    };
    use crate::moves::mov::Score;

    #[test]
    pub fn piece_values_as_expected() {
        assert_eq!(WHITE_PAWN.value(), PieceValue::Pawn as Score);
        assert_eq!(WHITE_BISHOP.value(), PieceValue::Bishop as Score);
        assert_eq!(WHITE_KNIGHT.value(), PieceValue::Knight as Score);
        assert_eq!(WHITE_ROOK.value(), PieceValue::Rook as Score);
        assert_eq!(WHITE_QUEEN.value(), PieceValue::Queen as Score);
        assert_eq!(WHITE_KING.value(), PieceValue::King as Score);

        assert_eq!(BLACK_PAWN.value(), PieceValue::Pawn as Score);
        assert_eq!(BLACK_BISHOP.value(), PieceValue::Bishop as Score);
        assert_eq!(BLACK_KNIGHT.value(), PieceValue::Knight as Score);
        assert_eq!(BLACK_ROOK.value(), PieceValue::Rook as Score);
        assert_eq!(BLACK_QUEEN.value(), PieceValue::Queen as Score);
        assert_eq!(BLACK_KING.value(), PieceValue::King as Score);
    }

    #[test]
    pub fn from_char() {
        // white
        assert_eq!(Piece::from_char('P'), WHITE_PAWN);
        assert_eq!(Piece::from_char('B'), WHITE_BISHOP);
        assert_eq!(Piece::from_char('N'), WHITE_KNIGHT);
        assert_eq!(Piece::from_char('R'), WHITE_ROOK);
        assert_eq!(Piece::from_char('Q'), WHITE_QUEEN);
        assert_eq!(Piece::from_char('K'), WHITE_KING);

        // black
        assert_eq!(Piece::from_char('p'), BLACK_PAWN);
        assert_eq!(Piece::from_char('b'), BLACK_BISHOP);
        assert_eq!(Piece::from_char('n'), BLACK_KNIGHT);
        assert_eq!(Piece::from_char('r'), BLACK_ROOK);
        assert_eq!(Piece::from_char('q'), BLACK_QUEEN);
        assert_eq!(Piece::from_char('k'), BLACK_KING);
    }

    #[test]
    pub fn label() {
        // white
        assert_eq!(Piece::label(WHITE_PAWN), 'P');
        assert_eq!(Piece::label(WHITE_BISHOP), 'B');
        assert_eq!(Piece::label(WHITE_KNIGHT), 'N');
        assert_eq!(Piece::label(WHITE_ROOK), 'R');
        assert_eq!(Piece::label(WHITE_QUEEN), 'Q');
        assert_eq!(Piece::label(WHITE_KING), 'K');

        // black

        assert_eq!(Piece::label(BLACK_PAWN), 'p');
        assert_eq!(Piece::label(BLACK_BISHOP), 'b');
        assert_eq!(Piece::label(BLACK_KNIGHT), 'n');
        assert_eq!(Piece::label(BLACK_ROOK), 'r');
        assert_eq!(Piece::label(BLACK_QUEEN), 'q');
        assert_eq!(Piece::label(BLACK_KING), 'k');
    }
}
