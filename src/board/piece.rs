use enum_primitive::FromPrimitive;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum Colour {
    White = 0xFF,
    Black = 0x00,
}
const COLOUR_MASK: u8 = 0x01;

// ---- -XXX    ROLE
// ---- X---    Colour 0-> White, 1 -> Black
// XXXX ----    Offset when used in an array
//===========================================
// 0000 0001    Pawn
// 0000 0010    Bishop
// 0000 0011    Knight
// 0000 0100    Rook
// 0000 0101    Queen
// 0000 0110    King
// 0000 1000    BLACK
// 0000 0000    White   Pawn Offset
// 0001 0000            Bishop Offset
// 0010 0000            Knight Offset
// 0011 0000            Rook Offset
// 0100 0000            Queen Offset
// 0101 0000            King Offset
// 0110 0000    Black   Pawn offset
// 0111 0000            Bishop Offset
// 1000 0000            Knight Offset
// 1001 0000            Rook offset
// 1010 0000            Queen Offset
// 1011 0000            King Offset

const PIECE_MASK_ROLE: u8 = 0b00000111;
const PCE_MASK_COLOUR: u8 = 0b00001000;
const PIECE_MASK_OFFSET: u8 = 0b11110000;
const PCE_SHFT_OFFSET: u8 = 4;

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Offset {
    WhitePawn = 0b0000,
    WhiteBishop = 0b0001,
    WhiteKnight = 0b0010,
    WhiteRook = 0b0011,
    WhiteQueen = 0b0100,
    WhiteKing = 0b0101,
    BlackPawn = 0b0110,
    BlackBishop = 0b0111,
    BlackKnight = 0b1000,
    BlackRook = 0b1001,
    BlackQueen = 0b1010,
    BlackKing = 0b1011,
}

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PieceRole {
    Pawn = 0b00000001,
    Bishop = 0b00000010,
    Knight = 0b00000011,
    Rook = 0b00000100,
    Queen = 0b00000101,
    King = 0b00000110,
}

impl fmt::Debug for PieceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        match self {
            PieceRole::Pawn => debug_str.push_str(&format!("RolePawn")),
            PieceRole::Bishop => debug_str.push_str(&format!("RoleBishop")),
            PieceRole::Knight => debug_str.push_str(&format!("RoleKnight")),
            PieceRole::Rook => debug_str.push_str(&format!("RoleRook")),
            PieceRole::Queen => debug_str.push_str(&format!("RoleQueen")),
            PieceRole::King => debug_str.push_str(&format!("RoleKing")),
        }

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for PieceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Piece {
    // white
    WhitePawn = PieceRole::Pawn as u8 | ((Offset::WhitePawn as u8) << PCE_SHFT_OFFSET),
    WhiteBishop = PieceRole::Bishop as u8 | ((Offset::WhiteBishop as u8) << PCE_SHFT_OFFSET),
    WhiteKnight = PieceRole::Knight as u8 | ((Offset::WhiteKnight as u8) << PCE_SHFT_OFFSET),
    WhiteRook = PieceRole::Rook as u8 | ((Offset::WhiteRook as u8) << PCE_SHFT_OFFSET),
    WhiteQueen = PieceRole::Queen as u8 | ((Offset::WhiteQueen as u8) << PCE_SHFT_OFFSET),
    WhiteKing = PieceRole::King as u8 | ((Offset::WhiteKing as u8) << PCE_SHFT_OFFSET),
    // black
    BlackPawn =
        PieceRole::Pawn as u8 | ((Offset::BlackPawn as u8) << PCE_SHFT_OFFSET) | PCE_MASK_COLOUR,
    BlackBishop = PieceRole::Bishop as u8
        | ((Offset::BlackBishop as u8) << PCE_SHFT_OFFSET)
        | PCE_MASK_COLOUR,
    BlackKnight = PieceRole::Knight as u8
        | ((Offset::BlackKnight as u8) << PCE_SHFT_OFFSET)
        | PCE_MASK_COLOUR,
    BlackRook =
        PieceRole::Rook as u8 | ((Offset::BlackRook as u8) << PCE_SHFT_OFFSET) | PCE_MASK_COLOUR,
    BlackQueen =
        PieceRole::Queen as u8 | ((Offset::BlackQueen as u8) << PCE_SHFT_OFFSET) | PCE_MASK_COLOUR,
    BlackKing =
        PieceRole::King as u8 | ((Offset::BlackKing as u8) << PCE_SHFT_OFFSET) | PCE_MASK_COLOUR,
}

impl Piece {
    pub fn new(role: PieceRole, col: Colour) -> Piece {
        match col {
            Colour::White => match role {
                PieceRole::Pawn => return Piece::WhitePawn,
                PieceRole::Bishop => return Piece::WhiteBishop,
                PieceRole::Knight => return Piece::WhiteKnight,
                PieceRole::Rook => return Piece::WhiteRook,
                PieceRole::Queen => return Piece::WhiteQueen,
                PieceRole::King => return Piece::WhiteKing,
            },
            Colour::Black => match role {
                PieceRole::Pawn => return Piece::BlackPawn,
                PieceRole::Bishop => return Piece::BlackBishop,
                PieceRole::Knight => return Piece::BlackKnight,
                PieceRole::Rook => return Piece::BlackRook,
                PieceRole::Queen => return Piece::BlackQueen,
                PieceRole::King => return Piece::BlackKing,
            },
        }
    }

    pub fn colour(&self) -> Colour {
        if *self as u8 & PCE_MASK_COLOUR > 0 {
            return Colour::Black;
        }
        return Colour::White;
    }

    pub fn role(&self) -> PieceRole {
        let role = (*self as u8) & PIECE_MASK_ROLE;
        return PieceRole::from_u8(role).unwrap();
    }

    pub fn offset(&self) -> usize {
        let o = (*self as u8 & PIECE_MASK_OFFSET) >> PCE_SHFT_OFFSET;
        return o as usize;
    }

    pub fn from_char(piece_char: char) -> Piece {
        match piece_char {
            'P' => return Piece::WhitePawn,
            'B' => return Piece::WhiteBishop,
            'N' => return Piece::WhiteKnight,
            'R' => return Piece::WhiteRook,
            'Q' => return Piece::WhiteQueen,
            'K' => return Piece::WhiteKing,
            'p' => return Piece::BlackPawn,
            'b' => return Piece::BlackBishop,
            'n' => return Piece::BlackKnight,
            'r' => return Piece::BlackRook,
            'q' => return Piece::BlackQueen,
            'k' => return Piece::BlackKing,
            _ => panic!("Invalid piece character {}.", piece_char),
        };
    }

    pub fn value(&self) -> u32 {
        let role = self.role();
        // piece values from here:
        // https://www.chessprogramming.org/Simplified_Evaluation_Function

        match role {
            PieceRole::Pawn => return 100,
            PieceRole::Knight => return 320,
            PieceRole::Bishop => return 330,
            PieceRole::Rook => return 500,
            PieceRole::Queen => return 900,
            PieceRole::King => return 20000,
        };
    }

    pub fn to_label(&self) -> String {
        let role = self.role();

        let col = match self.colour() {
            Colour::White => "W",
            Colour::Black => "B",
        };

        match role {
            PieceRole::Pawn => return col.to_owned() + "P",
            PieceRole::Bishop => return col.to_owned() + "B",
            PieceRole::Knight => return col.to_owned() + "N",
            PieceRole::Rook => return col.to_owned() + "R",
            PieceRole::Queen => return col.to_owned() + "Q",
            PieceRole::King => return col.to_owned() + "K",
        };
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        let label = self.to_label().to_string();
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
    pub fn flip_side(&self) -> Colour {
        return Colour::from_u8(!(*self as u8)).unwrap();
    }

    pub fn offset(&self) -> usize {
        return (*self as u8 & COLOUR_MASK) as usize;
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
    pub fn piece_colour_flipped() {
        let pce = Piece::WhiteKnight;
        assert!(pce.colour().flip_side() == Colour::Black);
    }

    #[test]
    pub fn roles_as_expected() {
        assert_eq!(Piece::WhiteBishop.role(), PieceRole::Bishop);
        assert_eq!(Piece::BlackBishop.role(), PieceRole::Bishop);

        assert_eq!(Piece::WhiteKnight.role(), PieceRole::Knight);
        assert_eq!(Piece::BlackKnight.role(), PieceRole::Knight);

        assert_eq!(Piece::WhitePawn.role(), PieceRole::Pawn);
        assert_eq!(Piece::BlackPawn.role(), PieceRole::Pawn);

        assert_eq!(Piece::WhiteRook.role(), PieceRole::Rook);
        assert_eq!(Piece::BlackRook.role(), PieceRole::Rook);

        assert_eq!(Piece::WhiteQueen.role(), PieceRole::Queen);
        assert_eq!(Piece::BlackQueen.role(), PieceRole::Queen);

        assert_eq!(Piece::WhiteKing.role(), PieceRole::King);
        assert_eq!(Piece::BlackKing.role(), PieceRole::King);
    }

    #[test]
    pub fn create_piece() {
        assert_eq!(
            Piece::new(PieceRole::Bishop, Colour::White),
            Piece::WhiteBishop
        );
        assert_eq!(Piece::new(PieceRole::King, Colour::White), Piece::WhiteKing);
        assert_eq!(
            Piece::new(PieceRole::Knight, Colour::White),
            Piece::WhiteKnight
        );
        assert_eq!(Piece::new(PieceRole::Pawn, Colour::White), Piece::WhitePawn);
        assert_eq!(
            Piece::new(PieceRole::Queen, Colour::White),
            Piece::WhiteQueen
        );
        assert_eq!(Piece::new(PieceRole::Rook, Colour::White), Piece::WhiteRook);

        assert_eq!(
            Piece::new(PieceRole::Bishop, Colour::Black),
            Piece::BlackBishop
        );
        assert_eq!(Piece::new(PieceRole::King, Colour::Black), Piece::BlackKing);
        assert_eq!(
            Piece::new(PieceRole::Knight, Colour::Black),
            Piece::BlackKnight
        );
        assert_eq!(Piece::new(PieceRole::Pawn, Colour::Black), Piece::BlackPawn);
        assert_eq!(
            Piece::new(PieceRole::Queen, Colour::Black),
            Piece::BlackQueen
        );
        assert_eq!(Piece::new(PieceRole::Rook, Colour::Black), Piece::BlackRook);
    }

    #[test]
    pub fn colour_as_expected() {
        assert_eq!(Colour::Black, Piece::BlackBishop.colour());
        assert_eq!(Colour::Black, Piece::BlackKing.colour());
        assert_eq!(Colour::Black, Piece::BlackKnight.colour());
        assert_eq!(Colour::Black, Piece::BlackPawn.colour());
        assert_eq!(Colour::Black, Piece::BlackQueen.colour());
        assert_eq!(Colour::Black, Piece::BlackRook.colour());

        assert_eq!(Colour::White, Piece::WhiteBishop.colour());
        assert_eq!(Colour::White, Piece::WhiteKing.colour());
        assert_eq!(Colour::White, Piece::WhiteKnight.colour());
        assert_eq!(Colour::White, Piece::WhitePawn.colour());
        assert_eq!(Colour::White, Piece::WhiteQueen.colour());
        assert_eq!(Colour::White, Piece::WhiteRook.colour());
    }

    #[test]
    pub fn offset_as_expected() {
        assert_eq!(Piece::new(PieceRole::Pawn, Colour::White).offset(), 0);
        assert_eq!(Piece::new(PieceRole::Bishop, Colour::White).offset(), 1);
        assert_eq!(Piece::new(PieceRole::Knight, Colour::White).offset(), 2);
        assert_eq!(Piece::new(PieceRole::Rook, Colour::White).offset(), 3);
        assert_eq!(Piece::new(PieceRole::Queen, Colour::White).offset(), 4);
        assert_eq!(Piece::new(PieceRole::King, Colour::White).offset(), 5);

        assert_eq!(Piece::new(PieceRole::Pawn, Colour::Black).offset(), 6);
        assert_eq!(Piece::new(PieceRole::Bishop, Colour::Black).offset(), 7);
        assert_eq!(Piece::new(PieceRole::Knight, Colour::Black).offset(), 8);
        assert_eq!(Piece::new(PieceRole::Rook, Colour::Black).offset(), 9);
        assert_eq!(Piece::new(PieceRole::Queen, Colour::Black).offset(), 10);
        assert_eq!(Piece::new(PieceRole::King, Colour::Black).offset(), 11);
    }
}
