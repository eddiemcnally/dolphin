use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub const NUM_COLOURS: usize = 2;

    pub const fn to_offset(self) -> usize {
        self as usize
    }

    pub const fn flip_side(self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }

    pub fn iterator() -> Iter<'static, Colour> {
        static COLOURS: [Colour; Colour::NUM_COLOURS] = [Colour::White, Colour::Black];
        COLOURS.iter()
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
        let white = Colour::default();
        assert!(white == Colour::White);
    }

    #[test]
    pub fn to_int() {
        let mut c = Colour::White;
        assert_eq!(c.to_offset(), 0);

        c = Colour::Black;
        assert_eq!(c.to_offset(), 1);
    }

    #[test]
    pub fn colour_flipped() {
        let white_col = Colour::White;
        assert!(white_col.flip_side() == Colour::Black);

        let black_col = Colour::Black;
        assert!(black_col.flip_side() == Colour::White);
    }
}
