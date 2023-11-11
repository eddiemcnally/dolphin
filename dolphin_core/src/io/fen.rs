use crate::board::colour::Colour;
use crate::board::file::File;
use crate::board::game_board::Board;
use crate::board::piece::Piece;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::position::castle_permissions::CastlePermission;
use crate::position::move_counter::MoveCounter;

// FEN fields
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

/// Parses a FEN string and returns populated structs
///
/// Sample FEN:
///      rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
///
///
pub fn decompose_fen(fen: &str) -> (Board, MoveCounter, CastlePermission, Colour, Option<Square>) {
    // split FEN into fields
    let piece_pos: Vec<&str> = fen.split(' ').collect();

    let board = extract_board_from_fen(piece_pos[FEN_BOARD]);
    let move_cntr = MoveCounter::new(
        get_half_move_clock(piece_pos[FEN_HALF_MOVE]),
        get_full_move_number(piece_pos[FEN_FULL_MOVE]),
    );
    let side_to_move = get_side_to_move(piece_pos[FEN_SIDE_TO_MOVE]);

    let castle_permissions = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
    let en_pass_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]);

    (
        board,
        move_cntr,
        castle_permissions,
        side_to_move,
        en_pass_sq,
    )
}

/// takes the list of ranks (starting at rank 8)
fn extract_board_from_fen(pieces: &str) -> Board {
    let ranks: Vec<_> = pieces.split('/').collect();
    let mut retval: Board = Board::new();

    for (rank, pieces) in ranks.iter().rev().enumerate() {
        let mut file = 0;

        for c in pieces.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    // it's a number, so incr the file
                    file += n;
                }
                None => {
                    // not a number, so it's a piece
                    if let Some((piece, colour)) = Piece::from_char(c) {
                        let r = Rank::new(rank as u8);

                        let f = File::new(file as u8);
                        let sq: Square = Square::from_rank_file(r, f);

                        file += 1;
                        retval.add_piece(piece, colour, sq);
                    }
                }
            }
        }
    }
    retval
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
        Some(Square::get_from_string(en_pass).unwrap())
    }
}

fn get_half_move_clock(half_cnt: &str) -> u16 {
    half_cnt.parse::<u16>().unwrap()
}

fn get_full_move_number(full_move_num: &str) -> u16 {
    full_move_num.parse::<u16>().unwrap()
}

fn get_castle_permissions(castleperm: &str) -> CastlePermission {
    let mut cp = CastlePermission::NO_CASTLE_PERMS_AVAIL;
    if castleperm.trim() != "-" {
        if castleperm.contains('K') {
            cp.set_white_king();
        }
        if castleperm.contains('Q') {
            cp.set_white_queen();
        }
        if castleperm.contains('k') {
            cp.set_black_king();
        }
        if castleperm.contains('q') {
            cp.set_black_queen();
        }
    }
    cp
}

#[cfg(test)]
mod tests {
    use super::get_castle_permissions;
    use super::get_en_passant_sq;
    use super::get_full_move_number;
    use super::get_half_move_clock;
    use super::get_side_to_move;
    use super::FEN_CASTLE_PERMISSIONS;
    use super::FEN_EN_PASSANT;
    use super::FEN_FULL_MOVE;
    use super::FEN_HALF_MOVE;
    use super::FEN_SIDE_TO_MOVE;
    use crate::board::colour::Colour;
    use crate::board::square::*;

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
        assert!(perm.is_white_king_set());
        assert!(!perm.is_black_king_set());
        assert!(!perm.is_white_queen_set());
        assert!(!perm.is_black_queen_set());
        assert!(perm.has_castle_permission());
    }
    #[test]
    pub fn castle_permissions_white_queenside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b Q - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(!perm.is_white_king_set());
        assert!(!perm.is_black_king_set());
        assert!(perm.is_white_queen_set());
        assert!(!perm.is_black_queen_set());
        assert!(perm.has_castle_permission());
    }
    #[test]
    pub fn castle_permissions_black_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b k - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(!perm.is_white_king_set());
        assert!(perm.is_black_king_set());
        assert!(!perm.is_white_queen_set());
        assert!(!perm.is_black_queen_set());
        assert!(perm.has_castle_permission());
    }
    #[test]
    pub fn castle_permissions_black_queenside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(!perm.is_white_king_set());
        assert!(!perm.is_black_king_set());
        assert!(!perm.is_white_queen_set());
        assert!(perm.is_black_queen_set());
        assert!(perm.has_castle_permission());
    }

    #[test]
    pub fn castle_permissions_none() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b - - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);
        assert!(!perm.has_castle_permission());
    }

    #[test]
    pub fn castle_permissions_white_kingside_queenside_black_kingside() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b KQk - 0 1";
        let piece_pos: Vec<&str> = fen.split(' ').collect();
        let perm = get_castle_permissions(piece_pos[FEN_CASTLE_PERMISSIONS]);

        assert!(perm.is_white_king_set());
        assert!(perm.is_black_king_set());
        assert!(perm.is_white_queen_set());
        assert!(!perm.is_black_queen_set());
        assert!(perm.has_castle_permission());
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
        assert_eq!(enp_sq, Square::C6);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q c3 0 0";
        piece_pos = fen.split(' ').collect();
        enp_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]).unwrap();
        assert_eq!(enp_sq, Square::C3);

        fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b q - 0 0";
        piece_pos = fen.split(' ').collect();
        let no_enp_sq = get_en_passant_sq(piece_pos[FEN_EN_PASSANT]);
        assert!(no_enp_sq.is_none());
    }
}
