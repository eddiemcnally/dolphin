#[allow(dead_code)]

use piece::Piece;
use piece::Colour;
use square::Square;
use square::rank::Rank;
use square::file::File;
use position::CastlePermissionBitMap;
use position::CastlePermission;
use std::mem::transmute;
use std::collections::HashMap;


#[derive(Default)]
pub struct ParsedFen {
    piece_positions: HashMap<Square, Piece>,
    side_to_move: Colour,
    castle_perm: Option<CastlePermission>,
    en_pass_sq: Option<Square>,
    half_move_cnt: u16,
    full_move_cnt: u16,
}


// [0] = piece positions
// [1] = side to move
// [2] = castle permissions
// [3] = en passant square (or '-' if no en passant)
// [4] = half-move clock
// [5] = full move number

const FEN_BOARD: usize = 0;
const FEN_SIDE_TO_MOVE: usize = 1;
const FEN_CASTLE_PERMISSIONS: usize = 2;
const FEN_EN_PASSANT: usize = 3;
const FEN_HALF_MOVE: usize = 4;
const FEN_FULL_MOVE: usize = 5;


/// parses a FEN string and populates the given board
///
/// Sample FEN:
///      rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
///
pub fn get_position(fen: &str) -> ParsedFen {

    let mut retval: ParsedFen = Default::default();

    let piece_pos: Vec<&str> = fen.split(' ').collect();

    retval.piece_positions = extract_piece_locations(piece_pos[FEN_BOARD]);
    retval.side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
    retval.castle_perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
    retval.en_pass_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]);
    retval.half_move_cnt = get_half_move_clock(piece_pos[FEN_HALF_MOVE]);
    retval.full_move_cnt = get_full_move_number(piece_pos[FEN_FULL_MOVE]);

    return retval;
}

/// takes the list of ranks (starting at rank 8)
pub fn extract_piece_locations(pieces: &str) -> HashMap<Square, Piece> {
    let ranks: Vec<_> = pieces.split('/').collect();
    let mut retval: HashMap<Square, Piece> = HashMap::new();
    for (rank, pieces) in ranks.iter().rev().enumerate() {
        let mut file: u8 = 0;

        for c in pieces.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    // it's a number, so incr the file
                    file = file + n as u8;
                }
                None => {
                    // not a number, so it's a piece
                    match Piece::from_char(c) {
                        Some(piece) => {
                            let r: Rank = unsafe { transmute(rank as u8) };
                            let f: File = unsafe { transmute(file as u8) };

                            let sq: Square = Square::get_square(r, f);
                            file += 1;

                            retval.insert(sq, piece);
                        }
                        None => panic!("Unexpected FEN piece. Parsed character '{c}'"),
                    }
                }
            }
        }
    }
    return retval;
}


fn get_side_to_move(side: &str) -> Colour {
    match side.trim() {
        "w" => Colour::White,
        "b" => Colour::Black,
        _ => panic!("Unexpected side-to-move. Parsed character '{}'", side),
    }
}


fn get_en_passant_sq(en_pass: &str) -> Option<Square> {
    if en_pass == "-" {
        None
    } else {
        Some(Square::get_from_string(en_pass))
    }
}


fn get_half_move_clock(half_cnt: &str) -> u16 {
    return half_cnt.parse::<u16>().unwrap();
}


fn get_full_move_number(full_move_num: &str) -> u16 {
    return full_move_num.parse::<u16>().unwrap();
}


fn get_castle_permissions(castleperm: &str) -> Option<CastlePermission> {
    if castleperm.trim() == "-" {
        None
    } else {
        let mut cp: CastlePermission = 0;
        if castleperm.contains("K") {
            cp = CastlePermissionBitMap::set_perm(CastlePermissionBitMap::WK, cp);
        }
        if castleperm.contains("Q") {
            cp = CastlePermissionBitMap::set_perm(CastlePermissionBitMap::WQ, cp);
        }
        if castleperm.contains("k") {
            cp = CastlePermissionBitMap::set_perm(CastlePermissionBitMap::BK, cp);
        }
        if castleperm.contains("q") {
            cp = CastlePermissionBitMap::set_perm(CastlePermissionBitMap::BQ, cp);
        }
        Some(cp)
    }
}


#[cfg(test)]
mod tests {
    use super::Square;
    use super::Piece;
    use super::Rank;
    use super::File;
    use super::Colour;
    use super::FEN_BOARD;
    use super::FEN_SIDE_TO_MOVE;
    use super::FEN_CASTLE_PERMISSIONS;
    use super::FEN_EN_PASSANT;
    use super::FEN_HALF_MOVE;
    use super::FEN_FULL_MOVE;
    use fen::extract_piece_locations;
    use fen::get_side_to_move;
    use fen::get_castle_permissions;
    use fen::get_half_move_clock;
    use fen::get_full_move_number;
    use std::collections::HashMap;
    use position::CastlePermissionBitMap;

    #[test]
    pub fn test_piece_positions() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();

        let sq_pce = extract_piece_locations(piece_pos[FEN_BOARD]);

        assert_eq!(sq_pce.len(), 32);

        assert_eq!(sq_pce[&Square::a1], Piece::WPawn);
        assert_eq!(sq_pce[&Square::d1], Piece::BQueen);
        assert_eq!(sq_pce[&Square::h1], Piece::BKnight);

        assert_eq!(sq_pce[&Square::a2], Piece::WPawn);
        assert_eq!(sq_pce[&Square::c2], Piece::BRook);
        assert_eq!(sq_pce[&Square::e2], Piece::BRook);
        assert_eq!(sq_pce[&Square::f2], Piece::WPawn);
        assert_eq!(sq_pce[&Square::h2], Piece::WPawn);

        assert_eq!(sq_pce[&Square::a3], Piece::BPawn);
        assert_eq!(sq_pce[&Square::b3], Piece::WPawn);
        assert_eq!(sq_pce[&Square::c3], Piece::WRook);
        assert_eq!(sq_pce[&Square::e3], Piece::WKnight);
        assert_eq!(sq_pce[&Square::f3], Piece::BPawn);

        assert_eq!(sq_pce[&Square::b4], Piece::WRook);
        assert_eq!(sq_pce[&Square::c4], Piece::WBishop);
        assert_eq!(sq_pce[&Square::f4], Piece::WPawn);

        assert_eq!(sq_pce[&Square::b5], Piece::WBishop);
        assert_eq!(sq_pce[&Square::e5], Piece::WPawn);
        assert_eq!(sq_pce[&Square::g5], Piece::WKing);

        assert_eq!(sq_pce[&Square::a6], Piece::WKnight);
        assert_eq!(sq_pce[&Square::c6], Piece::BPawn);
        assert_eq!(sq_pce[&Square::h6], Piece::BPawn);

        assert_eq!(sq_pce[&Square::b7], Piece::WPawn);
        assert_eq!(sq_pce[&Square::c7], Piece::BPawn);
        assert_eq!(sq_pce[&Square::d7], Piece::BPawn);
        assert_eq!(sq_pce[&Square::e7], Piece::WQueen);
        assert_eq!(sq_pce[&Square::f7], Piece::BPawn);
        assert_eq!(sq_pce[&Square::g7], Piece::BBishop);

        assert_eq!(sq_pce[&Square::b8], Piece::BKnight);
        assert_eq!(sq_pce[&Square::d8], Piece::BKing);
        assert_eq!(sq_pce[&Square::g8], Piece::BBishop);
        assert_eq!(sq_pce[&Square::h8], Piece::BPawn);
    }

    #[test]
    pub fn test_side_to_move_white() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
        assert_eq!(side_to_move, Colour::White);
    }
    #[test]
    pub fn test_side_to_move_black() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
        assert_eq!(side_to_move, Colour::Black);
    }
    #[test]
    #[should_panic]
    pub fn test_side_to_move_invalid_panics() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n X - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
    }


    #[test]
    pub fn test_castle_permissions_white_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b K - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
        assert_eq!(CastlePermissionBitMap::WK as u8, perm.unwrap());
    }
    #[test]
    pub fn test_castle_permissions_black_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b k - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
        assert_eq!(CastlePermissionBitMap::BK as u8, perm.unwrap());
    }
    #[test]
    pub fn test_castle_permissions_black_queenside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
        assert_eq!(CastlePermissionBitMap::BQ as u8, perm.unwrap());
    }

    #[test]
    pub fn test_castle_permissions_white_kingside_queenside_black_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b KQk - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        let isWK =
            CastlePermissionBitMap::is_perm_set(CastlePermissionBitMap::WK, perm.unwrap() as u8);
        assert_eq!(isWK, true);
        let isWQ =
            CastlePermissionBitMap::is_perm_set(CastlePermissionBitMap::WQ, perm.unwrap() as u8);
        assert_eq!(isWQ, true);
        let isBK =
            CastlePermissionBitMap::is_perm_set(CastlePermissionBitMap::BK, perm.unwrap() as u8);
        assert_eq!(isBK, true);
        let isBQ =
            CastlePermissionBitMap::is_perm_set(CastlePermissionBitMap::BQ, perm.unwrap() as u8);
        assert_eq!(isBQ, false);
    }

    #[test]
    pub fn test_parse_half_move_clock() {
        let mut fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 1";
        let mut piece_pos: Vec<&str> = fen.split(' ').collect();
        let mut half_clock = get_half_move_clock(piece_pos[FEN_HALF_MOVE]);
        assert_eq!(half_clock, 0);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 22 1";
        piece_pos = fen.split(' ').collect();
        half_clock = get_half_move_clock(piece_pos[FEN_HALF_MOVE]);
        assert_eq!(half_clock, 22);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 5 1";
        piece_pos = fen.split(' ').collect();
        half_clock = get_half_move_clock(piece_pos[FEN_HALF_MOVE]);
        assert_eq!(half_clock, 5);

    }


    #[test]
    pub fn test_parse_full_move_count() {
        let mut fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 0";
        let mut piece_pos: Vec<&str> = fen.split(' ').collect();
        let mut full_move_cnt = get_full_move_number(piece_pos[FEN_FULL_MOVE]);
        assert_eq!(full_move_cnt, 0);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 1";
        piece_pos = fen.split(' ').collect();
        full_move_cnt = get_full_move_number(piece_pos[FEN_FULL_MOVE]);
        assert_eq!(full_move_cnt, 1);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 55";
        piece_pos = fen.split(' ').collect();
        full_move_cnt = get_full_move_number(piece_pos[FEN_FULL_MOVE]);
        assert_eq!(full_move_cnt, 55);
    }



}
