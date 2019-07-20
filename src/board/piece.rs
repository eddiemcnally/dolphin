#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
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
}

impl Piece {
    pub fn new(role: PieceRole, colour: Colour) -> Piece {
        let off = pce_to_offset(role, colour);
        Piece {
            piece_role: role,
            colour: colour,
            offset: off,
        }
    }

    pub fn value(self) -> u32 {
        match self.piece_role {
            PieceRole::Pawn => 300,
            PieceRole::Bishop => 550,
            PieceRole::Knight => 550,
            PieceRole::Rook => 800,
            PieceRole::Queen => 1000,
            PieceRole::King => 50000,
        }
    }

    pub fn from_char(piece_char: char) -> Option<Piece> {
        let pce = match piece_char {
            'P' => Piece::new(PieceRole::Pawn, Colour::White),
            'B' => Piece::new(PieceRole::Bishop, Colour::White),
            'N' => Piece::new(PieceRole::Knight, Colour::White),
            'R' => Piece::new(PieceRole::Rook, Colour::White),
            'Q' => Piece::new(PieceRole::Queen, Colour::White),
            'K' => Piece::new(PieceRole::King, Colour::White),
            'p' => Piece::new(PieceRole::Pawn, Colour::Black),
            'b' => Piece::new(PieceRole::Bishop, Colour::Black),
            'n' => Piece::new(PieceRole::Knight, Colour::Black),
            'r' => Piece::new(PieceRole::Rook, Colour::Black),
            'q' => Piece::new(PieceRole::Queen, Colour::Black),
            'k' => Piece::new(PieceRole::King, Colour::Black),
            _ => return None,
        };
        Some(pce)
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
        _ => panic!("invalid pawn role"),
    };

    if col == Colour::Black {
        role_val += NUM_PIECE_ROLES;
    }
    return role_val as u8;
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
