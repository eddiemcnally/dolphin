use crate::board::colour::Colour;
use std::fmt;

#[derive(Eq, PartialEq)]
pub struct Piece {
    array_offset: usize,
    piece_type: PieceType,
    colour: Colour,
    value: u32,
    label: char,
    role: PieceRole,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum PieceType {
    WhitePawn,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackQueen,
    BlackKing,
}
impl fmt::Debug for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::WhitePawn => write!(f, "WhitePawn"),
            PieceType::WhiteBishop => write!(f, "WhiteBishop"),
            PieceType::WhiteKnight => write!(f, "WhiteKnight"),
            PieceType::WhiteRook => write!(f, "WhiteRook"),
            PieceType::WhiteQueen => write!(f, "WhiteQueen"),
            PieceType::WhiteKing => write!(f, "WhiteKing"),
            PieceType::BlackPawn => write!(f, "BlackPawn"),
            PieceType::BlackBishop => write!(f, "BlackBishop"),
            PieceType::BlackKnight => write!(f, "BlackKnight"),
            PieceType::BlackRook => write!(f, "BlackRook"),
            PieceType::BlackQueen => write!(f, "BlackQueen"),
            PieceType::BlackKing => write!(f, "BlackKing"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum PieceRole {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub const WHITE_PAWN: Piece = Piece {
    piece_type: PieceType::WhitePawn,
    colour: Colour::White,
    array_offset: PieceType::WhitePawn as usize,
    label: 'P',
    value: PieceValue::Pawn as u32,
    role: PieceRole::Pawn,
};

pub const WHITE_BISHOP: Piece = Piece {
    piece_type: PieceType::WhiteBishop,
    colour: Colour::White,
    array_offset: PieceType::WhiteBishop as usize,
    label: 'B',
    value: PieceValue::Bishop as u32,
    role: PieceRole::Bishop,
};

pub const WHITE_KNIGHT: Piece = Piece {
    piece_type: PieceType::WhiteKnight,
    colour: Colour::White,
    array_offset: PieceType::WhiteKnight as usize,
    label: 'N',
    value: PieceValue::Knight as u32,
    role: PieceRole::Knight,
};

pub const WHITE_ROOK: Piece = Piece {
    piece_type: PieceType::WhiteRook,
    colour: Colour::White,
    array_offset: PieceType::WhiteRook as usize,
    label: 'R',
    value: PieceValue::Rook as u32,
    role: PieceRole::Rook,
};

pub const WHITE_QUEEN: Piece = Piece {
    piece_type: PieceType::WhiteQueen,
    colour: Colour::White,
    array_offset: PieceType::WhiteQueen as usize,
    label: 'Q',
    value: PieceValue::Queen as u32,
    role: PieceRole::Queen,
};

pub const WHITE_KING: Piece = Piece {
    piece_type: PieceType::WhiteKing,
    colour: Colour::White,
    array_offset: PieceType::WhiteKing as usize,
    label: 'K',
    value: PieceValue::King as u32,
    role: PieceRole::King,
};

pub const BLACK_PAWN: Piece = Piece {
    piece_type: PieceType::BlackPawn,
    colour: Colour::Black,
    array_offset: PieceType::BlackPawn as usize,
    label: 'p',
    value: PieceValue::Pawn as u32,
    role: PieceRole::Pawn,
};

pub const BLACK_BISHOP: Piece = Piece {
    piece_type: PieceType::BlackBishop,
    colour: Colour::Black,
    array_offset: PieceType::BlackBishop as usize,
    label: 'b',
    value: PieceValue::Bishop as u32,
    role: PieceRole::Bishop,
};

pub const BLACK_KNIGHT: Piece = Piece {
    piece_type: PieceType::BlackKnight,
    colour: Colour::Black,
    array_offset: PieceType::BlackKnight as usize,
    label: 'n',
    value: PieceValue::Knight as u32,
    role: PieceRole::Knight,
};

pub const BLACK_ROOK: Piece = Piece {
    piece_type: PieceType::BlackRook,
    colour: Colour::Black,
    array_offset: PieceType::BlackRook as usize,
    label: 'r',
    value: PieceValue::Rook as u32,
    role: PieceRole::Rook,
};

pub const BLACK_QUEEN: Piece = Piece {
    piece_type: PieceType::BlackQueen,
    colour: Colour::Black,
    array_offset: PieceType::BlackQueen as usize,
    label: 'q',
    value: PieceValue::Queen as u32,
    role: PieceRole::Queen,
};

pub const BLACK_KING: Piece = Piece {
    piece_type: PieceType::BlackKing,
    colour: Colour::Black,
    array_offset: PieceType::BlackKing as usize,
    label: 'k',
    value: PieceValue::King as u32,
    role: PieceRole::King,
};

#[rustfmt::skip]
pub const ALL_PIECES: &[Piece] = &[
    WHITE_PAWN,
    WHITE_BISHOP,
    WHITE_KNIGHT,
    WHITE_ROOK,
    WHITE_QUEEN,
    WHITE_KING,
    BLACK_PAWN,
    BLACK_BISHOP,
    BLACK_KNIGHT,
    BLACK_ROOK,
    BLACK_QUEEN,
    BLACK_KING, 
];

impl Piece {
    pub const fn piece_type(&self) -> PieceType {
        self.piece_type
    }
    pub const fn value(&self) -> u32 {
        self.value
    }
    pub const fn colour(&self) -> Colour {
        self.colour
    }
    pub const fn label(&self) -> char {
        self.label
    }
    pub const fn offset(&self) -> usize {
        self.array_offset
    }

    pub fn is_king(&self) -> bool {
        self.role == PieceRole::King
    }
    pub fn is_pawn(&self) -> bool {
        self.role == PieceRole::Pawn
    }
    pub fn is_rook(&self) -> bool {
        self.role == PieceRole::Rook
    }

    pub fn from_char(piece_char: char) -> &'static Piece {
        match piece_char {
            'P' => &WHITE_PAWN,
            'B' => &WHITE_BISHOP,
            'N' => &WHITE_KNIGHT,
            'R' => &WHITE_ROOK,
            'Q' => &WHITE_QUEEN,
            'K' => &WHITE_KING,
            'p' => &BLACK_PAWN,
            'b' => &BLACK_BISHOP,
            'n' => &BLACK_KNIGHT,
            'r' => &BLACK_ROOK,
            'q' => &BLACK_QUEEN,
            'k' => &BLACK_KING,
            _ => panic!("Invalid piece character {}.", piece_char),
        }
    }
}

pub const NUM_PIECES: usize = 32;
pub const NUM_PIECE_TYPES: usize = 12;

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

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        let label = self.label();
        debug_str.push_str(&format!("{:?}", label));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Default for Piece {
    fn default() -> Piece {
        WHITE_PAWN
    }
}

impl Default for &Piece {
    fn default() -> &'static Piece {
        &WHITE_PAWN
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::colour::Colour;
    use crate::board::piece;
    use crate::board::piece::PieceType;
    use crate::board::piece::PieceValue;

    #[test]
    pub fn piece_colour_as_expected() {
        let mut pce = piece::WHITE_PAWN;
        assert_eq!(Colour::White, pce.colour());
        pce = piece::WHITE_BISHOP;
        assert_eq!(Colour::White, pce.colour());
        pce = piece::WHITE_KNIGHT;
        assert_eq!(Colour::White, pce.colour());
        pce = piece::WHITE_ROOK;
        assert_eq!(Colour::White, pce.colour());
        pce = piece::WHITE_QUEEN;
        assert_eq!(Colour::White, pce.colour());
        pce = piece::WHITE_KING;
        assert_eq!(Colour::White, pce.colour());

        pce = piece::BLACK_PAWN;
        assert_eq!(Colour::Black, pce.colour());
        pce = piece::BLACK_BISHOP;
        assert_eq!(Colour::Black, pce.colour());
        pce = piece::BLACK_KNIGHT;
        assert_eq!(Colour::Black, pce.colour());
        pce = piece::BLACK_ROOK;
        assert_eq!(Colour::Black, pce.colour());
        pce = piece::BLACK_QUEEN;
        assert_eq!(Colour::Black, pce.colour());
        pce = piece::BLACK_KING;
        assert_eq!(Colour::Black, pce.colour());
    }

    #[test]
    pub fn piece_offset_as_expected() {
        let mut pce = piece::WHITE_PAWN;
        assert_eq!(pce.offset(), PieceType::WhitePawn as usize);
        pce = piece::WHITE_BISHOP;
        assert_eq!(pce.offset(), PieceType::WhiteBishop as usize);
        pce = piece::WHITE_KNIGHT;
        assert_eq!(pce.offset(), PieceType::WhiteKnight as usize);
        pce = piece::WHITE_ROOK;
        assert_eq!(pce.offset(), PieceType::WhiteRook as usize);
        pce = piece::WHITE_QUEEN;
        assert_eq!(pce.offset(), PieceType::WhiteQueen as usize);
        pce = piece::WHITE_KING;
        assert_eq!(pce.offset(), PieceType::WhiteKing as usize);

        pce = piece::BLACK_PAWN;
        assert_eq!(pce.offset(), PieceType::BlackPawn as usize);
        pce = piece::BLACK_BISHOP;
        assert_eq!(pce.offset(), PieceType::BlackBishop as usize);
        pce = piece::BLACK_KNIGHT;
        assert_eq!(pce.offset(), PieceType::BlackKnight as usize);
        pce = piece::BLACK_ROOK;
        assert_eq!(pce.offset(), PieceType::BlackRook as usize);
        pce = piece::BLACK_QUEEN;
        assert_eq!(pce.offset(), PieceType::BlackQueen as usize);
        pce = piece::BLACK_KING;
        assert_eq!(pce.offset(), PieceType::BlackKing as usize);
    }

    #[test]
    pub fn piece_value_as_expected() {
        let mut pce = piece::WHITE_PAWN;
        assert_eq!(pce.value(), PieceValue::Pawn as u32);
        pce = piece::WHITE_BISHOP;
        assert_eq!(pce.value(), PieceValue::Bishop as u32);
        pce = piece::WHITE_KNIGHT;
        assert_eq!(pce.value(), PieceValue::Knight as u32);
        pce = piece::WHITE_ROOK;
        assert_eq!(pce.value(), PieceValue::Rook as u32);
        pce = piece::WHITE_QUEEN;
        assert_eq!(pce.value(), PieceValue::Queen as u32);
        pce = piece::WHITE_KING;
        assert_eq!(pce.value(), PieceValue::King as u32);

        pce = piece::BLACK_PAWN;
        assert_eq!(pce.value(), PieceValue::Pawn as u32);
        pce = piece::BLACK_BISHOP;
        assert_eq!(pce.value(), PieceValue::Bishop as u32);
        pce = piece::BLACK_KNIGHT;
        assert_eq!(pce.value(), PieceValue::Knight as u32);
        pce = piece::BLACK_ROOK;
        assert_eq!(pce.value(), PieceValue::Rook as u32);
        pce = piece::BLACK_QUEEN;
        assert_eq!(pce.value(), PieceValue::Queen as u32);
        pce = piece::BLACK_KING;
        assert_eq!(pce.value(), PieceValue::King as u32);
    }

    #[test]
    pub fn piece_type_as_expected() {
        let mut pce = piece::WHITE_PAWN;
        assert_eq!(pce.piece_type(), PieceType::WhitePawn);
        pce = piece::WHITE_BISHOP;
        assert_eq!(pce.piece_type(), PieceType::WhiteBishop);
        pce = piece::WHITE_KNIGHT;
        assert_eq!(pce.piece_type(), PieceType::WhiteKnight);
        pce = piece::WHITE_ROOK;
        assert_eq!(pce.piece_type(), PieceType::WhiteRook);
        pce = piece::WHITE_QUEEN;
        assert_eq!(pce.piece_type(), PieceType::WhiteQueen);
        pce = piece::WHITE_KING;
        assert_eq!(pce.piece_type(), PieceType::WhiteKing);

        pce = piece::BLACK_PAWN;
        assert_eq!(pce.piece_type(), PieceType::BlackPawn);
        pce = piece::BLACK_BISHOP;
        assert_eq!(pce.piece_type(), PieceType::BlackBishop);
        pce = piece::BLACK_KNIGHT;
        assert_eq!(pce.piece_type(), PieceType::BlackKnight);
        pce = piece::BLACK_ROOK;
        assert_eq!(pce.piece_type(), PieceType::BlackRook);
        pce = piece::BLACK_QUEEN;
        assert_eq!(pce.piece_type(), PieceType::BlackQueen);
        pce = piece::BLACK_KING;
        assert_eq!(pce.piece_type(), PieceType::BlackKing);
    }

    #[test]
    pub fn piece_is_king() {
        let mut pce = piece::WHITE_KING;
        assert!(pce.is_king());

        pce = piece::BLACK_KING;
        assert!(pce.is_king());
    }

    #[test]
    pub fn piece_is_rook() {
        let mut pce = piece::WHITE_ROOK;
        assert!(pce.is_rook());

        pce = piece::BLACK_ROOK;
        assert!(pce.is_rook());
    }

    #[test]
    pub fn piece_is_pawn() {
        let mut pce = piece::WHITE_PAWN;
        assert!(pce.is_pawn());

        pce = piece::BLACK_PAWN;
        assert!(pce.is_pawn());
    }
}
