use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square::File;
use board::square::Rank;
use board::square::Square;
use position::castle_permissions::CastlePermissionType;
use std::collections::HashMap;
use std::vec::Vec;

pub fn get_all_castle_permissions() -> Vec<CastlePermissionType> {
    let mut list: Vec<CastlePermissionType> = Vec::new();

    list.push(CastlePermissionType::BlackKing);
    list.push(CastlePermissionType::BlackQueen);
    list.push(CastlePermissionType::WhiteKing);
    list.push(CastlePermissionType::WhiteQueen);
    list
}

pub fn get_all_pieces() -> Vec<Piece> {
    let mut list: Vec<Piece> = Vec::new();

    let mut pce: Piece;

    pce = Piece::new(PieceRole::Pawn, Colour::White);
    list.push(pce);
    pce = Piece::new(PieceRole::Knight, Colour::White);
    list.push(pce);
    pce = Piece::new(PieceRole::Bishop, Colour::White);
    list.push(pce);
    pce = Piece::new(PieceRole::Rook, Colour::White);
    list.push(pce);
    pce = Piece::new(PieceRole::Queen, Colour::White);
    list.push(pce);
    pce = Piece::new(PieceRole::King, Colour::White);
    list.push(pce);
    pce = Piece::new(PieceRole::Pawn, Colour::Black);
    list.push(pce);
    pce = Piece::new(PieceRole::Knight, Colour::Black);
    list.push(pce);
    pce = Piece::new(PieceRole::Bishop, Colour::Black);
    list.push(pce);
    pce = Piece::new(PieceRole::Rook, Colour::Black);
    list.push(pce);
    pce = Piece::new(PieceRole::Queen, Colour::Black);
    list.push(pce);
    pce = Piece::new(PieceRole::King, Colour::Black);
    list.push(pce);

    list
}

// TODO : move into square.rs
pub fn get_ordered_square_list_by_file() -> Vec<Square> {
    let mut list: Vec<Square> = Vec::new();

    list.push(Square::a1);
    list.push(Square::b1);
    list.push(Square::c1);
    list.push(Square::d1);
    list.push(Square::e1);
    list.push(Square::f1);
    list.push(Square::g1);
    list.push(Square::h1);

    list.push(Square::a2);
    list.push(Square::b2);
    list.push(Square::c2);
    list.push(Square::d2);
    list.push(Square::e2);
    list.push(Square::f2);
    list.push(Square::g2);
    list.push(Square::h2);

    list.push(Square::a3);
    list.push(Square::b3);
    list.push(Square::c3);
    list.push(Square::d3);
    list.push(Square::e3);
    list.push(Square::f3);
    list.push(Square::g3);
    list.push(Square::h3);

    list.push(Square::a4);
    list.push(Square::b4);
    list.push(Square::c4);
    list.push(Square::d4);
    list.push(Square::e4);
    list.push(Square::f4);
    list.push(Square::g4);
    list.push(Square::h4);

    list.push(Square::a5);
    list.push(Square::b5);
    list.push(Square::c5);
    list.push(Square::d5);
    list.push(Square::e5);
    list.push(Square::f5);
    list.push(Square::g5);
    list.push(Square::h5);

    list.push(Square::a6);
    list.push(Square::b6);
    list.push(Square::c6);
    list.push(Square::d6);
    list.push(Square::e6);
    list.push(Square::f6);
    list.push(Square::g6);
    list.push(Square::h6);

    list.push(Square::a7);
    list.push(Square::b7);
    list.push(Square::c7);
    list.push(Square::d7);
    list.push(Square::e7);
    list.push(Square::f7);
    list.push(Square::g7);
    list.push(Square::h7);

    list.push(Square::a8);
    list.push(Square::b8);
    list.push(Square::c8);
    list.push(Square::d8);
    list.push(Square::e8);
    list.push(Square::f8);
    list.push(Square::g8);
    list.push(Square::h8);

    list
}

pub fn get_square_rank_file_map() -> HashMap<Square, (Rank, File)> {
    let mut map: HashMap<Square, (Rank, File)> = HashMap::new();

    map.insert(Square::a1, (Rank::Rank1, File::FileA));
    map.insert(Square::a2, (Rank::Rank2, File::FileA));
    map.insert(Square::a3, (Rank::Rank3, File::FileA));
    map.insert(Square::a4, (Rank::Rank4, File::FileA));
    map.insert(Square::a5, (Rank::Rank5, File::FileA));
    map.insert(Square::a6, (Rank::Rank6, File::FileA));
    map.insert(Square::a7, (Rank::Rank7, File::FileA));
    map.insert(Square::a8, (Rank::Rank8, File::FileA));

    map.insert(Square::b1, (Rank::Rank1, File::FileB));
    map.insert(Square::b2, (Rank::Rank2, File::FileB));
    map.insert(Square::b3, (Rank::Rank3, File::FileB));
    map.insert(Square::b4, (Rank::Rank4, File::FileB));
    map.insert(Square::b5, (Rank::Rank5, File::FileB));
    map.insert(Square::b6, (Rank::Rank6, File::FileB));
    map.insert(Square::b7, (Rank::Rank7, File::FileB));
    map.insert(Square::b8, (Rank::Rank8, File::FileB));

    map.insert(Square::c1, (Rank::Rank1, File::FileC));
    map.insert(Square::c2, (Rank::Rank2, File::FileC));
    map.insert(Square::c3, (Rank::Rank3, File::FileC));
    map.insert(Square::c4, (Rank::Rank4, File::FileC));
    map.insert(Square::c5, (Rank::Rank5, File::FileC));
    map.insert(Square::c6, (Rank::Rank6, File::FileC));
    map.insert(Square::c7, (Rank::Rank7, File::FileC));
    map.insert(Square::c8, (Rank::Rank8, File::FileC));

    map.insert(Square::d1, (Rank::Rank1, File::FileD));
    map.insert(Square::d2, (Rank::Rank2, File::FileD));
    map.insert(Square::d3, (Rank::Rank3, File::FileD));
    map.insert(Square::d4, (Rank::Rank4, File::FileD));
    map.insert(Square::d5, (Rank::Rank5, File::FileD));
    map.insert(Square::d6, (Rank::Rank6, File::FileD));
    map.insert(Square::d7, (Rank::Rank7, File::FileD));
    map.insert(Square::d8, (Rank::Rank8, File::FileD));

    map.insert(Square::e1, (Rank::Rank1, File::FileE));
    map.insert(Square::e2, (Rank::Rank2, File::FileE));
    map.insert(Square::e3, (Rank::Rank3, File::FileE));
    map.insert(Square::e4, (Rank::Rank4, File::FileE));
    map.insert(Square::e5, (Rank::Rank5, File::FileE));
    map.insert(Square::e6, (Rank::Rank6, File::FileE));
    map.insert(Square::e7, (Rank::Rank7, File::FileE));
    map.insert(Square::e8, (Rank::Rank8, File::FileE));

    map.insert(Square::f1, (Rank::Rank1, File::FileF));
    map.insert(Square::f2, (Rank::Rank2, File::FileF));
    map.insert(Square::f3, (Rank::Rank3, File::FileF));
    map.insert(Square::f4, (Rank::Rank4, File::FileF));
    map.insert(Square::f5, (Rank::Rank5, File::FileF));
    map.insert(Square::f6, (Rank::Rank6, File::FileF));
    map.insert(Square::f7, (Rank::Rank7, File::FileF));
    map.insert(Square::f8, (Rank::Rank8, File::FileF));

    map.insert(Square::g1, (Rank::Rank1, File::FileG));
    map.insert(Square::g2, (Rank::Rank2, File::FileG));
    map.insert(Square::g3, (Rank::Rank3, File::FileG));
    map.insert(Square::g4, (Rank::Rank4, File::FileG));
    map.insert(Square::g5, (Rank::Rank5, File::FileG));
    map.insert(Square::g6, (Rank::Rank6, File::FileG));
    map.insert(Square::g7, (Rank::Rank7, File::FileG));
    map.insert(Square::g8, (Rank::Rank8, File::FileG));

    map.insert(Square::h1, (Rank::Rank1, File::FileH));
    map.insert(Square::h2, (Rank::Rank2, File::FileH));
    map.insert(Square::h3, (Rank::Rank3, File::FileH));
    map.insert(Square::h4, (Rank::Rank4, File::FileH));
    map.insert(Square::h5, (Rank::Rank5, File::FileH));
    map.insert(Square::h6, (Rank::Rank6, File::FileH));
    map.insert(Square::h7, (Rank::Rank7, File::FileH));
    map.insert(Square::h8, (Rank::Rank8, File::FileH));

    map
}

static REVERSE_BITS: [u8; 256] = [
    0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0,
    0x08, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8,
    0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4,
    0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C, 0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC,
    0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52, 0xD2, 0x32, 0xB2, 0x72, 0xF2,
    0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA,
    0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6,
    0x0E, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE, 0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE,
    0x01, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61, 0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1,
    0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9, 0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9,
    0x05, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5,
    0x0D, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD,
    0x03, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3,
    0x0B, 0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB,
    0x07, 0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7,
    0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF,
];

pub fn reverse_byte(b: u8) -> u8 {
    REVERSE_BITS[b as usize]
}
