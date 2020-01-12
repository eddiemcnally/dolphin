use board::board::Board;
use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square::Square;
use input::fen::ParsedFen;
use moves::mov::Mov;
use position::attack_checker;
use position::castle_permissions::CastlePermission;
use position::hash;
use position::hash::PositionHash;
use position::position_history::PositionHistory;

pub struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

static CASTLE_SQUARES_KING_WHITE: [Square; 3] = [Square::e1, Square::f1, Square::g1];

static CASTLE_SQUARES_QUEEN_WHITE: [Square; 4] = [Square::b1, Square::c1, Square::d1, Square::e1];

static CASTLE_SQUARES_KING_BLACK: [Square; 3] = [Square::e8, Square::f8, Square::g8];

static CASTLE_SQUARES_QUEEN_BLACK: [Square; 4] = [Square::b8, Square::c8, Square::d8, Square::e8];

const MAX_MOVE_HISTORY: u16 = 2048;

pub struct Position {
    board: Board,
    side_to_move: Colour,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
    move_cntr: MoveCounter,
    fifty_move_cntr: u8,
    position_key: PositionHash,
    position_history: PositionHistory,
}

impl Position {
    pub fn new(parsed_fen: ParsedFen) -> Position {
        let mv_cntr = MoveCounter {
            half_move: parsed_fen.half_move_cnt,
            full_move: parsed_fen.full_move_cnt,
        };

        Position {
            board: Board::from_fen(&parsed_fen),
            side_to_move: parsed_fen.side_to_move,
            en_pass_sq: parsed_fen.en_pass_sq,
            castle_perm: parsed_fen.castle_perm,
            move_cntr: mv_cntr,
            fifty_move_cntr: 0,
            position_history: PositionHistory::new(MAX_MOVE_HISTORY),
            position_key: hash::generate_from_fen(&parsed_fen),
        }
    }

    pub fn side_to_move(&self) -> Colour {
        self.side_to_move
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_pass_sq
    }

    pub fn castle_permissions(&self) -> CastlePermission {
        self.castle_perm
    }

    pub fn move_counter(&self) -> &MoveCounter {
        &self.move_cntr
    }

    pub fn flip_side_to_move(&mut self) {
        match self.side_to_move {
            Colour::White => self.side_to_move = Colour::Black,
            Colour::Black => self.side_to_move = Colour::White,
        }
    }

    //
    //  ---- 0000 Quiet move
    //  ---- 0001 Double Pawn push
    //  ---- 0010 King Castle
    //  ---- 0011 Queen Castle
    //  ---- 0100 Capture
    //  ---- 0101 En Passant Capture
    //  ---- 1000 Promotion Knight
    //  ---- 1001 Promotion Bishop
    //  ---- 1010 Promotion Rook
    //  ---- 1011 Promotion Queen
    //  ---- 1100 Promotion Knight Capture
    //  ---- 1101 Promotion Bishop Capture
    //  ---- 1110 Promotion Rook Capture
    //  ---- 1111 Promotion Queen Capture

    pub fn make_move(&mut self, mv: Mov) -> bool {
        self.position_history.push(
            self.position_key,
            mv,
            self.fifty_move_cntr,
            self.en_pass_sq,
            self.castle_perm,
        );

        self.flip_side_to_move();
        hash::update_side(&mut self.position_key);

        self.move_cntr.half_move += 1;
        self.move_cntr.full_move += 1;

        // set up some general variables
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();
        let piece = self.board().get_piece_on_square(from_sq).unwrap();
        let side_to_move = self.side_to_move;

        handle_50_move_rule(self, mv, piece);

        // make the move
        if mv.is_quiet() {
            update_hash_on_piece_move(self, piece, from_sq, to_sq);
            self.board.move_piece(from_sq, to_sq, piece);
        } else if mv.is_double_pawn() {
            do_double_pawn_move(self, side_to_move, piece, from_sq, to_sq);
        } else if mv.is_king_castle() {
            do_castle_move_king(self, side_to_move);
        } else if mv.is_queen_castle() {
            do_castle_move_queen(self, side_to_move);
        } else if mv.is_en_passant() {
            do_en_passant(self, from_sq, to_sq);
        } else if mv.is_promote() {
            do_promotion(self, mv, from_sq, to_sq, side_to_move);
        } else if mv.is_capture() {
            do_capture_move(self, piece, from_sq, to_sq);
        }

        update_en_passant_sq(self, mv);

        // check if move results in king being in check
        let king_sq = self.board().get_king_sq(side_to_move);
        if attack_checker::is_sq_attacked(self.board(), king_sq, side_to_move) {
            return false;
        }

        // check castle through attacked squares (or king was in check before the castle move)
        if mv.is_castle() {
            let is_valid = self.is_castle_legal(mv, side_to_move);
            if is_valid == false {
                return false;
            }
        }

        return true;
    }

    fn is_castle_legal(&self, mv: Mov, side_to_move: Colour) -> bool {
        if mv.is_king_castle() {
            match side_to_move {
                Colour::White => {
                    return self.is_castle_through_attacked_squares(
                        side_to_move,
                        &CASTLE_SQUARES_KING_WHITE,
                    );
                }
                Colour::Black => {
                    return self.is_castle_through_attacked_squares(
                        side_to_move,
                        &CASTLE_SQUARES_KING_BLACK,
                    );
                }
            }
        } else if mv.is_queen_castle() {
            match side_to_move {
                Colour::White => {
                    return self.is_castle_through_attacked_squares(
                        side_to_move,
                        &CASTLE_SQUARES_QUEEN_WHITE,
                    );
                }
                Colour::Black => {
                    return self.is_castle_through_attacked_squares(
                        side_to_move,
                        &CASTLE_SQUARES_QUEEN_BLACK,
                    );
                }
            }
        } else {
            panic!("Invalid move test");
        }
    }

    fn is_castle_through_attacked_squares(&self, side_to_move: Colour, sq_list: &[Square]) -> bool {
        for sq in sq_list {
            let is_valid = attack_checker::is_sq_attacked(self.board(), *sq, side_to_move);
            if is_valid == false {
                return false;
            }
        }

        return true;
    }
}

fn find_en_passant_sq(from_sq: Square, col: Colour) -> Square {
    // use the *from_sq* to find the en passant sq
    match col {
        Colour::White => from_sq.square_plus_1_rank(),
        Colour::Black => from_sq.square_minus_1_rank(),
    }
}

fn remove_piece_from_board(position: &mut Position, pce: Piece, sq: Square) {
    position.board.remove_piece(pce, sq);
    hash::update_piece(&mut position.position_key, pce, sq);
}

fn add_piece_to_board(position: &mut Position, pce: Piece, sq: Square) {
    position.board.add_piece(pce, sq);
    hash::update_piece(&mut position.position_key, pce, sq);
}

fn update_hash_on_piece_move(position: &mut Position, pce: Piece, from_sq: Square, to_sq: Square) {
    hash::update_piece(&mut position.position_key, pce, from_sq);
    hash::update_piece(&mut position.position_key, pce, to_sq);
}

fn handle_50_move_rule(position: &mut Position, mv: Mov, pce_to_move: Piece) {
    if mv.is_capture() || pce_to_move.role() == PieceRole::Pawn {
        position.fifty_move_cntr = 0;
    } else {
        position.fifty_move_cntr += 1;
    }
}

fn update_en_passant_sq(position: &mut Position, mv: Mov) {
    // clear en passant
    if mv.is_double_pawn() == false {
        position.en_pass_sq = None;
    }
}

fn do_castle_move_king(position: &mut Position, col: Colour) {
    let (king_from_sq, king_to_sq) = match col {
        Colour::Black => (Square::e8, Square::g8),
        Colour::White => (Square::e1, Square::g1),
    };
    let (rook_from_sq, rook_to_sq) = match col {
        Colour::Black => (Square::h8, Square::f8),
        Colour::White => (Square::h1, Square::f1),
    };

    let king = Piece::new(PieceRole::King, col);
    update_hash_on_piece_move(position, king, king_from_sq, king_to_sq);
    position.board.move_piece(king_from_sq, king_to_sq, king);

    let rook = Piece::new(PieceRole::Rook, col);
    update_hash_on_piece_move(position, rook, rook_from_sq, rook_to_sq);
    position.board.move_piece(rook_from_sq, rook_to_sq, rook);

    position.castle_perm.set_king(col, false);
}

fn do_double_pawn_move(
    position: &mut Position,
    col: Colour,
    piece: Piece,
    from_sq: Square,
    to_sq: Square,
) {
    update_hash_on_piece_move(position, piece, from_sq, to_sq);
    position.board.move_piece(from_sq, to_sq, piece);
    let s = find_en_passant_sq(from_sq, col);
    position.en_pass_sq = Some(s);
}

fn do_castle_move_queen(position: &mut Position, col: Colour) {
    let (king_from_sq, king_to_sq) = match col {
        Colour::Black => (Square::e8, Square::c8),
        Colour::White => (Square::e1, Square::c1),
    };
    let (rook_from_sq, rook_to_sq) = match col {
        Colour::Black => (Square::a8, Square::d8),
        Colour::White => (Square::a1, Square::d1),
    };

    let king = Piece::new(PieceRole::King, col);
    update_hash_on_piece_move(position, king, king_from_sq, king_to_sq);
    position.board.move_piece(king_from_sq, king_to_sq, king);

    let rook = Piece::new(PieceRole::Rook, col);
    update_hash_on_piece_move(position, rook, rook_from_sq, rook_to_sq);
    position.board.move_piece(rook_from_sq, rook_to_sq, rook);

    position.castle_perm.set_queen(col, false);
}

fn do_en_passant(position: &mut Position, from_sq: Square, to_sq: Square) {
    let capt_sq = match position.side_to_move {
        Colour::White => to_sq.square_minus_1_rank(),
        Colour::Black => to_sq.square_plus_1_rank(),
    };

    let pawn = Piece::new(PieceRole::Pawn, position.side_to_move);
    let capt_pawn = Piece::new(PieceRole::Pawn, position.side_to_move.flip_side());

    remove_piece_from_board(position, capt_pawn, capt_sq);
    position.board.move_piece(from_sq, to_sq, pawn);
    update_hash_on_piece_move(position, pawn, from_sq, to_sq);
}

fn do_promotion(
    position: &mut Position,
    mv: Mov,
    from_sq: Square,
    to_sq: Square,
    side_to_move: Colour,
) {
    if mv.is_capture() {
        let capt_pce = position.board.get_piece_on_square(to_sq).unwrap();
        remove_piece_from_board(position, capt_pce, to_sq);
    }

    let source_pce = position.board.get_piece_on_square(from_sq).unwrap();
    let target_pce_role = mv.decode_promotion_piece_role();
    let target_pce = Piece::new(target_pce_role, side_to_move);

    remove_piece_from_board(position, source_pce, from_sq);
    add_piece_to_board(position, target_pce, to_sq);
}

fn do_capture_move(position: &mut Position, piece_to_move: Piece, from_sq: Square, to_sq: Square) {
    let capt_pce = position.board.get_piece_on_square(to_sq).unwrap();
    remove_piece_from_board(position, capt_pce, to_sq);
    update_hash_on_piece_move(position, piece_to_move, from_sq, to_sq);
    position.board.move_piece(from_sq, to_sq, piece_to_move);
}

#[cfg(test)]
mod tests {
    use board::piece::Colour;
    use board::piece::Piece;
    use board::piece::PieceRole;
    use board::square::Square;
    use input::fen;
    use moves::mov::Mov;
    use position::position::Position;

    #[test]
    pub fn make_move_quiet_piece_moved_hash_changed() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let before_hash = pos.position_key;

        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);

        // check before move
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e5,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));

        pos.make_move(mv);

        assert_eq!(pos.board().is_sq_empty(Square::e5), true);
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e6,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));
        assert_ne!(before_hash, pos.position_key);
    }
    #[test]
    pub fn make_move_history_updated() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        // initially no history
        assert_eq!(pos.position_history.len(), 0);
        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);
        pos.make_move(mv);

        // history updated
        assert_eq!(pos.position_history.len(), 1);
    }

    #[test]
    pub fn make_move_side_flipped() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        // initially correct side
        assert_eq!(pos.side_to_move, Colour::White);
        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);
        pos.make_move(mv);

        assert_eq!(pos.side_to_move, Colour::Black);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_reset_on_capture_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        // set to some value
        pos.fifty_move_cntr = 21;

        let mv = Mov::encode_move_capture(Square::b5, Square::c6);
        pos.make_move(mv);

        assert_eq!(0, pos.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_reset_on_pawn_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let pce_to_move = pos.board.get_piece_on_square(Square::e5).unwrap();
        assert_eq!(pce_to_move.role(), PieceRole::Pawn);

        // set to some value
        pos.fifty_move_cntr = 21;

        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);
        pos.make_move(mv);

        assert_eq!(0, pos.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_incremented_on_non_pawn_and_non_capture_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let pce_to_move = pos.board.get_piece_on_square(Square::c4).unwrap();
        assert_eq!(pce_to_move.role(), PieceRole::Bishop);

        // set to some value
        pos.fifty_move_cntr = 21;
        let expected_cntr_val = pos.fifty_move_cntr + 1;

        let mv = Mov::encode_move_quiet(Square::c4, Square::d5);
        pos.make_move(mv);

        assert_eq!(expected_cntr_val, pos.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_half_move_cntr_incremented() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let pce_to_move = pos.board.get_piece_on_square(Square::c4).unwrap();
        assert_eq!(pce_to_move.role(), PieceRole::Bishop);

        // set to some value
        pos.move_cntr.half_move = 21;
        pos.move_cntr.full_move = 32;

        let expected_half_move = pos.move_cntr.half_move + 1;
        let expected_full_move = pos.move_cntr.full_move + 1;

        let mv = Mov::encode_move_quiet(Square::c4, Square::d5);
        pos.make_move(mv);

        assert_eq!(expected_half_move, pos.move_cntr.half_move);
        assert_eq!(expected_full_move, pos.move_cntr.full_move);
    }

    #[test]
    pub fn make_move_double_pawn_move_en_passant_square_set_white_moves() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n b - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f2,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));

        // set to some value
        let mv = Mov::encode_move_double_pawn_first(Square::f2, Square::f4);
        pos.make_move(mv);

        assert_eq!(pos.en_pass_sq.unwrap(), Square::f3);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f4,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));

        assert!(is_sq_empty(&pos, Square::f2));
    }

    #[test]
    pub fn make_move_double_pawn_move_en_passant_square_set_black_moves() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n w - - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d7,
            Piece::new(PieceRole::Pawn, Colour::Black)
        ));

        // set to some value
        let mv = Mov::encode_move_double_pawn_first(Square::d7, Square::d5);
        pos.make_move(mv);

        assert_eq!(pos.en_pass_sq, Some(Square::d6));

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d5,
            Piece::new(PieceRole::Pawn, Colour::Black)
        ));

        assert!(is_sq_empty(&pos, Square::d7));
    }

    #[test]
    pub fn make_move_king_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(pos.castle_permissions().is_king_set(Colour::White));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e1,
            Piece::new(PieceRole::King, Colour::White)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::h1,
            Piece::new(PieceRole::Rook, Colour::White)
        ));
        let mv = Mov::encode_move_castle_kingside_white();
        pos.make_move(mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::e1));
        assert!(is_sq_empty(&pos, Square::h1));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::g1,
            Piece::new(PieceRole::King, Colour::White)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f1,
            Piece::new(PieceRole::Rook, Colour::White)
        ));

        assert!(pos.castle_permissions().is_king_set(Colour::White) == false);
    }

    #[test]
    pub fn make_move_king_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(pos.castle_permissions().is_king_set(Colour::Black));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e8,
            Piece::new(PieceRole::King, Colour::Black)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::h8,
            Piece::new(PieceRole::Rook, Colour::Black)
        ));
        let mv = Mov::encode_move_castle_kingside_black();
        pos.make_move(mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::e8));
        assert!(is_sq_empty(&pos, Square::h8));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::g8,
            Piece::new(PieceRole::King, Colour::Black)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f8,
            Piece::new(PieceRole::Rook, Colour::Black)
        ));

        assert!(pos.castle_permissions().is_king_set(Colour::Black) == false);
    }

    #[test]
    pub fn make_move_queen_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(pos.castle_permissions().is_queen_set(Colour::White));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e1,
            Piece::new(PieceRole::King, Colour::White)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::a1,
            Piece::new(PieceRole::Rook, Colour::White)
        ));
        let mv = Mov::encode_move_castle_queenside_white();
        pos.make_move(mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::e1));
        assert!(is_sq_empty(&pos, Square::a1));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::c1,
            Piece::new(PieceRole::King, Colour::White)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d1,
            Piece::new(PieceRole::Rook, Colour::White)
        ));

        assert!(pos.castle_permissions().is_queen_set(Colour::White) == false);
    }

    #[test]
    pub fn make_move_queen_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(pos.castle_permissions().is_queen_set(Colour::Black));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e8,
            Piece::new(PieceRole::King, Colour::Black)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::a8,
            Piece::new(PieceRole::Rook, Colour::Black)
        ));
        let mv = Mov::encode_move_castle_queenside_black();
        pos.make_move(mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::e8));
        assert!(is_sq_empty(&pos, Square::a8));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::c8,
            Piece::new(PieceRole::King, Colour::Black)
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d8,
            Piece::new(PieceRole::Rook, Colour::Black)
        ));

        assert!(pos.castle_permissions().is_queen_set(Colour::Black) == false);
    }

    #[test]
    pub fn make_move_en_passant_black() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/pPBP1P2/2R1NpP1/2r1r2P/R2q3n w - b3 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert_eq!(pos.en_passant_square(), Some(Square::b3));
        let mv = Mov::encode_move_en_passant(Square::a4, Square::b3);
        pos.make_move(mv);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::b3,
            Piece::new(PieceRole::Pawn, Colour::Black)
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::b4,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::a4,
            Piece::new(PieceRole::Pawn, Colour::Black)
        ));

        assert_eq!(pos.en_passant_square(), None);
    }

    #[test]
    pub fn make_move_en_passant_white() {
        let fen = "1n1k2bp/2p2pb1/1p5p/1B1pP1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n b - d6 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert_eq!(pos.en_passant_square(), Some(Square::d6));
        let mv = Mov::encode_move_en_passant(Square::e5, Square::d6);
        pos.make_move(mv);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d6,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::d5,
            Piece::new(PieceRole::Pawn, Colour::Black)
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::d5,
            Piece::new(PieceRole::Pawn, Colour::White)
        ));

        assert_eq!(pos.en_passant_square(), None);
    }

    #[test]
    pub fn make_move_promotion_capture_white_to_move() {
        let target_prom_pce = vec![
            PieceRole::Bishop,
            PieceRole::Knight,
            PieceRole::Queen,
            PieceRole::Rook,
        ];

        for target in target_prom_pce {
            let fen = "kn3b1p/2p1Pp2/1p5p/1B1pb1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n b - - 0 1";
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            // check pre-conditions
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::f8,
                Piece::new(PieceRole::Bishop, Colour::Black)
            ));

            let mv = Mov::encode_move_with_promotion_capture(Square::e7, Square::f8, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::e7));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::f8,
                Piece::new(target, Colour::White)
            ));
        }
    }

    #[test]
    pub fn make_move_promotion_capture_black_to_move() {
        let target_prom_pce = vec![
            PieceRole::Bishop,
            PieceRole::Knight,
            PieceRole::Queen,
            PieceRole::Rook,
        ];

        for target in target_prom_pce {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/2R5 w - - 0 1";
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            // check pre-conditions
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::c1,
                Piece::new(PieceRole::Rook, Colour::White)
            ));

            let mv = Mov::encode_move_with_promotion_capture(Square::d2, Square::c1, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::d2));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::c1,
                Piece::new(target, Colour::Black)
            ));
        }
    }

    #[test]
    pub fn make_move_promotion_black_to_move() {
        let target_prom_pce = vec![
            PieceRole::Bishop,
            PieceRole::Knight,
            PieceRole::Queen,
            PieceRole::Rook,
        ];

        for target in target_prom_pce {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 w - - 0 1";
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            // check pre-conditions
            assert!(is_sq_empty(&pos, Square::d1));

            let mv = Mov::encode_move_with_promotion(Square::d2, Square::d1, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::d2));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::d1,
                Piece::new(target, Colour::Black)
            ));
        }
    }

    #[test]
    pub fn make_move_promotion_white_to_move() {
        let target_prom_pce = vec![
            PieceRole::Bishop,
            PieceRole::Knight,
            PieceRole::Queen,
            PieceRole::Rook,
        ];

        for target in target_prom_pce {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 b - - 0 1";
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            // check pre-conditions
            assert!(is_sq_empty(&pos, Square::b8));

            let mv = Mov::encode_move_with_promotion(Square::b7, Square::b8, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::b7));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::b8,
                Piece::new(target, Colour::White)
            ));
        }
    }

    #[test]
    pub fn make_move_king_castle_white_through_attached_squares_is_illegal() {
        let fens = vec![
            "8/8/8/8/3b4/8/8/4K2R w K - 0 1",
            "8/8/8/8/8/2b5/8/4K2R w K - 0 1",
            "8/8/8/8/8/4b3/8/4K2R w K - 0 1",
            "8/8/8/8/8/3n4/8/4K2R w K - 0 1",
            "8/8/8/8/8/4n3/8/4K2R w K - 0 1",
            "8/8/8/8/8/5n2/8/4K2R w K - 0 1",
            "8/8/8/8/8/5n2/8/4K2R w K - 0 1",
            "8/8/8/8/8/7n/8/4K2R w K - 0 1",
            "4r3/8/8/8/8/8/8/4K2R w K - 0 1",
            "5r2/8/8/8/8/8/8/4K2R w K - 0 1",
            "6r1/8/8/8/8/8/8/4K2R w K - 0 1",
            "8/8/8/8/8/8/3p4/4K2R w K - 0 1",
            "8/8/8/8/8/8/4p3/4K2R w K - 0 1",
            "8/8/8/8/8/8/5p2/4K2R w K - 0 1",
            "8/8/8/8/8/8/6p1/4K2R w K - 0 1",
            "8/8/8/8/8/8/7p/4K2R w K - 0 1",
            "8/8/8/8/1q6/8/8/4K2R w K - 0 1",
            "8/8/8/8/2q5/8/8/4K2R w K - 0 1",
            "8/8/8/8/3q4/8/8/4K2R w K - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_kingside_white();

            let is_valid_move = pos.make_move(mv);
            assert_eq!(is_valid_move, false);
        }
    }

    #[test]
    pub fn make_move_queen_castle_white_through_attached_squares_is_illegal() {
        let fens = vec![
            "8/8/8/8/4q3/8/8/R3K3 w Q - 0 1",
            "8/8/8/8/5q2/8/8/R3K3 w Q - 0 1",
            "8/8/8/8/6q1/8/8/R3K3 w Q - 0 1",
            "8/8/8/8/7q/8/8/R3K3 w Q - 0 1",
            "1r6/8/8/8/8/8/8/R3K3 w Q - 0 1",
            "2r5/8/8/8/8/8/8/R3K3 w Q - 0 1",
            "3r4/8/8/8/8/8/8/R3K3 w Q - 0 1",
            "4r3/8/8/8/8/8/8/R3K3 w Q - 0 1",
            "8/8/8/b7/8/8/8/R3K3 w Q - 0 1",
            "8/8/8/8/b7/8/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/b7/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/b7/R3K3 w Q - 0 1",
            "8/8/8/8/8/n7/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/1n6/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/2n5/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/3n4/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/4n3/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/5n2/8/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/p7/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/1p6/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/2p5/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/3p4/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/4p3/R3K3 w Q - 0 1",
            "8/8/8/8/8/8/5p2/R3K3 w Q - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_queenside_white();

            let is_valid_move = pos.make_move(mv);
            assert_eq!(is_valid_move, false);
        }
    }

    #[test]
    pub fn make_move_king_castle_black_through_attached_squares_is_illegal() {
        let fens = vec![
            "4k2r/8/8/8/8/Q7/8/8 b k - 0 1",
            "4k2r/8/8/8/Q7/8/8/8 b k - 0 1",
            "4k2r/8/8/8/8/8/Q7/8 b k - 0 1",
            "4k2r/8/8/8/8/8/8/4R3 b k - 0 1",
            "4k2r/8/8/8/8/8/8/5R2 b k - 0 1",
            "4k2r/8/8/8/8/8/8/6R1 b k - 0 1",
            "4k2r/8/8/7B/8/8/8/8 b k - 0 1",
            "4k2r/8/7B/8/8/8/8/8 b k - 0 1",
            "4k2r/7B/8/8/8/8/8/8 b k - 0 1",
            "4k2r/8/3N4/8/8/8/8/8 b k - 0 1",
            "4k2r/8/4N3/8/8/8/8/8 b k - 0 1",
            "4k2r/8/5N2/8/8/8/8/8 b k - 0 1",
            "4k2r/8/6N1/8/8/8/8/8 b k - 0 1",
            "4k2r/8/7N/8/8/8/8/8 b k - 0 1",
            "4k2r/3P4/8/8/8/8/8/8 b k - 0 1",
            "4k2r/4P3/8/8/8/8/8/8 b k - 0 1",
            "4k2r/5P2/8/8/8/8/8/8 b k - 0 1",
            "4k2r/6P1/8/8/8/8/8/8 b k - 0 1",
            "4k2r/6P1/8/8/8/8/8/8 b k - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_kingside_black();

            let is_valid_move = pos.make_move(mv);
            assert_eq!(is_valid_move, false);
        }
    }

    #[test]
    pub fn make_move_queen_castle_black_through_attached_squares_is_illegal() {
        let fens = vec![
            "r3k3/8/8/7Q/8/8/8/8 b q - 0 1",
            "r3k3/8/8/8/7Q/8/8/8 b q - 0 1",
            "r3k3/8/8/8/8/7Q/8/8 b q - 0 1",
            "r3k3/8/8/8/8/8/7Q/8 b q - 0 1",
            "r3k3/8/8/8/4R3/8/8/8 b q - 0 1",
            "r3k3/8/8/8/3R4/8/8/8 b q - 0 1",
            "r3k3/8/8/8/2R5/8/8/8 b q - 0 1",
            "r3k3/8/8/8/1R6/8/8/8 b q - 0 1",
            "r3k3/8/8/8/B7/8/8/8 b q - 0 1",
            "r3k3/8/8/B7/8/8/8/8 b q - 0 1",
            "r3k3/8/B7/8/8/8/8/8 b q - 0 1",
            "r3k3/B7/8/8/8/8/8/8 b q - 0 1",
            "r3k3/B7/8/8/8/8/8/8 b q - 0 1",
            "r3k3/4P3/8/8/8/8/8/8 b q - 0 1",
            "r3k3/3P4/8/8/8/8/8/8 b q - 0 1",
            "r3k3/2P5/8/8/8/8/8/8 b q - 0 1",
            "r3k3/1P6/8/8/8/8/8/8 b q - 0 1",
            "r3k3/P7/8/8/8/8/8/8 b q - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_queenside_black();

            let is_valid_move = pos.make_move(mv);
            assert_eq!(is_valid_move, false);
        }
    }

    fn is_piece_on_square_as_expected(pos: &Position, sq: Square, pce: Piece) -> bool {
        let pce_on_board = pos.board.get_piece_on_square(sq);

        if pce_on_board == None {
            return false;
        }

        return pce_on_board.unwrap() == pce;
    }

    fn is_sq_empty(pos: &Position, sq: Square) -> bool {
        let empty_sq = pos.board.get_piece_on_square(sq);
        return empty_sq == None;
    }
}
