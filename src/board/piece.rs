#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PieceRole {
    Pawn = 0,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Piece {
    piece_role: PieceRole,
    colour: Colour,
    offset: u8,
    value: u32,
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
            'P' => return Piece::new(PieceRole::Pawn, Colour::White),
            'B' => return Piece::new(PieceRole::Bishop, Colour::White),
            'N' => return Piece::new(PieceRole::Knight, Colour::White),
            'R' => return Piece::new(PieceRole::Rook, Colour::White),
            'Q' => return Piece::new(PieceRole::Queen, Colour::White),
            'K' => return Piece::new(PieceRole::King, Colour::White),
            'p' => return Piece::new(PieceRole::Pawn, Colour::Black),
            'b' => return Piece::new(PieceRole::Bishop, Colour::Black),
            'n' => return Piece::new(PieceRole::Knight, Colour::Black),
            'r' => return Piece::new(PieceRole::Rook, Colour::Black),
            'q' => return Piece::new(PieceRole::Queen, Colour::Black),
            'k' => return Piece::new(PieceRole::King, Colour::Black),
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
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
    White,
    Black,
}
impl Default for Colour {
    fn default() -> Colour {
        Colour::White
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
}

#[cfg(test)]
pub mod tests {
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;

    #[test]
    pub fn test_flip_side_as_expected() {
        let c = Colour::default();
        assert!(c == Colour::White);

        let f = c.flip_side();
        assert!(f == Colour::Black);

        let o = f.flip_side();
        assert!(o == Colour::White);
    }

    #[test]
    pub fn test_default_colour() {
        let c = Colour::default();
        assert!(c == Colour::White);
    }

    #[test]
    pub fn test_pce_offset_as_expected() {
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
    pub fn test_piece_role_value() {
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
    pub fn test_piece_label_is_valid() {
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
