use std::fmt;

use super::types::ToInt;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Colour {
    White,
    Black,
}

pub const NUM_COLOURS: usize = 2;

impl ToInt for Colour {
    fn to_u8(&self) -> u8 {
        *self as u8
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl Colour {
    pub const fn flip_side(self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }

    pub fn is_white(self) -> bool {
        self == Colour::White
    }
    pub fn is_black(self) -> bool {
        self == Colour::Black
    }
}

pub const fn offset(colour: Colour) -> usize {
    colour as usize
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

#[cfg(test)]
pub mod tests {
    use crate::board::colour::Colour;

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
    pub fn colour_flipped() {
        let white_col = Colour::White;
        assert!(white_col.flip_side() == Colour::Black);

        let black_col = Colour::Black;
        assert!(black_col.flip_side() == Colour::White);
    }

    #[test]
    pub fn colour_is_correct() {
        let black_col = Colour::Black;
        assert!(black_col.is_black());

        let white_col = Colour::White;
        assert!(white_col.is_white());
    }
}
