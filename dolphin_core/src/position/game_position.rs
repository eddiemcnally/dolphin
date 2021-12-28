use crate::board::bitboard;
use crate::board::colour::Colour;
use crate::board::game_board::Board;
use crate::board::occupancy_masks::OccupancyMasks;
use crate::board::piece;
use crate::board::piece::Piece;
use crate::board::square::Square;
use crate::moves::mov::Mov;
use crate::moves::mov::MoveType;
use crate::position::attack_checker;
use crate::position::castle_permissions;
use crate::position::castle_permissions::{CastlePermission, CastlePermissionType};
use crate::position::move_counter::MoveCounter;
use crate::position::position_history::PositionHistory;
use crate::position::zobrist_keys::ZobristHash;
use crate::position::zobrist_keys::ZobristKeys;
use std::fmt;

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

const CASTLE_SQUARES_KING_WHITE: [Square; 3] = [Square::e1, Square::f1, Square::g1];

const CASTLE_SQUARES_QUEEN_WHITE: [Square; 3] = [Square::c1, Square::d1, Square::e1];

const CASTLE_SQUARES_KING_BLACK: [Square; 3] = [Square::e8, Square::f8, Square::g8];

const CASTLE_SQUARES_QUEEN_BLACK: [Square; 3] = [Square::c8, Square::d8, Square::e8];

#[derive(Eq, Clone)]
pub struct Position<'a> {
    board: Board,
    position_history: Box<PositionHistory>,
    occ_masks: &'a OccupancyMasks,
    zobrist_keys: &'a ZobristKeys,
    game_state: GameState,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct GameState {
    side_to_move: Colour,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
    move_cntr: MoveCounter,
    fifty_move_cntr: u8,
    position_hash: ZobristHash,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            side_to_move: Colour::White,
            position_hash: 0,
            move_cntr: MoveCounter::default(),
            fifty_move_cntr: 0,
            en_pass_sq: None,
            castle_perm: castle_permissions::NO_CASTLE_PERMS_AVAIL,
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState::default()
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl<'a> fmt::Debug for Position<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("Board : {}\n", self.board()));
        debug_str.push_str(&format!("SideToMove : {}\n", self.game_state.side_to_move));
        if self.game_state.en_pass_sq.is_none() {
            debug_str.push_str(&"En pass Sq : -\n".to_string());
        } else {
            debug_str.push_str(&format!(
                "En pass Sq : {}\n",
                self.game_state.en_pass_sq.unwrap()
            ));
        }

        debug_str.push_str(&format!("Move Cntr : {}\n", self.game_state.move_cntr));
        debug_str.push_str(&format!(
            "50 Move Cntr : {}\n",
            self.game_state.fifty_move_cntr
        ));

        debug_str.push_str(&format!("Position Hist: {}\n", self.position_history));

        write!(f, "{}", debug_str)
    }
}

impl PartialEq for Position<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.board() != other.board() {
            println!("POS: boards are different");
            return false;
        }

        if self.side_to_move() != other.side_to_move() {
            println!("POS: side to move are different");
            return false;
        }

        if self.game_state.en_pass_sq != other.game_state.en_pass_sq {
            println!("POS: en passant squares are different");
            return false;
        }

        if self.game_state.castle_perm != other.game_state.castle_perm {
            println!("POS: castle permissions are different");
            return false;
        }

        if self.game_state.move_cntr != other.game_state.move_cntr {
            println!("POS: move counters are different");
            return false;
        }

        if self.game_state.fifty_move_cntr != other.game_state.fifty_move_cntr {
            println!("POS: 50-move counters are different");
            return false;
        }
        if self.game_state.position_hash != other.game_state.position_hash {
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

impl<'a> Position<'a> {
    pub fn new(
        board: Board,
        castle_permissions: CastlePermission,
        move_counter: MoveCounter,
        en_passant_sq: Option<Square>,
        side_to_move: Colour,
        zobrist_keys: &'a ZobristKeys,
        occupancy_masks: &'a OccupancyMasks,
    ) -> Position<'a> {
        let game_state = GameState {
            side_to_move,
            en_pass_sq: en_passant_sq,
            castle_perm: castle_permissions,
            move_cntr: move_counter,
            ..Default::default()
        };

        let mut pos = Position {
            board,
            game_state,
            position_history: PositionHistory::new(),
            occ_masks: occupancy_masks,
            zobrist_keys,
        };

        // generate position hash
        let mut pce_bb = pos.board.get_bitboard();
        while pce_bb != 0 {
            let sq = bitboard::pop_1st_bit(&mut pce_bb);

            let pce = &mut None;
            pos.board().get_piece_on_square(sq, pce);

            pos.game_state.position_hash ^= pos.zobrist_keys.piece_square(pce.unwrap(), sq);
        }

        pos.game_state.position_hash ^= pos.zobrist_keys.side();

        if castle_permissions::is_black_king_set(castle_permissions) {
            pos.game_state.position_hash ^= pos
                .zobrist_keys
                .castle_permission(CastlePermissionType::BlackKing);
        }
        if castle_permissions::is_white_king_set(castle_permissions) {
            pos.game_state.position_hash ^= pos
                .zobrist_keys
                .castle_permission(CastlePermissionType::WhiteKing);
        }
        if castle_permissions::is_black_queen_set(castle_permissions) {
            pos.game_state.position_hash ^= pos
                .zobrist_keys
                .castle_permission(CastlePermissionType::BlackQueen);
        }
        if castle_permissions::is_white_queen_set(castle_permissions) {
            pos.game_state.position_hash ^= pos
                .zobrist_keys
                .castle_permission(CastlePermissionType::WhiteQueen);
        }

        if let Some(_enp) = en_passant_sq {
            pos.game_state.position_hash ^= pos.zobrist_keys.en_passant(en_passant_sq.unwrap());
        }

        // validate position
        let bk_bb = pos.board().get_piece_bitboard(&piece::BLACK_KING);
        assert_ne!(bk_bb, 0);
        let wk_bb = pos.board().get_piece_bitboard(&piece::WHITE_KING);
        assert_ne!(wk_bb, 0);

        pos
    }

    pub fn side_to_move(&self) -> Colour {
        self.game_state.side_to_move
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.game_state.en_pass_sq
    }

    pub fn castle_permissions(&self) -> CastlePermission {
        self.game_state.castle_perm
    }

    pub fn move_counter(&self) -> &MoveCounter {
        &self.game_state.move_cntr
    }

    pub fn position_hash(&self) -> ZobristHash {
        self.game_state.position_hash
    }

    pub fn occupancy_masks(&self) -> &'a OccupancyMasks {
        self.occ_masks
    }

    pub fn flip_side_to_move(&mut self) {
        match self.game_state.side_to_move {
            Colour::White => self.game_state.side_to_move = Colour::Black,
            Colour::Black => self.game_state.side_to_move = Colour::White,
        };
        self.game_state.position_hash ^= self.zobrist_keys.side();
    }

    pub fn make_move(&mut self, mv: Mov) -> MoveLegality {
        // set up some general variables
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();

        let piece = &mut None;
        self.board().get_piece_on_square(from_sq, piece);

        let capt_pce = &mut None;
        if mv.is_capture() && !mv.is_en_passant() {
            self.board.get_piece_on_square(to_sq, capt_pce);
        }
        self.position_history
            .push(self.game_state, mv, piece.unwrap(), *capt_pce);

        self.game_state.move_cntr.incr_half_move();
        self.game_state.move_cntr.incr_full_move();

        handle_50_move_rule(self, mv, piece.unwrap());

        let move_type = mv.get_move_type();

        match move_type {
            MoveType::Quiet => move_piece_on_board(self, piece.unwrap(), from_sq, to_sq),
            MoveType::Capture => {
                do_capture_move(self, piece.unwrap(), from_sq, to_sq, capt_pce.unwrap())
            }
            MoveType::DoublePawn => do_double_pawn_move(self, piece.unwrap(), from_sq, to_sq),
            MoveType::KingCastle | MoveType::QueenCastle => do_castle_move(self, mv),
            MoveType::EnPassant => do_en_passant(self, from_sq, to_sq),
            MoveType::PromoteKnightQuiet
            | MoveType::PromoteBishopQuiet
            | MoveType::PromoteRookQuiet
            | MoveType::PromoteQueenQuiet
            | MoveType::PromoteKnightCapture
            | MoveType::PromoteBishopCapture
            | MoveType::PromoteRookCapture
            | MoveType::PromoteQueenCapture => {
                do_promotion(self, mv, from_sq, to_sq, piece.unwrap())
            }
        }

        // update some states based on the move
        self.update_en_passant_sq(mv);
        if castle_permissions::has_castle_permission(self.game_state.castle_perm) {
            self.update_castle_perms(mv, from_sq, to_sq, piece.unwrap());
        }

        let move_legality = self.get_move_legality(mv);

        self.flip_side_to_move();

        move_legality
    }

    pub fn take_move(&mut self) {
        self.flip_side_to_move();

        let (gs, mv, piece, capt_piece) = self.position_history.pop();

        self.game_state = gs;

        let mt = mv.get_move_type();

        match mt {
            MoveType::Quiet => self.reverse_quiet_move(mv, piece),
            MoveType::DoublePawn => self.reverse_quiet_move(mv, piece),
            MoveType::Capture => self.reverse_capture_move(mv, piece, capt_piece),
            MoveType::KingCastle | MoveType::QueenCastle => {
                self.reverse_castle_move(mv, self.side_to_move())
            }
            MoveType::EnPassant => self.reverse_en_passant_move(mv, self.side_to_move()),
            MoveType::PromoteKnightQuiet
            | MoveType::PromoteBishopQuiet
            | MoveType::PromoteRookQuiet
            | MoveType::PromoteQueenQuiet
            | MoveType::PromoteKnightCapture
            | MoveType::PromoteBishopCapture
            | MoveType::PromoteRookCapture
            | MoveType::PromoteQueenCapture => self.reverse_promotion_move(mv, piece, capt_piece),
        }
    }

    fn reverse_quiet_move(&mut self, mv: Mov, piece: &'static Piece) {
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();

        // revert the move
        self.board.move_piece(to_sq, from_sq, piece);
    }

    fn reverse_capture_move(
        &mut self,
        mv: Mov,
        pce: &'static Piece,
        capture_pce: Option<&'static Piece>,
    ) {
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();

        // revert move
        self.board.move_piece(to_sq, from_sq, pce);
        // add back the captured piece
        self.board.add_piece(capture_pce.unwrap(), to_sq);
    }

    fn reverse_promotion_move(
        &mut self,
        mv: Mov,
        pce: &'static Piece,
        capture_pce: Option<&'static Piece>,
    ) {
        debug_assert!(mv.is_promote(), "reverse_promotion_move, invalid move type");

        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();

        // remove promoted piece
        self.board.remove_from_sq(to_sq);
        // put the moved piece back to it's original square
        self.board.add_piece(pce, from_sq);

        if let Some(..) = capture_pce {
            self.board.add_piece(capture_pce.unwrap(), to_sq);
        }
    }

    fn reverse_en_passant_move(&mut self, mv: Mov, side_move: Colour) {
        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();

        match side_move {
            Colour::White => {
                self.board.move_piece(to_sq, from_sq, &piece::WHITE_PAWN);

                let capt_sq = to_sq.square_minus_1_rank();
                self.board.add_piece(&piece::BLACK_PAWN, capt_sq.unwrap());
            }
            Colour::Black => {
                self.board.move_piece(to_sq, from_sq, &piece::BLACK_PAWN);

                let capt_sq = to_sq.square_plus_1_rank();
                self.board.add_piece(&piece::WHITE_PAWN, capt_sq.unwrap());
            }
        }
    }

    fn reverse_castle_move(&mut self, mv: Mov, side_move: Colour) {
        match side_move {
            Colour::White => {
                if mv.is_king_castle() {
                    self.board
                        .move_piece(Square::g1, Square::e1, &piece::WHITE_KING);
                    self.board
                        .move_piece(Square::f1, Square::h1, &piece::WHITE_ROOK);
                } else {
                    self.board
                        .move_piece(Square::c1, Square::e1, &piece::WHITE_KING);
                    self.board
                        .move_piece(Square::d1, Square::a1, &piece::WHITE_ROOK);
                }
            }
            Colour::Black => {
                if mv.is_king_castle() {
                    self.board
                        .move_piece(Square::g8, Square::e8, &piece::BLACK_KING);
                    self.board
                        .move_piece(Square::f8, Square::h8, &piece::BLACK_ROOK);
                } else {
                    self.board
                        .move_piece(Square::c8, Square::e8, &piece::BLACK_KING);
                    self.board
                        .move_piece(Square::d8, Square::a8, &piece::BLACK_ROOK);
                }
            }
        }
    }

    fn get_move_legality(&self, mv: Mov) -> MoveLegality {
        // check if move results in king being in check
        let king_sq = self.board().get_king_sq(self.game_state.side_to_move);
        let attacking_side = self.game_state.side_to_move.flip_side();

        if attack_checker::is_sq_attacked(self.occ_masks, self.board(), king_sq, attacking_side) {
            return MoveLegality::Illegal;
        }

        // check castle through attacked squares (or king was in check before the castle move)
        if mv.is_castle() {
            let squares_to_check =
                self.get_castle_squares_to_check(mv, self.game_state.side_to_move);
            let is_invalid_castle = attack_checker::is_castle_squares_attacked(
                self.occ_masks,
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
        if self.game_state.en_pass_sq.is_some() && !mv.is_double_pawn() {
            self.game_state.position_hash ^= self
                .zobrist_keys
                .en_passant(self.game_state.en_pass_sq.unwrap());
            self.game_state.en_pass_sq = None;
        }
    }

    // remove castle permissions based on the move
    fn update_castle_perms(
        &mut self,
        mv: Mov,
        from_sq: Square,
        to_sq: Square,
        pce: &'static Piece,
    ) {
        if mv.is_castle() {
            // permissions already adjusted
            return;
        }

        // check if rook has just been captured
        if mv.is_capture() {
            match to_sq {
                Square::a1 => {
                    self.game_state.castle_perm =
                        castle_permissions::clear_queen_white(self.game_state.castle_perm)
                }
                Square::h1 => {
                    self.game_state.castle_perm =
                        castle_permissions::clear_king_white(self.game_state.castle_perm)
                }
                Square::a8 => {
                    self.game_state.castle_perm =
                        castle_permissions::clear_queen_black(self.game_state.castle_perm)
                }
                Square::h8 => {
                    self.game_state.castle_perm =
                        castle_permissions::clear_king_black(self.game_state.castle_perm)
                }
                _ => (),
            }
        }

        // check if king or rook have moved
        if pce.is_king() {
            match self.side_to_move() {
                Colour::White => {
                    self.game_state.castle_perm =
                        castle_permissions::clear_white_king_and_queen(self.game_state.castle_perm)
                }
                Colour::Black => {
                    self.game_state.castle_perm =
                        castle_permissions::clear_black_king_and_queen(self.game_state.castle_perm)
                }
            }
        } else if pce.is_rook() {
            match self.side_to_move() {
                Colour::White => {
                    match from_sq {
                        Square::a1 => {
                            self.game_state.castle_perm =
                                castle_permissions::clear_queen_white(self.game_state.castle_perm);
                        }
                        Square::h1 => {
                            self.game_state.castle_perm =
                                castle_permissions::clear_king_white(self.game_state.castle_perm)
                        }
                        _ => (),
                    };
                }
                Colour::Black => {
                    match from_sq {
                        Square::a8 => {
                            self.game_state.castle_perm =
                                castle_permissions::clear_queen_black(self.game_state.castle_perm)
                        }
                        Square::h8 => {
                            self.game_state.castle_perm =
                                castle_permissions::clear_king_black(self.game_state.castle_perm)
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

fn find_en_passant_sq(from_sq: Square, col: Colour) -> Option<Square> {
    // use the *from_sq* to find the en passant sq
    match col {
        Colour::White => from_sq.square_plus_1_rank(),
        Colour::Black => from_sq.square_minus_1_rank(),
    }
}

fn remove_piece_from_board(position: &mut Position, pce: &'static Piece, sq: Square) {
    position.board.remove_piece(pce, sq);
    position.game_state.position_hash ^= position.zobrist_keys.piece_square(pce, sq);
}

fn add_piece_to_board(position: &mut Position, pce: &'static Piece, sq: Square) {
    position.board.add_piece(pce, sq);
    position.game_state.position_hash ^= position.zobrist_keys.piece_square(pce, sq);
}

fn move_piece_on_board(
    position: &mut Position,
    pce: &'static Piece,
    from_sq: Square,
    to_sq: Square,
) {
    position.game_state.position_hash ^= position.zobrist_keys.piece_square(pce, from_sq);
    position.game_state.position_hash ^= position.zobrist_keys.piece_square(pce, to_sq);
    position.board.move_piece(from_sq, to_sq, pce);
}

fn handle_50_move_rule(position: &mut Position, mv: Mov, pce_to_move: &'static Piece) {
    if mv.is_capture() || pce_to_move.is_pawn() {
        position.game_state.fifty_move_cntr = 0;
    } else {
        position.game_state.fifty_move_cntr += 1;
    }
}

fn do_castle_move(position: &mut Position, mv: Mov) {
    let colour = position.side_to_move();

    let (king, rook, rook_from_sq, rook_to_sq, king_from_sq, king_to_sq) =
        if mv.is_king_castle() && colour == Colour::White {
            (
                &piece::WHITE_KING,
                &piece::WHITE_ROOK,
                Square::h1,
                Square::f1,
                Square::e1,
                Square::g1,
            )
        } else if mv.is_king_castle() && colour == Colour::Black {
            (
                &piece::BLACK_KING,
                &piece::BLACK_ROOK,
                Square::h8,
                Square::f8,
                Square::e8,
                Square::g8,
            )
        } else if mv.is_queen_castle() && colour == Colour::White {
            (
                &piece::WHITE_KING,
                &piece::WHITE_ROOK,
                Square::a1,
                Square::d1,
                Square::e1,
                Square::c1,
            )
        } else if mv.is_queen_castle() && colour == Colour::Black {
            (
                &piece::BLACK_KING,
                &piece::BLACK_ROOK,
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
            position.game_state.castle_perm =
                castle_permissions::clear_white_king_and_queen(position.game_state.castle_perm);
            position.game_state.position_hash ^= position
                .zobrist_keys
                .castle_permission(CastlePermissionType::WhiteKing);
            position.game_state.position_hash ^= position
                .zobrist_keys
                .castle_permission(CastlePermissionType::WhiteQueen);
        }
        Colour::Black => {
            position.game_state.castle_perm =
                castle_permissions::clear_black_king_and_queen(position.game_state.castle_perm);
            position.game_state.position_hash ^= position
                .zobrist_keys
                .castle_permission(CastlePermissionType::BlackKing);
            position.game_state.position_hash ^= position
                .zobrist_keys
                .castle_permission(CastlePermissionType::BlackQueen);
        }
    }
}

fn do_double_pawn_move(
    position: &mut Position,
    piece: &'static Piece,
    from_sq: Square,
    to_sq: Square,
) {
    move_piece_on_board(position, piece, from_sq, to_sq);

    let s = find_en_passant_sq(from_sq, position.side_to_move());
    match s {
        Some(_) => {
            position.game_state.en_pass_sq = s;
            position.game_state.position_hash ^= position.zobrist_keys.en_passant(s.unwrap());
        }
        None => panic!("Unable to find en passant square"),
    }
}

fn do_en_passant(position: &mut Position, from_sq: Square, to_sq: Square) {
    let (capt_sq, pawn, capt_pawn) = match position.game_state.side_to_move {
        Colour::White => (
            to_sq.square_minus_1_rank(),
            &piece::WHITE_PAWN,
            &piece::BLACK_PAWN,
        ),
        Colour::Black => (
            to_sq.square_plus_1_rank(),
            &piece::BLACK_PAWN,
            &piece::WHITE_PAWN,
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
    source_pce: &'static Piece,
) {
    if mv.is_capture() {
        let capt_pce = &mut None;
        position.board.get_piece_on_square(to_sq, capt_pce);

        remove_piece_from_board(position, capt_pce.unwrap(), to_sq);
    }

    let target_pce = mv.decode_promotion_piece(position.side_to_move());
    remove_piece_from_board(position, source_pce, from_sq);
    add_piece_to_board(position, target_pce, to_sq);
}

fn do_capture_move(
    position: &mut Position,
    piece_to_move: &'static Piece,
    from_sq: Square,
    to_sq: Square,
    capt_pce: &'static Piece,
) {
    remove_piece_from_board(position, capt_pce, to_sq);
    move_piece_on_board(position, piece_to_move, from_sq, to_sq);
}

#[cfg(test)]
mod tests {
    use crate::board::colour::Colour;
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::board::piece;
    use crate::board::piece::Piece;
    use crate::board::square::Square;
    use crate::io::fen;
    use crate::moves::mov::Mov;
    use crate::position::castle_permissions;
    use crate::position::game_position::MoveLegality;
    use crate::position::game_position::Position;
    use crate::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn make_move_quiet_piece_moved_hash_changed() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        let before_hash = pos.game_state.position_hash;

        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);

        // check before move
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e5,
            &piece::WHITE_PAWN
        ));

        pos.make_move(mv);

        assert!(pos.board().is_sq_empty(Square::e5));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e6,
            &piece::WHITE_PAWN
        ));
        assert_ne!(before_hash, pos.game_state.position_hash);
    }
    #[test]
    pub fn make_move_history_updated() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

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
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        // initially correct side
        assert_eq!(pos.game_state.side_to_move, Colour::White);
        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);
        pos.make_move(mv);

        assert_eq!(pos.game_state.side_to_move, Colour::Black);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_reset_on_capture_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        // set to some value
        pos.game_state.fifty_move_cntr = 21;

        let mv = Mov::encode_move_capture(Square::b5, Square::c6);
        pos.make_move(mv);

        assert_eq!(0, pos.game_state.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_reset_on_pawn_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        let pce_to_move = &mut None;
        pos.board.get_piece_on_square(Square::e5, pce_to_move);

        assert_eq!(*pce_to_move.unwrap(), piece::WHITE_PAWN);

        // set to some value
        pos.game_state.fifty_move_cntr = 21;

        let mv = Mov::encode_move_quiet(Square::e5, Square::e6);
        pos.make_move(mv);

        assert_eq!(0, pos.game_state.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_incremented_on_non_pawn_and_non_capture_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        let pce_to_move = &mut None;
        pos.board.get_piece_on_square(Square::c4, pce_to_move);

        assert_eq!(*pce_to_move.unwrap(), piece::WHITE_BISHOP);

        // set to some value
        pos.game_state.fifty_move_cntr = 21;
        let expected_cntr_val = pos.game_state.fifty_move_cntr + 1;

        let mv = Mov::encode_move_quiet(Square::c4, Square::d5);
        pos.make_move(mv);

        assert_eq!(expected_cntr_val, pos.game_state.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_half_move_cntr_incremented() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 21 32";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        let pce_to_move = &mut None;
        pos.board.get_piece_on_square(Square::c4, pce_to_move);

        assert_eq!(*pce_to_move.unwrap(), piece::WHITE_BISHOP);

        let expected_half_move = pos.game_state.move_cntr.half_move() + 1;
        let expected_full_move = pos.game_state.move_cntr.full_move() + 1;

        let mv = Mov::encode_move_quiet(Square::c4, Square::d5);
        pos.make_move(mv);

        assert_eq!(expected_half_move, pos.game_state.move_cntr.half_move());
        assert_eq!(expected_full_move, pos.game_state.move_cntr.full_move());
    }

    #[test]
    pub fn make_move_double_pawn_move_en_passant_square_set_white_moves() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f2,
            &piece::WHITE_PAWN
        ));

        // set to some value
        let mv = Mov::encode_move_double_pawn_first(Square::f2, Square::f4);
        pos.make_move(mv);

        assert_eq!(pos.game_state.en_pass_sq.unwrap(), Square::f3);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f4,
            &piece::WHITE_PAWN
        ));

        assert!(is_sq_empty(&pos, Square::f2));
    }

    #[test]
    pub fn make_move_double_pawn_move_en_passant_square_set_black_moves() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d7,
            &piece::BLACK_PAWN
        ));

        // set to some value
        let mv = Mov::encode_move_double_pawn_first(Square::d7, Square::d5);
        pos.make_move(mv);

        assert_eq!(pos.game_state.en_pass_sq, Some(Square::d6));

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d5,
            &piece::BLACK_PAWN
        ));

        assert!(is_sq_empty(&pos, Square::d7));
    }

    #[test]
    pub fn make_move_king_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e1,
            &piece::WHITE_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::h1,
            &piece::WHITE_ROOK
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
            &piece::WHITE_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f1,
            &piece::WHITE_ROOK
        ));

        assert!(!castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_king_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e8,
            &piece::BLACK_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::h8,
            &piece::BLACK_ROOK
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
            &piece::BLACK_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::f8,
            &piece::BLACK_ROOK
        ));

        assert!(!castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_queen_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e1,
            &piece::WHITE_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::a1,
            &piece::WHITE_ROOK
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
            &piece::WHITE_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d1,
            &piece::WHITE_ROOK
        ));
        assert!(!castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_queen_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::e8,
            &piece::BLACK_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::a8,
            &piece::BLACK_ROOK
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
            &piece::BLACK_KING
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d8,
            &piece::BLACK_ROOK
        ));

        assert!(!castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_en_passant_black() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/pPBP1P2/2R1NpP1/2r1r2P/R2q3n b - b3 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert_eq!(pos.en_passant_square(), Some(Square::b3));
        let mv = Mov::encode_move_en_passant(Square::a4, Square::b3);
        pos.make_move(mv);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::b3,
            &piece::BLACK_PAWN
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::b4,
            &piece::BLACK_PAWN
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::a4,
            &piece::BLACK_PAWN
        ));

        assert_eq!(pos.en_passant_square(), None);
    }

    #[test]
    pub fn make_move_en_passant_white() {
        let fen = "1n1k2bp/2p2pb1/1p5p/1B1pP1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - d6 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert_eq!(pos.en_passant_square(), Some(Square::d6));
        let mv = Mov::encode_move_en_passant(Square::e5, Square::d6);
        pos.make_move(mv);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::d6,
            &piece::WHITE_PAWN
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::d5,
            &piece::BLACK_PAWN
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::d5,
            &piece::WHITE_PAWN
        ));

        assert_eq!(pos.en_passant_square(), None);
    }

    #[test]
    pub fn make_move_promotion_capture_white_to_move() {
        let target_prom_pce = vec![
            &piece::WHITE_BISHOP,
            &piece::WHITE_KNIGHT,
            &piece::WHITE_QUEEN,
            &piece::WHITE_ROOK,
        ];

        for target in target_prom_pce {
            let fen = "kn3b1p/2p1Pp2/1p5p/1B1pb1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - - 0 1";
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

            // check pre-conditions
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::f8,
                &piece::BLACK_BISHOP
            ));

            let mv = Mov::encode_move_with_promotion_capture(Square::e7, Square::f8, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::e7));
            assert!(is_piece_on_square_as_expected(&pos, Square::f8, target));
        }
    }

    #[test]
    pub fn make_move_promotion_capture_black_to_move() {
        let target_prom_pce = vec![
            &piece::BLACK_BISHOP,
            &piece::BLACK_KNIGHT,
            &piece::BLACK_QUEEN,
            &piece::BLACK_ROOK,
        ];

        for target in target_prom_pce {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/2R5 b - - 0 1";
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

            // check pre-conditions
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::c1,
                &piece::WHITE_ROOK
            ));

            let mv = Mov::encode_move_with_promotion_capture(Square::d2, Square::c1, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::d2));
            assert!(is_piece_on_square_as_expected(&pos, Square::c1, target));
        }
    }

    #[test]
    pub fn make_move_promotion_black_to_move() {
        let target_prom_pce = vec![
            &piece::BLACK_BISHOP,
            &piece::BLACK_KNIGHT,
            &piece::BLACK_QUEEN,
            &piece::BLACK_ROOK,
        ];

        for target in target_prom_pce {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 b - - 0 1";
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );
            // check pre-conditions
            assert!(is_sq_empty(&pos, Square::d1));

            let mv = Mov::encode_move_with_promotion(Square::d2, Square::d1, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::d2));
            assert!(is_piece_on_square_as_expected(&pos, Square::d1, target));
        }
    }

    #[test]
    pub fn make_move_promotion_white_to_move() {
        let target_prom_pce = vec![
            &piece::WHITE_BISHOP,
            &piece::WHITE_KNIGHT,
            &piece::WHITE_QUEEN,
            &piece::WHITE_ROOK,
        ];

        let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 w - - 0 1";
        for target in target_prom_pce {
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

            // check pre-conditions
            assert!(is_sq_empty(&pos, Square::b8));

            let mv = Mov::encode_move_with_promotion(Square::b7, Square::b8, target);
            pos.make_move(mv);

            assert!(is_sq_empty(&pos, Square::b7));
            assert!(is_piece_on_square_as_expected(&pos, Square::b8, target));
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
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

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
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

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
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

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
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
            );

            let mv = Mov::encode_move_castle_queenside_black();

            let move_legality = pos.make_move(mv);
            assert_eq!(move_legality, MoveLegality::Illegal);
        }
    }

    #[test]
    pub fn make_move_white_king_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));

        let mv = Mov::encode_move_quiet(Square::e1, Square::e2);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(!castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_white_kings_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));

        let mv = Mov::encode_move_quiet(Square::h1, Square::g1);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_white_queens_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));

        let mv = Mov::encode_move_quiet(Square::a1, Square::b1);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_white_king_set(
            pos.castle_permissions()
        ));
        assert!(!castle_permissions::is_white_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_black_king_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));

        let mv = Mov::encode_move_quiet(Square::e8, Square::e7);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(!castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_black_kings_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));

        let mv = Mov::encode_move_quiet(Square::h8, Square::g8);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_black_queens_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );

        assert!(castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));

        let mv = Mov::encode_move_quiet(Square::a8, Square::b8);

        let move_legality = pos.make_move(mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(castle_permissions::is_black_king_set(
            pos.castle_permissions()
        ));
        assert!(!castle_permissions::is_black_queen_set(
            pos.castle_permissions()
        ));
    }

    #[test]
    pub fn make_move_take_move_position_and_board_restored_white_to_move() {
        let fen = "1b1kN3/Qp1P2p1/q2P1Nn1/PP3r2/3rPnb1/1p1pp3/B1P1P2B/R3K2R w KQ - 5 8";

        let ml = vec![
            Mov::encode_move_castle_kingside_white(),
            Mov::encode_move_castle_queenside_white(),
            Mov::encode_move_capture(Square::e8, Square::g7),
            Mov::encode_move_quiet(Square::b5, Square::b6),
            Mov::encode_move_double_pawn_first(Square::c2, Square::c4),
        ];

        let (board1, move_cntr1, castle_permissions1, side_to_move1, en_pass_sq1) =
            fen::decompose_fen(fen);

        let zobrist_keys1 = ZobristKeys::new();
        let occ_masks1 = OccupancyMasks::new();

        let mut pos1 = Position::new(
            board1,
            castle_permissions1,
            move_cntr1,
            en_pass_sq1,
            side_to_move1,
            &zobrist_keys1,
            &occ_masks1,
        );

        let (board2, move_cntr2, castle_permissions2, side_to_move2, en_pass_sq2) =
            fen::decompose_fen(fen);

        let occ_masks2 = OccupancyMasks::new();

        // note : use the same Zobrist keys - else the position equlaty will fail
        let pos2 = Position::new(
            board2,
            castle_permissions2,
            move_cntr2,
            en_pass_sq2,
            side_to_move2,
            &zobrist_keys1,
            &occ_masks2,
        );

        for mv in ml {
            println!("move: {}", mv);
            pos1.make_move(mv);
            assert_ne!(pos1, pos2);

            pos1.take_move();

            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    pub fn make_move_take_move_position_and_board_restored_black_to_move() {
        let fen = "r3k2r/1pb2p2/qQ1P2n1/PPPN2N1/4Pnb1/1p1pp3/B1P1P2B/R3K2R b kq - 3 11";

        let ml = vec![
            Mov::encode_move_castle_kingside_black(),
            Mov::encode_move_castle_queenside_black(),
            Mov::encode_move_capture(Square::c7, Square::b6),
            Mov::encode_move_quiet(Square::f7, Square::f6),
            Mov::encode_move_double_pawn_first(Square::f7, Square::f6),
        ];

        let (board1, move_cntr1, castle_permissions1, side_to_move1, en_pass_sq1) =
            fen::decompose_fen(fen);

        let zobrist_keys1 = ZobristKeys::new();
        let occ_masks1 = OccupancyMasks::new();

        let mut pos1 = Position::new(
            board1,
            castle_permissions1,
            move_cntr1,
            en_pass_sq1,
            side_to_move1,
            &zobrist_keys1,
            &occ_masks1,
        );

        let (board2, move_cntr2, castle_permissions2, side_to_move2, en_pass_sq2) =
            fen::decompose_fen(fen);

        let occ_masks2 = OccupancyMasks::new();

        // note : use the same Zobrist keys - else the position equlaty will fail
        let pos2 = Position::new(
            board2,
            castle_permissions2,
            move_cntr2,
            en_pass_sq2,
            side_to_move2,
            &zobrist_keys1,
            &occ_masks2,
        );

        // initial states are the same
        assert_eq!(pos1, pos2);

        for mv in ml {
            pos1.make_move(mv);
            assert_ne!(pos1, pos2);

            pos1.take_move();

            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    pub fn make_move_hash_updated_white_double_pawn_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&piece::WHITE_PAWN, Square::b2);
        expected_hash ^= zobrist_keys.piece_square(&piece::WHITE_PAWN, Square::b4);
        expected_hash ^= zobrist_keys.en_passant(Square::b3);
        expected_hash ^= zobrist_keys.side();

        let wp_double_mv = Mov::encode_move_double_pawn_first(Square::b2, Square::b4);
        pos.make_move(wp_double_mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    #[test]
    pub fn make_move_hash_updated_black_double_pawn_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&piece::BLACK_PAWN, Square::b7);
        expected_hash ^= zobrist_keys.piece_square(&piece::BLACK_PAWN, Square::b5);
        expected_hash ^= zobrist_keys.en_passant(Square::b6);
        expected_hash ^= zobrist_keys.side();

        let bp_double_mv = Mov::encode_move_double_pawn_first(Square::b7, Square::b5);
        pos.make_move(bp_double_mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    #[test]
    pub fn make_move_hash_updated_white_quiet_move() {
        let fen = "r1bqkbnr/pp1n1p1p/2pp4/4p1p1/1P1P4/5PP1/P1P1PN1P/RNBQKB1R w KQkq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&piece::WHITE_KNIGHT, Square::f2);
        expected_hash ^= zobrist_keys.piece_square(&piece::WHITE_KNIGHT, Square::g4);
        expected_hash ^= zobrist_keys.side();

        let wp_double_mv = Mov::encode_move_quiet(Square::f2, Square::g4);
        pos.make_move(wp_double_mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    #[test]
    pub fn make_move_hash_updated_black_quiet_move() {
        let fen = "r1bqkbnr/pp1n1p1p/2pp4/4p1p1/1P1P4/5PP1/P1P1PN1P/RNBQKB1R b KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&piece::BLACK_KNIGHT, Square::f6);
        expected_hash ^= zobrist_keys.piece_square(&piece::BLACK_KNIGHT, Square::d7);
        expected_hash ^= zobrist_keys.side();

        let wp_double_mv = Mov::encode_move_quiet(Square::d7, Square::f6);
        pos.make_move(wp_double_mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    #[test]
    pub fn make_move_hash_updated_black_en_passant_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/pPBP1P2/2R1NpP1/2r1r2P/R2q3n b - b3 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );
        let init_hash = pos.position_hash();

        // remove white pawn on b4
        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&piece::WHITE_PAWN, Square::b4);
        // move a4->b3
        expected_hash ^= zobrist_keys.piece_square(&piece::BLACK_PAWN, Square::a4);
        expected_hash ^= zobrist_keys.piece_square(&piece::BLACK_PAWN, Square::b3);
        expected_hash ^= zobrist_keys.en_passant(Square::b3);
        expected_hash ^= zobrist_keys.side();

        assert_eq!(pos.en_passant_square(), Some(Square::b3));
        let mv = Mov::encode_move_en_passant(Square::a4, Square::b3);
        pos.make_move(mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    #[test]
    pub fn make_move_hash_updated_white_en_passant() {
        let fen = "1n1k2bp/2p2pb1/1p5p/1B1pP1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - d6 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
        );
        let init_hash = pos.position_hash();

        // remove black pawn
        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&piece::BLACK_PAWN, Square::d5);
        // move e5->d6
        expected_hash ^= zobrist_keys.piece_square(&piece::WHITE_PAWN, Square::e5);
        expected_hash ^= zobrist_keys.piece_square(&piece::WHITE_PAWN, Square::d6);
        expected_hash ^= zobrist_keys.en_passant(Square::d6);
        expected_hash ^= zobrist_keys.side();

        assert_eq!(pos.en_passant_square(), Some(Square::d6));
        let mv = Mov::encode_move_en_passant(Square::e5, Square::d6);
        pos.make_move(mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    fn is_piece_on_square_as_expected(pos: &Position, sq: Square, pce: &'static Piece) -> bool {
        let pce_on_board = &mut None;
        pos.board.get_piece_on_square(sq, pce_on_board);

        if pce_on_board.is_none() {
            return false;
        }

        pce_on_board.unwrap() == pce
    }

    fn is_sq_empty(pos: &Position, sq: Square) -> bool {
        let pce_on_board = &mut None;
        pos.board.get_piece_on_square(sq, pce_on_board);
        pce_on_board.is_none()
    }
}
