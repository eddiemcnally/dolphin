use core::core_traits::ArrayAccessor;
use enum_primitive::FromPrimitive;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum Colour {
    White = 0x0,
    Black = 0x1,
}

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

const PIECE_MASK_ROLE: u8 = 0b0000_0111;
const PCE_MASK_COLOUR: u8 = 0b0000_1000;
const PCE_SHFT_COLOUR: u8 = 3;
const PIECE_MASK_OFFSET: u8 = 0b1111_0000;
const PCE_SHFT_OFFSET: u8 = 4;

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
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

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PieceRole {
    Pawn = 0b0000_0001,
    Bishop = 0b0000_0010,
    Knight = 0b0000_0011,
    Rook = 0b0000_0100,
    Queen = 0b0000_0101,
    King = 0b0000_0110,
}

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
    pub const fn new(role: PieceRole, col: Colour) -> Piece {
        match col {
            Colour::White => match role {
                PieceRole::Pawn => Piece::WhitePawn,
                PieceRole::Bishop => Piece::WhiteBishop,
                PieceRole::Knight => Piece::WhiteKnight,
                PieceRole::Rook => Piece::WhiteRook,
                PieceRole::Queen => Piece::WhiteQueen,
                PieceRole::King => Piece::WhiteKing,
            },
            Colour::Black => match role {
                PieceRole::Pawn => Piece::BlackPawn,
                PieceRole::Bishop => Piece::BlackBishop,
                PieceRole::Knight => Piece::BlackKnight,
                PieceRole::Rook => Piece::BlackRook,
                PieceRole::Queen => Piece::BlackQueen,
                PieceRole::King => Piece::BlackKing,
            },
        }
    }

    pub const fn offset(self) -> usize {
        let o = (self as u8 & PIECE_MASK_OFFSET) >> PCE_SHFT_OFFSET;
        o as usize
    }

    pub fn from_offset(offset: u8) -> Piece {
        match offset {
            0 => Piece::WhitePawn,
            1 => Piece::WhiteBishop,
            2 => Piece::WhiteKnight,
            3 => Piece::WhiteRook,
            4 => Piece::WhiteQueen,
            5 => Piece::WhiteKing,
            6 => Piece::BlackPawn,
            7 => Piece::BlackBishop,
            8 => Piece::BlackKnight,
            9 => Piece::BlackRook,
            10 => Piece::BlackQueen,
            11 => Piece::BlackKing,
            _ => panic!("Invalid piece offset {}.", offset),
        }
    }

    pub const fn colour(self) -> Colour {
        let c = (self as u8 & PCE_MASK_COLOUR) >> PCE_SHFT_COLOUR;

        // TODO replace this with a 'match' when panic can be called in a const fn
        if c == 0 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    pub fn role(self) -> PieceRole {
        let role = (self as u8) & PIECE_MASK_ROLE;
        PieceRole::from_u8(role).unwrap()
    }

    pub fn from_char(piece_char: char) -> Piece {
        match piece_char {
            'P' => Piece::WhitePawn,
            'B' => Piece::WhiteBishop,
            'N' => Piece::WhiteKnight,
            'R' => Piece::WhiteRook,
            'Q' => Piece::WhiteQueen,
            'K' => Piece::WhiteKing,
            'p' => Piece::BlackPawn,
            'b' => Piece::BlackBishop,
            'n' => Piece::BlackKnight,
            'r' => Piece::BlackRook,
            'q' => Piece::BlackQueen,
            'k' => Piece::BlackKing,
            _ => panic!("Invalid piece character {}.", piece_char),
        }
    }

    pub fn value(self) -> u32 {
        let role = self.role();
        // piece values from here:
        // https://www.chessprogramming.org/Simplified_Evaluation_Function

        match role {
            PieceRole::Pawn => 100,
            PieceRole::Knight => 320,
            PieceRole::Bishop => 330,
            PieceRole::Rook => 500,
            PieceRole::Queen => 900,
            PieceRole::King => 20000,
        }
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

impl ArrayAccessor for Piece {
    fn to_offset(self) -> usize {
        let o = (self as u8 & PIECE_MASK_OFFSET) >> PCE_SHFT_OFFSET;
        o as usize
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
}

impl Default for Colour {
    fn default() -> Colour {
        Colour::White
    }
}

impl ArrayAccessor for Colour {
    fn to_offset(self) -> usize {
        self as usize
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
    use core::core_traits::ArrayAccessor;

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
