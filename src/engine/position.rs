use components::board::Board;
use components::piece::Colour;
use components::piece::Piece;
use components::piece::PieceRole;
use components::square::Square;
use engine::attack_checker;
use engine::castle_permissions;
use engine::castle_permissions::CastlePermission;
use engine::castle_permissions::CastlePermissionType;
use engine::hash;
use engine::hash::PositionHash;
use engine::position_history::PositionHistory;
use input::fen::ParsedFen;
use moves::mov::Mov;
use moves::mov::MoveType;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

impl fmt::Debug for MoveCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("HalfMove : {}, ", self.half_move));
        debug_str.push_str(&format!("FullMove : {} ", self.full_move));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for MoveCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

// something to avoid bugs with bool states
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum MoveLegality {
    Legal,
    Illegal,
}
impl fmt::Display for MoveLegality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        match self {
            MoveLegality::Legal => debug_str.push_str("Legal"),
            MoveLegality::Illegal => debug_str.push_str("Illegal"),
        };

        write!(f, "{}", debug_str)
    }
}

impl fmt::Debug for MoveLegality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

static CASTLE_SQUARES_KING_WHITE: [Square; 3] = [Square::e1, Square::f1, Square::g1];

static CASTLE_SQUARES_QUEEN_WHITE: [Square; 3] = [Square::c1, Square::d1, Square::e1];

static CASTLE_SQUARES_KING_BLACK: [Square; 3] = [Square::e8, Square::f8, Square::g8];

static CASTLE_SQUARES_QUEEN_BLACK: [Square; 3] = [Square::c8, Square::d8, Square::e8];

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

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("Board : {}\n", self.board()));
        debug_str.push_str(&format!("SideToMove : {}\n", self.side_to_move()));
        if self.en_pass_sq.is_none() {
            debug_str.push_str(&"En pass Sq : -\n".to_string());
        } else {
            debug_str.push_str(&format!("En pass Sq : {}\n", self.en_pass_sq.unwrap()));
        }

        debug_str.push_str(&format!("Move Cntr : {}\n", self.move_cntr));
        debug_str.push_str(&format!("50 Move Cntr : {}\n", self.fifty_move_cntr));

        debug_str.push_str(&format!("Position Hist: {}\n", self.position_history));

        write!(f, "{}", debug_str)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        if self.board() != other.board() {
            println!("POS: boards are different");
            return false;
        }

        if self.side_to_move() != other.side_to_move() {
            println!("POS: side to move are different");
            return false;
        }

        if self.en_pass_sq != other.en_pass_sq {
            println!("POS: en passant squares are different");
            return false;
        }

        if self.castle_perm != other.castle_perm {
            println!("POS: castle permissions are different");
            return false;
        }

        if self.move_cntr != other.move_cntr {
            println!("POS: move counters are different");
            return false;
        }

        if self.fifty_move_cntr != other.fifty_move_cntr {
            println!("POS: 50-move counters are different");
            return false;
        }
        if self.position_key != other.position_key {
            println!("POS: position keys are different");
            return false;
        }
        if self.position_history != other.position_history {
            println!("POS: position histories are different");
            return false;
        }

        true
    }
}
impl Position {
    pub fn new(parsed_fen: ParsedFen) -> Position {
        let mv_cntr = MoveCounter {
            half_move: parsed_fen.half_move_cnt,
            full_move: parsed_fen.full_move_cnt,
        };

        let mut pos = Position {
            board: Board::from_fen(&parsed_fen),
            side_to_move: parsed_fen.side_to_move,
            en_pass_sq: parsed_fen.en_pass_sq,
            castle_perm: parsed_fen.castle_perm,
            move_cntr: mv_cntr,
            fifty_move_cntr: 0,
            position_history: PositionHistory::new(),
            position_key: 0,
        };

        generate_hash_from_fen(&mut pos, &parsed_fen);

        // validate position
        let bk_bb = pos.board().get_piece_bitboard(&Piece::BLACK_KING);
        assert_ne!(bk_bb, 0);
        let wk_bb = pos.board().get_piece_bitboard(&Piece::WHITE_KING);
        assert_ne!(wk_bb, 0);

        pos
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

    pub fn position_key(&self) -> PositionHash {
        self.position_key
    }

    pub fn flip_side_to_move(&mut self) {
        match self.side_to_move {
            Colour::White => self.side_to_move = Colour::Black,
            Colour::Black => self.side_to_move = Colour::White,
        };
        self.position_key = hash::update_side(self.position_key);
    }

    pub fn make_move(&mut self, mv: Mov) -> MoveLegality {
        // set up some general variables
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();

        assert!(self.board().get_piece_on_square(from_sq).is_some());
        let piece = self.board().get_piece_on_square(from_sq).unwrap();

        let capt_piece = if mv.is_capture() && !mv.is_en_passant() {
            self.board.get_piece_on_square(to_sq)
        } else {
            None
        };

        self.position_history.push(
            &self.board,
            self.position_key,
            mv,
            self.fifty_move_cntr,
            self.en_pass_sq,
            self.castle_perm,
            capt_piece,
        );

        self.move_cntr.half_move += 1;
        self.move_cntr.full_move += 1;

        handle_50_move_rule(self, mv, piece);

        let move_type = mv.get_move_type();
        if move_type.is_none() {
            panic!("Invalid MoveType");
        }

        match move_type.unwrap() {
            MoveType::Quiet => move_piece_on_board(self, piece, from_sq, to_sq),
            MoveType::Capture => do_capture_move(self, piece, from_sq, to_sq, &capt_piece.unwrap()),
            MoveType::DoublePawn => do_double_pawn_move(self, piece, from_sq, to_sq),
            MoveType::KingCastle | MoveType::QueenCastle => do_castle_move(self, mv),
            MoveType::EnPassant => do_en_passant(self, from_sq, to_sq),
            MoveType::PromoteKnightQuiet
            | MoveType::PromoteBishopQuiet
            | MoveType::PromoteRookQuiet
            | MoveType::PromoteQueenQuiet
            | MoveType::PromoteKnightCapture
            | MoveType::PromoteBishopCapture
            | MoveType::PromoteRookCapture
            | MoveType::PromoteQueenCapture => do_promotion(self, mv, from_sq, to_sq, &piece),
        }

        self.update_en_passant_sq(mv);
        self.update_castle_perms(mv, from_sq, piece);

        let move_legality = self.get_move_legality(mv);

        self.flip_side_to_move();

        move_legality
    }

    pub fn take_move(&mut self) {
        self.move_cntr.half_move -= 1;
        self.move_cntr.full_move -= 1;

        self.flip_side_to_move();

        let (board, pos_hash, _mv, fifty_move_cntr, en_pass_sq, cast_perms, _capt_pce) =
            self.position_history.pop();

        self.board = board;
        self.position_key = pos_hash;
        self.fifty_move_cntr = fifty_move_cntr;
        self.en_pass_sq = en_pass_sq;
        self.castle_perm = cast_perms;
    }

    fn get_move_legality(&self, mv: Mov) -> MoveLegality {
        // check if move results in king being in check
        let king_sq = self.board().get_king_sq(self.side_to_move);
        let attacking_side = self.side_to_move.flip_side();

        if attack_checker::is_king_sq_attacked(self.board(), king_sq, attacking_side) {
            return MoveLegality::Illegal;
        }

        // check castle through attacked squares (or king was in check before the castle move)
        if mv.is_castle() {
            let squares_to_check = self.get_castle_squares_to_check(mv, self.side_to_move);
            let is_invalid_castle = attack_checker::is_castle_squares_attacked(
                self.board(),
                squares_to_check,
                attacking_side,
            );

            if is_invalid_castle {
                return MoveLegality::Illegal;
            } else {
                return MoveLegality::Legal;
            }
        }
        MoveLegality::Legal
    }

    fn get_castle_squares_to_check(&self, mv: Mov, side_to_move: Colour) -> &[Square] {
        if mv.is_king_castle() {
            match side_to_move {
                Colour::White => &CASTLE_SQUARES_KING_WHITE,
                Colour::Black => &CASTLE_SQUARES_KING_BLACK,
            }
        } else if mv.is_queen_castle() {
            match side_to_move {
                Colour::White => &CASTLE_SQUARES_QUEEN_WHITE,
                Colour::Black => &CASTLE_SQUARES_QUEEN_BLACK,
            }
        } else {
            panic!("Invalid move test");
        }
    }

    fn update_en_passant_sq(&mut self, mv: Mov) {
        // clear en passant
        if self.en_pass_sq.is_some() {
            if mv.get_move_type().unwrap() != MoveType::DoublePawn {
                self.position_key =
                    hash::update_en_passant(self.position_key, self.en_pass_sq.unwrap());
                self.en_pass_sq = None;
            }
        }
    }

    // remove castle permissions based on the move
    fn update_castle_perms(&mut self, mv: Mov, from_sq: Square, pce: Piece) {
        if !castle_permissions::has_castle_permission(self.castle_perm) {
            // nothing to do
            return;
        }

        if pce.role() != PieceRole::King && pce.role() != PieceRole::Rook {
            // nothing to do
            return;
        }

        if mv.is_castle() {
            // permissions already adjusted
            return;
        }

        if pce.role() == PieceRole::King {
            if pce.colour() == Colour::White {
                castle_permissions::clear_white_king_and_queen(&mut self.castle_perm);
            } else if pce.colour() == Colour::Black {
                castle_permissions::clear_black_king_and_queen(&mut self.castle_perm);
            }
        } else if pce.role() == PieceRole::Rook {
            if pce.colour() == Colour::White {
                match from_sq {
                    Square::a1 => castle_permissions::clear_queen_white(&mut self.castle_perm),
                    Square::h1 => castle_permissions::clear_king_white(&mut self.castle_perm),
                    _ => (),
                };
            } else if pce.colour() == Colour::Black {
                match from_sq {
                    Square::a8 => castle_permissions::clear_queen_black(&mut self.castle_perm),
                    Square::h8 => castle_permissions::clear_king_black(&mut self.castle_perm),
                    _ => (),
                };
            }
        }
    }
}

fn generate_hash_from_fen(position: &mut Position, parsed_fen: &ParsedFen) {
    let positions = parsed_fen.piece_positions.iter();
    for (sq, pce) in positions {
        position.position_key = hash::update_piece(position.position_key, pce, *sq);
    }

    position.position_key = hash::update_side(position.position_key);

    let cp = parsed_fen.castle_perm;

    if castle_permissions::is_black_king_set(cp) {
        position.position_key =
            hash::update_castle_permissions(position.position_key, CastlePermissionType::BlackKing);
    }
    if castle_permissions::is_white_king_set(cp) {
        position.position_key =
            hash::update_castle_permissions(position.position_key, CastlePermissionType::WhiteKing);
    }
    if castle_permissions::is_black_queen_set(cp) {
        position.position_key = hash::update_castle_permissions(
            position.position_key,
            CastlePermissionType::BlackQueen,
        );
    }
    if castle_permissions::is_white_queen_set(cp) {
        position.position_key = hash::update_castle_permissions(
            position.position_key,
            CastlePermissionType::WhiteQueen,
        );
    }

    let enp = parsed_fen.en_pass_sq;
    if let Some(enp) = enp {
        position.position_key = hash::update_en_passant(position.position_key, enp);
    }
}

fn find_en_passant_sq(from_sq: Square, col: Colour) -> Option<Square> {
    // use the *from_sq* to find the en passant sq
    match col {
        Colour::White => from_sq.square_plus_1_rank(),
        Colour::Black => from_sq.square_minus_1_rank(),
    }
}

fn remove_piece_from_board(position: &mut Position, pce: Piece, sq: Square) {
    position.board.remove_piece(pce, sq);
    position.position_key = hash::update_piece(position.position_key, &pce, sq);
}

fn add_piece_to_board(position: &mut Position, pce: Piece, sq: Square) {
    position.board.add_piece(pce, sq);
    position.position_key = hash::update_piece(position.position_key, &pce, sq);
}

fn move_piece_on_board(position: &mut Position, pce: Piece, from_sq: Square, to_sq: Square) {
    position.position_key = hash::update_piece(position.position_key, &pce, from_sq);
    position.position_key = hash::update_piece(position.position_key, &pce, to_sq);
    position.board.move_piece(from_sq, to_sq, pce);
}

fn handle_50_move_rule(position: &mut Position, mv: Mov, pce_to_move: Piece) {
    if mv.is_capture() || pce_to_move.role() == PieceRole::Pawn {
        position.fifty_move_cntr = 0;
    } else {
        position.fifty_move_cntr += 1;
    }
}

fn do_castle_move(position: &mut Position, mv: Mov) {
    let colour = position.side_to_move();

    let (king, rook, rook_from_sq, rook_to_sq, king_from_sq, king_to_sq) =
        if mv.is_king_castle() && colour == Colour::White {
            (
                Piece::WHITE_KING,
                Piece::WHITE_ROOK,
                Square::h1,
                Square::f1,
                Square::e1,
                Square::g1,
            )
        } else if mv.is_king_castle() && colour == Colour::Black {
            (
                Piece::BLACK_KING,
                Piece::BLACK_ROOK,
                Square::h8,
                Square::f8,
                Square::e8,
                Square::g8,
            )
        } else if mv.is_queen_castle() && colour == Colour::White {
            (
                Piece::WHITE_KING,
                Piece::WHITE_ROOK,
                Square::a1,
                Square::d1,
                Square::e1,
                Square::c1,
            )
        } else if mv.is_queen_castle() && colour == Colour::Black {
            (
                Piece::BLACK_KING,
                Piece::BLACK_ROOK,
                Square::a8,
                Square::d8,
                Square::e8,
                Square::c8,
            )
        } else {
            panic!("Invalid castle move");
        };

    move_piece_on_board(position, king, king_from_sq, king_to_sq);
    move_piece_on_board(position, rook, rook_from_sq, rook_to_sq);

    clear_castle_permissions_for_colour(position, colour);
}

fn clear_castle_permissions_for_colour(position: &mut Position, col: Colour) {
    match col {
        Colour::White => {
            castle_permissions::clear_white_king_and_queen(&mut position.castle_perm);

            position.position_key = hash::update_castle_permissions(
                position.position_key,
                CastlePermissionType::WhiteKing,
            );
            position.position_key = hash::update_castle_permissions(
                position.position_key,
                CastlePermissionType::WhiteQueen,
            );
        }
        Colour::Black => {
            castle_permissions::clear_black_king_and_queen(&mut position.castle_perm);

            position.position_key = hash::update_castle_permissions(
                position.position_key,
                CastlePermissionType::BlackKing,
            );
            position.position_key = hash::update_castle_permissions(
                position.position_key,
                CastlePermissionType::BlackQueen,
            );
        }
    }
}

fn do_double_pawn_move(position: &mut Position, piece: Piece, from_sq: Square, to_sq: Square) {
    move_piece_on_board(position, piece, from_sq, to_sq);

    let s = find_en_passant_sq(from_sq, position.side_to_move());
    match s {
        Some(_) => {
            position.en_pass_sq = s;
            position.position_key = hash::update_en_passant(position.position_key(), s.unwrap());
        }
        None => panic!("Unable to find en passant square"),
    }
}

fn do_en_passant(position: &mut Position, from_sq: Square, to_sq: Square) {
    let (capt_sq, pawn, capt_pawn) = match position.side_to_move {
        Colour::White => (
            to_sq.square_minus_1_rank(),
            Piece::WHITE_PAWN,
            Piece::BLACK_PAWN,
        ),
        Colour::Black => (
            to_sq.square_plus_1_rank(),
            Piece::BLACK_PAWN,
            Piece::WHITE_PAWN,
        ),
    };

    match capt_sq {
        Some(_) => {
            remove_piece_from_board(position, capt_pawn, capt_sq.unwrap());
            move_piece_on_board(position, pawn, from_sq, to_sq);
        }
        None => panic!("Invalid capture square for en passant move"),
    }
}

fn do_promotion(
    position: &mut Position,
    mv: Mov,
    from_sq: Square,
    to_sq: Square,
    source_pce: &Piece,
) {
    if mv.is_capture() {
        let capt_pce = position.board.get_piece_on_square(to_sq).unwrap();
        remove_piece_from_board(position, capt_pce, to_sq);
    }

    let target_pce_role = mv.decode_promotion_piece_role();
    let target_pce = Piece::new(target_pce_role, position.side_to_move());

    remove_piece_from_board(position, *source_pce, from_sq);
    add_piece_to_board(position, target_pce, to_sq);
}

fn do_capture_move(
    position: &mut Position,
    piece_to_move: Piece,
    from_sq: Square,
    to_sq: Square,
    capt_pce: &Piece,
) {
    remove_piece_from_board(position, *capt_pce, to_sq);
    move_piece_on_board(position, piece_to_move, from_sq, to_sq);
}

#[cfg(test)]
mod tests {
    use components::piece::Colour;
    use components::piece::Piece;
    use components::piece::PieceRole;
    use components::square::Square;
    use engine::castle_permissions;
    use engine::hash;
    use engine::position::MoveLegality;
    use engine::position::Position;
    use input::fen;
    use moves::mov::Mov;

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
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n w - - 0 1";
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
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n b - - 0 1";
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
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
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

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_king_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
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

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_queen_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));
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
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_queen_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));
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

        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_en_passant_black() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/pPBP1P2/2R1NpP1/2r1r2P/R2q3n b - b3 0 1";
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
        let fen = "1n1k2bp/2p2pb1/1p5p/1B1pP1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - d6 0 1";
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
            let fen = "kn3b1p/2p1Pp2/1p5p/1B1pb1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - - 0 1";
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
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/2R5 b - - 0 1";
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
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 b - - 0 1";
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
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 w - - 0 1";
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
    pub fn make_move_king_castle_white_through_attacked_squares_is_illegal() {
        let fens = vec![
            "1k6/8/8/8/3q4/8/8/4K2R w K - 0 1",
            "1k6/8/8/8/8/3q4/8/4K2R w K - 0 1",
            "1k6/8/8/8/8/8/8/q3K2R w K - 0 1",
            "1k6/8/8/8/8/8/7q/4K2R w K - 0 1",
            "1k6/8/8/8/8/7q/8/4K2R w K - 0 1",
            "1k4q1/8/8/8/8/8/8/4K2R w K - 0 1",
            "1k3q2/8/8/8/8/8/8/4K2R w K - 0 1",
            "1k2q3/8/8/8/8/8/8/4K2R w K - 0 1",
            "1k6/8/8/1q6/8/8/8/4K2R w K - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_kingside_white();

            let move_legality = pos.make_move(mv);
            assert_eq!(move_legality, MoveLegality::Illegal);
        }
    }

    #[test]
    pub fn make_move_queen_castle_white_through_attacked_squares_is_illegal() {
        let fens = vec![
            "6k1/8/8/8/5q2/8/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/6q1/8/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/8/6q1/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/8/8/8/R3K2q w Q - 0 1",
            "2k5/8/8/8/8/4q3/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/8/3q4/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/8/q7/8/R3K3 w Q - 0 1",
            "2k5/2q5/8/8/8/8/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/q7/8/8/R3K3 w Q - 0 1",
            "2k5/8/8/8/8/q7/8/R3K3 w Q - 0 1",
        ];

        for fen in fens {
            println!(" FEN **** : {}", fen);
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_queenside_white();

            let move_legality = pos.make_move(mv);
            assert_eq!(move_legality, MoveLegality::Illegal);
        }
    }

    #[test]
    pub fn make_move_king_castle_black_through_attacked_squares_is_illegal() {
        let fens = vec![
            "4k2r/8/8/8/Q7/8/8/7K b k - 0 1",
            "4k2r/8/8/8/8/Q7/8/7K b k - 0 1",
            "4k2r/8/8/8/8/1Q6/8/7K b k - 0 1",
            "4k2r/8/8/8/8/5Q2/8/7K b k - 0 1",
            "4k2r/8/8/8/8/6Q1/8/7K b k - 0 1",
            "4k2r/8/7Q/8/8/8/8/7K b k - 0 1",
            "4k2r/7Q/8/8/8/8/8/7K b k - 0 1",
            "4k2r/4Q3/8/8/8/8/8/7K b k - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_kingside_black();

            let move_legality = pos.make_move(mv);
            assert_eq!(move_legality, MoveLegality::Illegal);
        }
    }

    #[test]
    pub fn make_move_queen_castle_black_through_attacked_squares_is_illegal() {
        let fens = vec![
            "r3k3/8/8/7Q/8/8/8/1K6 b q - 0 1",
            "r3k3/8/8/3Q4/8/8/8/1K6 b q - 0 1",
            "r3k3/8/2Q5/8/8/8/8/1K6 b q - 0 1",
            "r3k3/8/Q7/8/8/8/8/1K6 b q - 0 1",
            "r3k1Q1/8/8/8/8/8/8/1K6 b q - 0 1",
            "r3k3/8/8/8/8/8/8/1KQ5 b q - 0 1",
            "r3k3/8/8/8/8/8/8/1K1Q4 b q - 0 1",
            "r3k3/8/8/8/Q7/8/8/1K6 b q - 0 1",
        ];

        for fen in fens {
            let parsed_fen = fen::get_position(&fen);
            let mut pos = Position::new(parsed_fen);

            let mv = Mov::encode_move_castle_queenside_black();

            let move_legality = pos.make_move(mv);
            assert_eq!(move_legality, MoveLegality::Illegal);
        }
    }

    #[test]
    pub fn make_move_white_king_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == true);

        let mv = Mov::encode_move_quiet(Square::e1, Square::e2);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == false);
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_white_kings_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == true);

        let mv = Mov::encode_move_quiet(Square::h1, Square::g1);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == false);
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == true);
    }

    #[test]
    pub fn make_move_white_queens_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == true);

        let mv = Mov::encode_move_quiet(Square::a1, Square::b1);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_white_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_white_queen_set(pos.castle_permissions()) == false);
    }

    /////////////////////////////
    ///

    #[test]
    pub fn make_move_black_king_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == true);

        let mv = Mov::encode_move_quiet(Square::e8, Square::e7);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == false);
        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_black_kings_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == true);

        let mv = Mov::encode_move_quiet(Square::h8, Square::g8);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == false);
        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == true);
    }

    #[test]
    pub fn make_move_black_queens_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == true);

        let mv = Mov::encode_move_quiet(Square::a8, Square::b8);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_black_king_set(pos.castle_permissions()) == true);
        assert!(castle_permissions::is_black_queen_set(pos.castle_permissions()) == false);
    }

    #[test]
    pub fn make_move_take_move_position_and_board_restored_white_to_move() {
        let fen = "1b1kN3/Qp1P2p1/q2P1Nn1/PP3r2/3rPnb1/1p1pp3/B1P1P2B/R3K2R w KQ - 5 8";

        let mut ml = vec![];
        ml.push(Mov::encode_move_castle_kingside_white());
        ml.push(Mov::encode_move_castle_queenside_white());
        ml.push(Mov::encode_move_capture(Square::e8, Square::g7));
        ml.push(Mov::encode_move_quiet(Square::b5, Square::b6));
        ml.push(Mov::encode_move_double_pawn_first(Square::c2, Square::c4));

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let parsed_fen_orig = fen::get_position(&fen);
        let pos_orig = Position::new(parsed_fen_orig);

        for mv in ml {
            println!("move: {}", mv);
            pos.make_move(mv);
            assert_ne!(pos_orig, pos);

            pos.take_move();

            assert!(pos_orig == pos);
        }
    }

    #[test]
    pub fn make_move_take_move_position_and_board_restored_black_to_move() {
        let fen = "r3k2r/1pb2p2/qQ1P2n1/PPPN2N1/4Pnb1/1p1pp3/B1P1P2B/R3K2R b kq - 3 11";

        let mut ml = vec![];
        ml.push(Mov::encode_move_castle_kingside_black());
        ml.push(Mov::encode_move_castle_queenside_black());
        ml.push(Mov::encode_move_capture(Square::c7, Square::b6));
        ml.push(Mov::encode_move_quiet(Square::f7, Square::f6));
        ml.push(Mov::encode_move_double_pawn_first(Square::f7, Square::f6));

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let parsed_fen_orig = fen::get_position(&fen);
        let pos_orig = Position::new(parsed_fen_orig);

        for mv in ml {
            pos.make_move(mv);
            assert_ne!(pos_orig, pos);

            pos.take_move();

            assert!(pos_orig == pos);
        }
    }

    #[test]
    pub fn make_move_hash_updated_white_double_pawn_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);
        let init_hash = pos.position_key();

        let mut expected_hash = hash::update_piece(init_hash, &Piece::WHITE_PAWN, Square::b2);
        expected_hash = hash::update_piece(expected_hash, &Piece::WHITE_PAWN, Square::b4);
        expected_hash = hash::update_en_passant(expected_hash, Square::b3);
        expected_hash = hash::update_side(expected_hash);

        let wp_double_mv = Mov::encode_move_double_pawn_first(Square::b2, Square::b4);
        pos.make_move(wp_double_mv);

        assert!(init_hash != pos.position_key());
        assert!(expected_hash == pos.position_key());
    }

    #[test]
    pub fn make_move_hash_updated_black_double_pawn_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);
        let init_hash = pos.position_key();

        let mut expected_hash = hash::update_piece(init_hash, &Piece::BLACK_PAWN, Square::b7);
        expected_hash = hash::update_piece(expected_hash, &Piece::BLACK_PAWN, Square::b5);
        expected_hash = hash::update_en_passant(expected_hash, Square::b6);
        expected_hash = hash::update_side(expected_hash);

        let bp_double_mv = Mov::encode_move_double_pawn_first(Square::b7, Square::b5);
        pos.make_move(bp_double_mv);

        assert!(init_hash != pos.position_key());
        assert!(expected_hash == pos.position_key());
    }

    #[test]
    pub fn make_move_hash_updated_white_quiet_move() {
        let fen = "r1bqkbnr/pp1n1p1p/2pp4/4p1p1/1P1P4/5PP1/P1P1PN1P/RNBQKB1R w KQkq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);
        let init_hash = pos.position_key();

        let mut expected_hash = hash::update_piece(init_hash, &Piece::WHITE_KNIGHT, Square::f2);
        expected_hash = hash::update_piece(expected_hash, &Piece::WHITE_KNIGHT, Square::g4);
        expected_hash = hash::update_side(expected_hash);

        let wp_double_mv = Mov::encode_move_quiet(Square::f2, Square::g4);
        pos.make_move(wp_double_mv);

        assert!(init_hash != pos.position_key());
        assert!(expected_hash == pos.position_key());
    }

    #[test]
    pub fn make_move_hash_updated_black_quiet_move() {
        let fen = "r1bqkbnr/pp1n1p1p/2pp4/4p1p1/1P1P4/5PP1/P1P1PN1P/RNBQKB1R b KQkq - 0 1";

        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);
        let init_hash = pos.position_key();

        let mut expected_hash = hash::update_piece(init_hash, &Piece::BLACK_KNIGHT, Square::f6);
        expected_hash = hash::update_piece(expected_hash, &Piece::BLACK_KNIGHT, Square::d7);
        expected_hash = hash::update_side(expected_hash);

        let wp_double_mv = Mov::encode_move_quiet(Square::d7, Square::f6);
        pos.make_move(wp_double_mv);

        assert!(init_hash != pos.position_key());
        assert!(expected_hash == pos.position_key());
    }

    #[test]
    pub fn make_move_hash_updated_black_en_passant_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/pPBP1P2/2R1NpP1/2r1r2P/R2q3n b - b3 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let init_hash = pos.position_key();

        // remove white pawn on b4
        let mut expected_hash = hash::update_piece(init_hash, &Piece::WHITE_PAWN, Square::b4);
        // move a4->b3
        expected_hash = hash::update_piece(expected_hash, &Piece::BLACK_PAWN, Square::a4);
        expected_hash = hash::update_piece(expected_hash, &Piece::BLACK_PAWN, Square::b3);
        expected_hash = hash::update_en_passant(expected_hash, Square::b3);
        expected_hash = hash::update_side(expected_hash);

        assert_eq!(pos.en_passant_square(), Some(Square::b3));
        let mv = Mov::encode_move_en_passant(Square::a4, Square::b3);
        pos.make_move(mv);

        assert!(init_hash != pos.position_key());
        assert!(expected_hash == pos.position_key());
    }

    #[test]
    pub fn make_move_hash_updated_white_en_passant() {
        let fen = "1n1k2bp/2p2pb1/1p5p/1B1pP1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - d6 0 1";
        let parsed_fen = fen::get_position(&fen);
        let mut pos = Position::new(parsed_fen);

        let init_hash = pos.position_key();

        // remove black pawn
        let mut expected_hash = hash::update_piece(init_hash, &Piece::BLACK_PAWN, Square::d5);
        // move e5->d6
        expected_hash = hash::update_piece(expected_hash, &Piece::WHITE_PAWN, Square::e5);
        expected_hash = hash::update_piece(expected_hash, &Piece::WHITE_PAWN, Square::d6);
        expected_hash = hash::update_en_passant(expected_hash, Square::d6);
        expected_hash = hash::update_side(expected_hash);

        assert_eq!(pos.en_passant_square(), Some(Square::d6));
        let mv = Mov::encode_move_en_passant(Square::e5, Square::d6);
        pos.make_move(mv);

        assert!(init_hash != pos.position_key());
        assert!(expected_hash == pos.position_key());
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
        return empty_sq.is_none();
    }
}
