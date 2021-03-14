use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
    White = 0,
    Black = 1,
}

// Array offsets for each piece
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u8)]
enum Offset {
    WhitePawn = 0,
    WhiteBishop = 1,
    WhiteKnight = 2,
    WhiteRook = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackBishop = 7,
    BlackKnight = 8,
    BlackRook = 9,
    BlackQueen = 10,
    BlackKing = 11,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Piece {
    role: PieceRole,
    colour: Colour,
    offset: usize,
    value: PieceValue,
}

// piece values from here:
// https://www.chessprogramming.org/Simplified_Evaluation_Function
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u32)]
enum PieceValue {
    Pawn = 100,
    Knight = 320,
    Bishop = 330,
    Rook = 500,
    Queen = 900,
    King = 20000,
}

enum_from_primitive! {
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum PieceRole {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}}

impl fmt::Debug for PieceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        match self {
            PieceRole::Pawn => debug_str.push_str(&"RolePawn".to_string()),
            PieceRole::Bishop => debug_str.push_str(&"RoleBishop".to_string()),
            PieceRole::Knight => debug_str.push_str(&"RoleKnight".to_string()),
            PieceRole::Rook => debug_str.push_str(&"RoleRook".to_string()),
            PieceRole::Queen => debug_str.push_str(&"RoleQueen".to_string()),
            PieceRole::King => debug_str.push_str(&"RoleKing".to_string()),
        }

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for PieceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Piece {
    pub const WHITE_PAWN: Piece = Piece {
        role: PieceRole::Pawn,
        colour: Colour::White,
        offset: Offset::WhitePawn as usize,
        value: PieceValue::Pawn,
    };
    pub const WHITE_BISHOP: Piece = Piece {
        role: PieceRole::Bishop,
        colour: Colour::White,
        offset: Offset::WhiteBishop as usize,
        value: PieceValue::Bishop,
    };
    pub const WHITE_KNIGHT: Piece = Piece {
        role: PieceRole::Knight,
        colour: Colour::White,
        offset: Offset::WhiteKnight as usize,
        value: PieceValue::Knight,
    };
    pub const WHITE_ROOK: Piece = Piece {
        role: PieceRole::Rook,
        colour: Colour::White,
        offset: Offset::WhiteRook as usize,
        value: PieceValue::Rook,
    };
    pub const WHITE_QUEEN: Piece = Piece {
        role: PieceRole::Queen,
        colour: Colour::White,
        offset: Offset::WhiteQueen as usize,
        value: PieceValue::Queen,
    };
    pub const WHITE_KING: Piece = Piece {
        role: PieceRole::King,
        colour: Colour::White,
        offset: Offset::WhiteKing as usize,
        value: PieceValue::King,
    };

    pub const BLACK_PAWN: Piece = Piece {
        role: PieceRole::Pawn,
        colour: Colour::Black,
        offset: Offset::BlackPawn as usize,
        value: PieceValue::Pawn,
    };
    pub const BLACK_BISHOP: Piece = Piece {
        role: PieceRole::Bishop,
        colour: Colour::Black,
        offset: Offset::BlackBishop as usize,
        value: PieceValue::Bishop,
    };
    pub const BLACK_KNIGHT: Piece = Piece {
        role: PieceRole::Knight,
        colour: Colour::Black,
        offset: Offset::BlackKnight as usize,
        value: PieceValue::Knight,
    };
    pub const BLACK_ROOK: Piece = Piece {
        role: PieceRole::Rook,
        colour: Colour::Black,
        offset: Offset::BlackRook as usize,
        value: PieceValue::Rook,
    };
    pub const BLACK_QUEEN: Piece = Piece {
        role: PieceRole::Queen,
        colour: Colour::Black,
        offset: Offset::BlackQueen as usize,
        value: PieceValue::Queen,
    };
    pub const BLACK_KING: Piece = Piece {
        role: PieceRole::King,
        colour: Colour::Black,
        offset: Offset::BlackKing as usize,
        value: PieceValue::King,
    };

    pub const fn new(role: PieceRole, col: Colour) -> &'static Piece {
        match col {
            Colour::White => match role {
                PieceRole::Pawn => &Piece::WHITE_PAWN,
                PieceRole::Bishop => &Piece::WHITE_BISHOP,
                PieceRole::Knight => &Piece::WHITE_KNIGHT,
                PieceRole::Rook => &Piece::WHITE_ROOK,
                PieceRole::Queen => &Piece::WHITE_QUEEN,
                PieceRole::King => &Piece::WHITE_KING,
            },
            Colour::Black => match role {
                PieceRole::Pawn => &Piece::BLACK_PAWN,
                PieceRole::Bishop => &Piece::BLACK_BISHOP,
                PieceRole::Knight => &Piece::BLACK_KNIGHT,
                PieceRole::Rook => &Piece::BLACK_ROOK,
                PieceRole::Queen => &Piece::BLACK_QUEEN,
                PieceRole::King => &Piece::BLACK_KING,
            },
        }
    }

    pub const fn to_offset(&self) -> usize {
        return self.offset;
    }

    pub fn from_offset(offset: u8) -> &'static Piece {
        match offset {
            0 => &Piece::WHITE_PAWN,
            1 => &Piece::WHITE_BISHOP,
            2 => &Piece::WHITE_KNIGHT,
            3 => &Piece::WHITE_ROOK,
            4 => &Piece::WHITE_QUEEN,
            5 => &Piece::WHITE_KING,
            6 => &Piece::BLACK_PAWN,
            7 => &Piece::BLACK_BISHOP,
            8 => &Piece::BLACK_KNIGHT,
            9 => &Piece::BLACK_ROOK,
            10 => &Piece::BLACK_QUEEN,
            11 => &Piece::BLACK_KING,
            _ => panic!("Invalid piece offset {}.", offset),
        }
    }

    pub const fn colour(&self) -> Colour {
        return self.colour;
    }

    pub const fn role(&self) -> PieceRole {
        return self.role;
    }

    pub fn from_char(piece_char: char) -> &'static Piece {
        match piece_char {
            'P' => &Piece::WHITE_PAWN,
            'B' => &Piece::WHITE_BISHOP,
            'N' => &Piece::WHITE_KNIGHT,
            'R' => &Piece::WHITE_ROOK,
            'Q' => &Piece::WHITE_QUEEN,
            'K' => &Piece::WHITE_KING,
            'p' => &Piece::BLACK_PAWN,
            'b' => &Piece::BLACK_BISHOP,
            'n' => &Piece::BLACK_KNIGHT,
            'r' => &Piece::BLACK_ROOK,
            'q' => &Piece::BLACK_QUEEN,
            'k' => &Piece::BLACK_KING,
            _ => panic!("Invalid piece character {}.", piece_char),
        }
    }

    pub const fn value(&self) -> u32 {
        self.value as u32
    }

    pub fn to_label(self) -> String {
        let role = self.role();

        let col = match self.colour() {
            Colour::White => "W",
            Colour::Black => "B",
        };

        match role {
            PieceRole::Pawn => col.to_owned() + "P",
            PieceRole::Bishop => col.to_owned() + "B",
            PieceRole::Knight => col.to_owned() + "N",
            PieceRole::Rook => col.to_owned() + "R",
            PieceRole::Queen => col.to_owned() + "Q",
            PieceRole::King => col.to_owned() + "K",
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

pub const NUM_PIECES: usize = 12;
pub const NUM_PIECE_ROLES: usize = 6;
pub const NUM_COLOURS: usize = 2;

#[cfg(test)]
pub mod tests {
    use components::piece::Colour;
    use components::piece::Piece;
    use components::piece::PieceRole;

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
        let pce = Piece::WHITE_KNIGHT;
        assert!(pce.colour().flip_side() == Colour::Black);
    }

    #[test]
    pub fn roles_as_expected() {
        assert_eq!(Piece::WHITE_BISHOP.role(), PieceRole::Bishop);
        assert_eq!(Piece::BLACK_BISHOP.role(), PieceRole::Bishop);

        assert_eq!(Piece::WHITE_KNIGHT.role(), PieceRole::Knight);
        assert_eq!(Piece::BLACK_KNIGHT.role(), PieceRole::Knight);

        assert_eq!(Piece::WHITE_PAWN.role(), PieceRole::Pawn);
        assert_eq!(Piece::BLACK_PAWN.role(), PieceRole::Pawn);

        assert_eq!(Piece::WHITE_ROOK.role(), PieceRole::Rook);
        assert_eq!(Piece::BLACK_ROOK.role(), PieceRole::Rook);

        assert_eq!(Piece::WHITE_QUEEN.role(), PieceRole::Queen);
        assert_eq!(Piece::BLACK_QUEEN.role(), PieceRole::Queen);

        assert_eq!(Piece::WHITE_KING.role(), PieceRole::King);
        assert_eq!(Piece::BLACK_KING.role(), PieceRole::King);
    }

    #[test]
    pub fn create_piece() {
        assert_eq!(
            Piece::new(PieceRole::Bishop, Colour::White),
            &Piece::WHITE_BISHOP
        );
        assert_eq!(
            Piece::new(PieceRole::King, Colour::White),
            &Piece::WHITE_KING
        );
        assert_eq!(
            Piece::new(PieceRole::Knight, Colour::White),
            &Piece::WHITE_KNIGHT
        );
        assert_eq!(
            Piece::new(PieceRole::Pawn, Colour::White),
            &Piece::WHITE_PAWN
        );
        assert_eq!(
            Piece::new(PieceRole::Queen, Colour::White),
            &Piece::WHITE_QUEEN
        );
        assert_eq!(
            Piece::new(PieceRole::Rook, Colour::White),
            &Piece::WHITE_ROOK
        );

        assert_eq!(
            Piece::new(PieceRole::Bishop, Colour::Black),
            &Piece::BLACK_BISHOP
        );
        assert_eq!(
            Piece::new(PieceRole::King, Colour::Black),
            &Piece::BLACK_KING
        );
        assert_eq!(
            Piece::new(PieceRole::Knight, Colour::Black),
            &Piece::BLACK_KNIGHT
        );
        assert_eq!(
            Piece::new(PieceRole::Pawn, Colour::Black),
            &Piece::BLACK_PAWN
        );
        assert_eq!(
            Piece::new(PieceRole::Queen, Colour::Black),
            &Piece::BLACK_QUEEN
        );
        assert_eq!(
            Piece::new(PieceRole::Rook, Colour::Black),
            &Piece::BLACK_ROOK
        );
    }

    #[test]
    pub fn colour_as_expected() {
        assert_eq!(Colour::Black, Piece::BLACK_BISHOP.colour());
        assert_eq!(Colour::Black, Piece::BLACK_KING.colour());
        assert_eq!(Colour::Black, Piece::BLACK_KNIGHT.colour());
        assert_eq!(Colour::Black, Piece::BLACK_PAWN.colour());
        assert_eq!(Colour::Black, Piece::BLACK_QUEEN.colour());
        assert_eq!(Colour::Black, Piece::BLACK_KING.colour());

        assert_eq!(Colour::White, Piece::WHITE_BISHOP.colour());
        assert_eq!(Colour::White, Piece::WHITE_KING.colour());
        assert_eq!(Colour::White, Piece::WHITE_KNIGHT.colour());
        assert_eq!(Colour::White, Piece::WHITE_PAWN.colour());
        assert_eq!(Colour::White, Piece::WHITE_QUEEN.colour());
        assert_eq!(Colour::White, Piece::WHITE_ROOK.colour());
    }

    #[test]
    pub fn offset_as_expected() {
        assert_eq!(Piece::new(PieceRole::Pawn, Colour::White).to_offset(), 0);
        assert_eq!(Piece::new(PieceRole::Bishop, Colour::White).to_offset(), 1);
        assert_eq!(Piece::new(PieceRole::Knight, Colour::White).to_offset(), 2);
        assert_eq!(Piece::new(PieceRole::Rook, Colour::White).to_offset(), 3);
        assert_eq!(Piece::new(PieceRole::Queen, Colour::White).to_offset(), 4);
        assert_eq!(Piece::new(PieceRole::King, Colour::White).to_offset(), 5);

        assert_eq!(Piece::new(PieceRole::Pawn, Colour::Black).to_offset(), 6);
        assert_eq!(Piece::new(PieceRole::Bishop, Colour::Black).to_offset(), 7);
        assert_eq!(Piece::new(PieceRole::Knight, Colour::Black).to_offset(), 8);
        assert_eq!(Piece::new(PieceRole::Rook, Colour::Black).to_offset(), 9);
        assert_eq!(Piece::new(PieceRole::Queen, Colour::Black).to_offset(), 10);
        assert_eq!(Piece::new(PieceRole::King, Colour::Black).to_offset(), 11);
    }
}
