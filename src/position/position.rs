use board::board::Board;
use board::piece::Colour;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square::Square;
use input::fen::ParsedFen;
use moves::mov::Mov;
use position::castle_permissions::CastlePermission;
use position::hash::PositionHash;
use position::position_history::PositionHistory;

pub struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

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
            position_key: PositionHash::new(&parsed_fen),
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
            self.position_key.get_hash(),
            mv,
            self.fifty_move_cntr,
            self.en_pass_sq,
            self.castle_perm,
        );

        self.flip_side_to_move();
        self.position_key.update_side();

        self.move_cntr.half_move += 1;
        self.move_cntr.full_move += 1;

        // set up some general variables
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();
        let piece = self.board().get_piece_on_square(from_sq).unwrap();
        let side_to_move = self.side_to_move;

        handle_50_move_rule(self, mv, piece);

        if mv.is_quiet() {
            update_hash_on_piece_move(self, piece, from_sq, to_sq);
            self.board.move_piece(from_sq, to_sq, piece);
        } else if mv.is_double_pawn() {
            update_hash_on_piece_move(self, piece, from_sq, to_sq);
            self.board.move_piece(from_sq, to_sq, piece);

            let s = find_en_passant_sq(from_sq, side_to_move);
            self.en_pass_sq = Some(s);
        } else if mv.is_king_castle() {
            do_castle_move_king(self, side_to_move);
        } else if mv.is_queen_castle() {
            do_castle_move_queen(self, side_to_move);
        } else if mv.is_en_passant() {
            do_en_passant(self, from_sq, to_sq);
        } else if mv.is_promote() {
            if mv.is_capture() {
                let capt_pce = self.board.get_piece_on_square(to_sq).unwrap();
                remove_piece_from_board(self, capt_pce, to_sq);
            }

            let source_pce = self.board.get_piece_on_square(from_sq).unwrap();
            let target_pce_role = mv.decode_promotion_piece_role();
            let target_pce = Piece::new(target_pce_role, side_to_move);

            remove_piece_from_board(self, source_pce, from_sq);
            add_piece_to_board(self, target_pce, to_sq);
        } else if mv.is_capture() {
            let capt_pce = self.board.get_piece_on_square(to_sq).unwrap();
            remove_piece_from_board(self, capt_pce, to_sq);
            update_hash_on_piece_move(self, piece, from_sq, to_sq);
            self.board.move_piece(from_sq, to_sq, piece);
        }

        // to do - validate legality pf move
        return true;
    }
}

fn find_en_passant_sq(from_sq: Square, col: Colour) -> Square {
    match col {
        Colour::White => from_sq.square_plus_1_rank(),
        Colour::Black => from_sq.square_minus_1_rank(),
    }
}

fn remove_piece_from_board(position: &mut Position, pce: Piece, sq: Square) {
    position.board.remove_piece(pce, sq);
    position.position_key.update_piece(pce, sq);
}

fn add_piece_to_board(position: &mut Position, pce: Piece, sq: Square) {
    position.board.add_piece(pce, sq);
    position.position_key.update_piece(pce, sq);
}

fn update_hash_on_piece_move(position: &mut Position, pce: Piece, from_sq: Square, to_sq: Square) {
    position.position_key.update_piece(pce, from_sq);
    position.position_key.update_piece(pce, to_sq);
}

fn handle_50_move_rule(position: &mut Position, mv: Mov, pce_to_move: Piece) {
    if mv.is_capture() || pce_to_move.role() == PieceRole::Pawn {
        position.fifty_move_cntr = 0;
    } else {
        position.fifty_move_cntr += 1;
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

fn do_castle_move_queen(position: &mut Position, col: Colour) {
    let (queen_from_sq, queen_to_sq) = match col {
        Colour::Black => (Square::e8, Square::c8),
        Colour::White => (Square::e1, Square::c1),
    };
    let (rook_from_sq, rook_to_sq) = match col {
        Colour::Black => (Square::a8, Square::d8),
        Colour::White => (Square::a1, Square::d1),
    };

    let queen = Piece::new(PieceRole::Queen, col);
    update_hash_on_piece_move(position, queen, queen_from_sq, queen_to_sq);
    position.board.move_piece(queen_from_sq, queen_to_sq, queen);

    let rook = Piece::new(PieceRole::Rook, col);
    update_hash_on_piece_move(position, rook, rook_from_sq, rook_to_sq);
    position.board.move_piece(rook_from_sq, rook_to_sq, rook);

    position.castle_perm.set_queen(col, false);
}

fn do_en_passant(position: &mut Position, from_sq: Square, to_sq: Square) {
    let enp_sq = match position.side_to_move {
        Colour::White => to_sq.square_minus_1_rank(),
        Colour::Black => to_sq.square_plus_1_rank(),
    };

    let pawn = Piece::new(PieceRole::Pawn, position.side_to_move);
    let capt_pawn = Piece::new(PieceRole::Pawn, position.side_to_move.flip_side());

    position.board.remove_piece(capt_pawn, enp_sq);
    position.position_key.update_piece(capt_pawn, enp_sq);
    position.board.move_piece(from_sq, to_sq, pawn);

    update_hash_on_piece_move(position, pawn, from_sq, to_sq);
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

        let before_hash = pos.position_key.get_hash();

        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);

        // check before move
        assert_eq!(
            pos.board.get_piece_on_square(Square::e5).unwrap(),
            Piece::new(PieceRole::Pawn, Colour::White)
        );

        pos.make_move(mv);

        // piece has has moved
        assert_eq!(pos.board.is_sq_empty(Square::e5), true);
        assert_eq!(
            pos.board.get_piece_on_square(Square::e6).unwrap(),
            Piece::new(PieceRole::Pawn, Colour::White)
        );

        // check hashes are different
        assert_ne!(before_hash, pos.position_key.get_hash());
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
}
