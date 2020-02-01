use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
#[repr(u8)]
pub enum PieceRole {
    Pawn = 0,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Colour {
    White = 0,
    Black,
}

impl Default for Colour {
    fn default() -> Colour {
        Colour::White
    }
}

// todo look at mapping this to a u64 (or u32 if we remove the value)
#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct Piece {
    piece_role: PieceRole,
    colour: Colour,
    offset: u8,
    value: u32,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_label())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_label())
    }
}

lazy_static! {
    pub static ref ROOK_BLACK: Piece = Piece::new(PieceRole::Rook, Colour::Black);
    pub static ref ROOK_WHITE: Piece = Piece::new(PieceRole::Rook, Colour::White);
    pub static ref QUEEN_BLACK: Piece = Piece::new(PieceRole::Queen, Colour::Black);
    pub static ref QUEEN_WHITE: Piece = Piece::new(PieceRole::Queen, Colour::White);
    pub static ref BISHOP_BLACK: Piece = Piece::new(PieceRole::Bishop, Colour::Black);
    pub static ref BISHOP_WHITE: Piece = Piece::new(PieceRole::Bishop, Colour::White);
    pub static ref KNIGHT_BLACK: Piece = Piece::new(PieceRole::Knight, Colour::Black);
    pub static ref KNIGHT_WHITE: Piece = Piece::new(PieceRole::Knight, Colour::White);
    pub static ref KING_BLACK: Piece = Piece::new(PieceRole::King, Colour::Black);
    pub static ref KING_WHITE: Piece = Piece::new(PieceRole::King, Colour::White);
    pub static ref PAWN_BLACK: Piece = Piece::new(PieceRole::Pawn, Colour::Black);
    pub static ref PAWN_WHITE: Piece = Piece::new(PieceRole::Pawn, Colour::White);
}

impl Piece {
    pub fn new(role: PieceRole, colour: Colour) -> Piece {
        let off = pce_to_offset(role, colour);
        let val = pce_value(role);
        Piece {
            piece_role: role,
            colour: colour,
            offset: off,
            value: val,
        }
    }

    pub fn from_char(piece_char: char) -> Piece {
        match piece_char {
            'P' => return *PAWN_WHITE,
            'B' => return *BISHOP_WHITE,
            'N' => return *KNIGHT_WHITE,
            'R' => return *ROOK_WHITE,
            'Q' => return *QUEEN_WHITE,
            'K' => return *KING_WHITE,
            'p' => return *PAWN_BLACK,
            'b' => return *BISHOP_BLACK,
            'n' => return *KNIGHT_BLACK,
            'r' => return *ROOK_BLACK,
            'q' => return *QUEEN_BLACK,
            'k' => return *KING_BLACK,
            _ => panic!("Invalid piece character {}.", piece_char),
        };
    }

    pub fn colour(self) -> Colour {
        self.colour
    }
    pub fn role(self) -> PieceRole {
        self.piece_role
    }

    pub fn offset(self) -> usize {
        self.offset as usize
    }
    pub fn value(self) -> u32 {
        self.value
    }

    pub fn to_label(self) -> String {
        match self.colour() {
            Colour::White => match self.role() {
                PieceRole::Pawn => return String::from("WP"),
                PieceRole::Bishop => return String::from("WB"),
                PieceRole::Knight => return String::from("WN"),
                PieceRole::Rook => return String::from("WR"),
                PieceRole::Queen => return String::from("WQ"),
                PieceRole::King => return String::from("WK"),
            },
            Colour::Black => match self.role() {
                PieceRole::Pawn => return String::from("BP"),
                PieceRole::Bishop => return String::from("BB"),
                PieceRole::Knight => return String::from("BN"),
                PieceRole::Rook => return String::from("BR"),
                PieceRole::Queen => return String::from("BQ"),
                PieceRole::King => return String::from("BK"),
            },
        }
    }
}

pub const NUM_PIECES: usize = 12;
pub const NUM_PIECE_ROLES: usize = 6;
pub const NUM_COLOURS: usize = 2;

fn pce_to_offset(pce_role: PieceRole, col: Colour) -> u8 {
    let mut role_val = match pce_role {
        PieceRole::Pawn => 0,
        PieceRole::Bishop => 1,
        PieceRole::Knight => 2,
        PieceRole::Rook => 3,
        PieceRole::Queen => 4,
        PieceRole::King => 5,
    };

    if col == Colour::Black {
        role_val += NUM_PIECE_ROLES;
    }
    return role_val as u8;
}

fn pce_value(role: PieceRole) -> u32 {
    match role {
        PieceRole::Pawn => 300,
        PieceRole::Bishop => 550,
        PieceRole::Knight => 550,
        PieceRole::Rook => 800,
        PieceRole::Queen => 1000,
        PieceRole::King => 50000,
    }
}

impl Colour {
    pub fn flip_side(&self) -> Colour {
        if *self == Colour::White {
            return Colour::Black;
        } else {
            return Colour::White;
        }
    }
    pub fn offset(&self) -> usize {
        match self {
            Colour::White => 0,
            Colour::Black => 1,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;

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
    pub fn pce_offset_as_expected() {
        let mut pce = Piece::new(PieceRole::Pawn, Colour::White);
        assert_eq!(pce.offset, 0);

        pce = Piece::new(PieceRole::Bishop, Colour::White);
        assert_eq!(pce.offset, 1);

        pce = Piece::new(PieceRole::Knight, Colour::White);
        assert_eq!(pce.offset, 2);

        pce = Piece::new(PieceRole::Rook, Colour::White);
        assert_eq!(pce.offset, 3);

        pce = Piece::new(PieceRole::Queen, Colour::White);
        assert_eq!(pce.offset, 4);

        pce = Piece::new(PieceRole::King, Colour::White);
        assert_eq!(pce.offset, 5);

        pce = Piece::new(PieceRole::Pawn, Colour::Black);
        assert_eq!(pce.offset, 6);

        pce = Piece::new(PieceRole::Bishop, Colour::Black);
        assert_eq!(pce.offset, 7);

        pce = Piece::new(PieceRole::Knight, Colour::Black);
        assert_eq!(pce.offset, 8);

        pce = Piece::new(PieceRole::Rook, Colour::Black);
        assert_eq!(pce.offset, 9);

        pce = Piece::new(PieceRole::Queen, Colour::Black);
        assert_eq!(pce.offset, 10);

        pce = Piece::new(PieceRole::King, Colour::Black);
        assert_eq!(pce.offset, 11);
    }

    #[test]
    pub fn piece_role_value() {
        let mut pce = Piece::new(PieceRole::Pawn, Colour::White);
        assert_eq!(pce.value, 300);

        pce = Piece::new(PieceRole::Bishop, Colour::White);
        assert_eq!(pce.value, 550);

        pce = Piece::new(PieceRole::Knight, Colour::White);
        assert_eq!(pce.value, 550);

        pce = Piece::new(PieceRole::Rook, Colour::White);
        assert_eq!(pce.value, 800);

        pce = Piece::new(PieceRole::Queen, Colour::White);
        assert_eq!(pce.value, 1000);

        pce = Piece::new(PieceRole::King, Colour::White);
        assert_eq!(pce.value, 50000);

        pce = Piece::new(PieceRole::Pawn, Colour::Black);
        assert_eq!(pce.value, 300);

        pce = Piece::new(PieceRole::Bishop, Colour::Black);
        assert_eq!(pce.value, 550);

        pce = Piece::new(PieceRole::Knight, Colour::Black);
        assert_eq!(pce.value, 550);

        pce = Piece::new(PieceRole::Rook, Colour::Black);
        assert_eq!(pce.value, 800);

        pce = Piece::new(PieceRole::Queen, Colour::Black);
        assert_eq!(pce.value, 1000);

        pce = Piece::new(PieceRole::King, Colour::Black);
        assert_eq!(pce.value, 50000);
    }

    #[test]
    pub fn piece_label_is_valid() {
        let mut pce = Piece::from_char('P');
        assert_eq!(Piece::new(PieceRole::Pawn, Colour::White), pce);

        pce = Piece::from_char('B');
        assert_eq!(Piece::new(PieceRole::Bishop, Colour::White), pce);

        pce = Piece::from_char('N');
        assert_eq!(Piece::new(PieceRole::Knight, Colour::White), pce);

        pce = Piece::from_char('R');
        assert_eq!(Piece::new(PieceRole::Rook, Colour::White), pce);

        pce = Piece::from_char('Q');
        assert_eq!(Piece::new(PieceRole::Queen, Colour::White), pce);

        pce = Piece::from_char('K');
        assert_eq!(Piece::new(PieceRole::King, Colour::White), pce);

        pce = Piece::from_char('p');
        assert_eq!(Piece::new(PieceRole::Pawn, Colour::Black), pce);

        pce = Piece::from_char('b');
        assert_eq!(Piece::new(PieceRole::Bishop, Colour::Black), pce);

        pce = Piece::from_char('n');
        assert_eq!(Piece::new(PieceRole::Knight, Colour::Black), pce);

        pce = Piece::from_char('r');
        assert_eq!(Piece::new(PieceRole::Rook, Colour::Black), pce);

        pce = Piece::from_char('q');
        assert_eq!(Piece::new(PieceRole::Queen, Colour::Black), pce);

        pce = Piece::from_char('k');
        assert_eq!(Piece::new(PieceRole::King, Colour::Black), pce);
    }
}
