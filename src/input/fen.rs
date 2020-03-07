use board::piece::Colour;
use board::piece::Piece;
use board::square::file::File;
use board::square::rank::Rank;
use board::square::Square;
use position::castle_permissions;
use position::castle_permissions::CastlePermission;
use std::collections::HashMap;

#[derive(Default)]
pub struct ParsedFen {
    pub piece_positions: HashMap<Square, Piece>,
    pub side_to_move: Colour,
    pub castle_perm: CastlePermission,
    pub en_pass_sq: Option<Square>,
    pub half_move_cnt: u16,
    pub full_move_cnt: u16,
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
                    let piece = Piece::from_char(c);

                    let r: Rank = Rank::from_num(rank as u8);
                    let f: File = File::from_num(file);

                    let sq: Square = Square::get_square(r, f);
                    file += 1;

                    retval.insert(sq, piece);
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

fn get_castle_permissions(castleperm: &str) -> CastlePermission {
    let mut cp = castle_permissions::NO_CASTLE_PERMS;
    if castleperm.trim() != "-" {
        if castleperm.contains("K") {
            castle_permissions::set_white_king(&mut cp);
        }
        if castleperm.contains("Q") {
            castle_permissions::set_white_queen(&mut cp);
        }
        if castleperm.contains("k") {
            castle_permissions::set_black_king(&mut cp);
        }
        if castleperm.contains("q") {
            castle_permissions::set_black_queen(&mut cp);
        }
    }
    return cp;
}

#[cfg(test)]
mod tests {
    use super::FEN_BOARD;
    use super::FEN_CASTLE_PERMISSIONS;
    use super::FEN_EN_PASSANT;
    use super::FEN_FULL_MOVE;
    use super::FEN_HALF_MOVE;
    use super::FEN_SIDE_TO_MOVE;
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;
    use board::square::Square;
    use fen::extract_piece_locations;
    use fen::get_castle_permissions;
    use fen::get_en_passant_sq;
    use fen::get_full_move_number;
    use fen::get_half_move_clock;
    use fen::get_side_to_move;
    use position::castle_permissions;

    #[test]
    pub fn piece_positions() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();

        let sq_pce = extract_piece_locations(piece_pos[FEN_BOARD]);

        assert_eq!(sq_pce.len(), 32);

        assert_eq!(
            sq_pce[&Square::a1],
            Piece::new(PieceRole::Pawn, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::d1],
            Piece::new(PieceRole::Queen, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::h1],
            Piece::new(PieceRole::Knight, Colour::Black)
        );

        assert_eq!(
            sq_pce[&Square::a2],
            Piece::new(PieceRole::Pawn, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::c2],
            Piece::new(PieceRole::Rook, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::e2],
            Piece::new(PieceRole::Rook, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::f2],
            Piece::new(PieceRole::Pawn, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::h2],
            Piece::new(PieceRole::Pawn, Colour::White)
        );

        assert_eq!(
            sq_pce[&Square::a3],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::b3],
            Piece::new(PieceRole::Pawn, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::c3],
            Piece::new(PieceRole::Rook, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::e3],
            Piece::new(PieceRole::Knight, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::f3],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );

        assert_eq!(
            sq_pce[&Square::b4],
            Piece::new(PieceRole::Rook, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::c4],
            Piece::new(PieceRole::Bishop, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::f4],
            Piece::new(PieceRole::Pawn, Colour::White)
        );

        assert_eq!(
            sq_pce[&Square::b5],
            Piece::new(PieceRole::Bishop, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::e5],
            Piece::new(PieceRole::Pawn, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::g5],
            Piece::new(PieceRole::King, Colour::White)
        );

        assert_eq!(
            sq_pce[&Square::a6],
            Piece::new(PieceRole::Knight, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::c6],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::h6],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );

        assert_eq!(
            sq_pce[&Square::b7],
            Piece::new(PieceRole::Pawn, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::c7],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::d7],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::e7],
            Piece::new(PieceRole::Queen, Colour::White)
        );
        assert_eq!(
            sq_pce[&Square::f7],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::g7],
            Piece::new(PieceRole::Bishop, Colour::Black)
        );

        assert_eq!(
            sq_pce[&Square::b8],
            Piece::new(PieceRole::Knight, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::d8],
            Piece::new(PieceRole::King, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::g8],
            Piece::new(PieceRole::Bishop, Colour::Black)
        );
        assert_eq!(
            sq_pce[&Square::h8],
            Piece::new(PieceRole::Pawn, Colour::Black)
        );
    }

    #[test]
    pub fn pieces_edge_squares_h1() {
        let fen = "8/8/8/8/8/8/6N1/5N1k w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let sq_pce = extract_piece_locations(piece_pos[FEN_BOARD]);

        let pce = sq_pce[&Square::h1];
        assert_eq!(pce, Piece::new(PieceRole::King, Colour::Black));
    }

    #[test]
    pub fn pieces_edge_squares_h8() {
        let fen = "7k/8/8/8/8/8/6N1/5N2 w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let sq_pce = extract_piece_locations(piece_pos[FEN_BOARD]);

        let pce = sq_pce[&Square::h8];
        assert_eq!(pce, Piece::new(PieceRole::King, Colour::Black));
    }

    #[test]
    pub fn pieces_edge_squares_a1() {
        let fen = "8/8/8/8/8/8/6N1/k4N2 w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let sq_pce = extract_piece_locations(piece_pos[FEN_BOARD]);

        let pce = sq_pce[&Square::a1];
        assert_eq!(pce, Piece::new(PieceRole::King, Colour::Black));
    }

    #[test]
    pub fn pieces_edge_squares_a8() {
        let fen = "k7/8/8/8/8/8/6N1/5N2 w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let sq_pce = extract_piece_locations(piece_pos[FEN_BOARD]);

        let pce = sq_pce[&Square::a8];
        assert_eq!(pce, Piece::new(PieceRole::King, Colour::Black));
    }

    #[test]
    pub fn side_to_move_white() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
        assert_eq!(side_to_move, Colour::White);
    }
    #[test]
    pub fn side_to_move_black() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
        assert_eq!(side_to_move, Colour::Black);
    }
    #[test]
    #[should_panic]
    pub fn side_to_move_invalid_panics() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n X - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);
    }

    #[test]
    pub fn castle_permissions_white_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b K - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
        assert!(castle_permissions::is_white_king_set(perm) == true);
        assert!(castle_permissions::is_black_king_set(perm) == false);
        assert!(castle_permissions::is_white_queen_set(perm) == false);
        assert!(castle_permissions::is_black_queen_set(perm) == false);
        assert!(castle_permissions::has_castle_permission(perm) == true);
    }
    #[test]
    pub fn castle_permissions_white_queenside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b Q - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(castle_permissions::is_white_king_set(perm) == false);
        assert!(castle_permissions::is_black_king_set(perm) == false);
        assert!(castle_permissions::is_white_queen_set(perm) == true);
        assert!(castle_permissions::is_black_queen_set(perm) == false);
        assert!(castle_permissions::has_castle_permission(perm) == true);
    }
    #[test]
    pub fn castle_permissions_black_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b k - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(castle_permissions::is_white_king_set(perm) == false);
        assert!(castle_permissions::is_black_king_set(perm) == true);
        assert!(castle_permissions::is_white_queen_set(perm) == false);
        assert!(castle_permissions::is_black_queen_set(perm) == false);
        assert!(castle_permissions::has_castle_permission(perm) == true);
    }
    #[test]
    pub fn castle_permissions_black_queenside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(castle_permissions::is_white_king_set(perm) == false);
        assert!(castle_permissions::is_black_king_set(perm) == false);
        assert!(castle_permissions::is_white_queen_set(perm) == false);
        assert!(castle_permissions::is_black_queen_set(perm) == true);
        assert!(castle_permissions::has_castle_permission(perm) == true);
    }

    #[test]
    pub fn castle_permissions_none() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
        assert!(castle_permissions::has_castle_permission(perm) == false);
    }

    #[test]
    pub fn castle_permissions_white_kingside_queenside_black_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b KQk - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(castle_permissions::is_white_king_set(perm) == true);
        assert!(castle_permissions::is_black_king_set(perm) == true);
        assert!(castle_permissions::is_white_queen_set(perm) == true);
        assert!(castle_permissions::is_black_queen_set(perm) == false);
        assert!(castle_permissions::has_castle_permission(perm) == true);
    }

    #[test]
    pub fn parse_half_move_clock() {
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
    pub fn parse_full_move_count() {
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

    #[test]
    pub fn parse_en_passant() {
        let mut fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q c6 0 0";
        let mut piece_pos: Vec<&str> = fen.split(' ').collect();
        let mut enp_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]).unwrap();
        assert_eq!(enp_sq, Square::c6);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q c3 0 0";
        piece_pos = fen.split(' ').collect();
        enp_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]).unwrap();
        assert_eq!(enp_sq, Square::c3);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 0";
        piece_pos = fen.split(' ').collect();
        let no_enp_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]);
        assert_eq!(no_enp_sq.is_some(), false);
    }
}
