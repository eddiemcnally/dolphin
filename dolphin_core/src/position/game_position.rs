use crate::board::colour::Colour;
use crate::board::file::File;
use crate::board::game_board::Board;
use crate::board::occupancy_masks::OccupancyMasks;
use crate::board::piece::Piece;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::moves::mov::Move;
use crate::moves::mov::MoveType;
use crate::position::attack_checker::AttackChecker;
use crate::position::castle_permissions::CastlePermission;
use crate::position::move_counter::MoveCounter;
use crate::position::position_history::PositionHistory;
use crate::position::zobrist_keys::ZobristHash;
use crate::position::zobrist_keys::ZobristKeys;
use std::fmt;
use std::process;

// something to avoid bugs with bool states
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum MoveLegality {
    Legal,
    Illegal,
}

const CASTLE_SQUARES_KING_WHITE: [Square; 3] = [Square::E1, Square::F1, Square::G1];

const CASTLE_SQUARES_QUEEN_WHITE: [Square; 3] = [Square::C1, Square::D1, Square::E1];

const CASTLE_SQUARES_KING_BLACK: [Square; 3] = [Square::E8, Square::F8, Square::G8];

const CASTLE_SQUARES_QUEEN_BLACK: [Square; 3] = [Square::C8, Square::D8, Square::E8];

pub struct Position<'a> {
    board: Board,
    position_history: Box<PositionHistory>,
    occ_masks: &'a OccupancyMasks,
    zobrist_keys: &'a ZobristKeys,
    attack_checker: &'a AttackChecker,
    game_state: GameState,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct GameState {
    position_hash: ZobristHash,
    move_cntr: MoveCounter,
    side_to_move: Colour,
    en_pass_sq: Option<Square>,
    castle_perm: CastlePermission,
    fifty_move_cntr: u8,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            side_to_move: Colour::White,
            position_hash: 0,
            move_cntr: MoveCounter::default(),
            fifty_move_cntr: 0,
            en_pass_sq: None,
            castle_perm: CastlePermission::NO_CASTLE_PERMS_AVAIL,
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState::default()
    }
    pub fn get_zobrist_hash(&self) -> ZobristHash {
        self.position_hash
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
        attack_checker: &'a AttackChecker,
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
            attack_checker,
            zobrist_keys,
        };

        // generate position hash
        pos.board.get_bitboard().iterator().for_each(|sq| {
            if let Some((piece, colour)) = pos.board().get_piece_and_colour_on_square(&sq) {
                pos.game_state.position_hash ^= pos.zobrist_keys.piece_square(&piece, &colour, &sq);
            };
        });

        pos.game_state.position_hash ^= pos.zobrist_keys.side();

        if castle_permissions.is_black_king_set() {
            pos.game_state.position_hash ^= pos.zobrist_keys.castle_permissions_black_king();
        }
        if castle_permissions.is_white_king_set() {
            pos.game_state.position_hash ^= pos.zobrist_keys.castle_permissions_white_king();
        }
        if castle_permissions.is_black_queen_set() {
            pos.game_state.position_hash ^= pos.zobrist_keys.castle_permissions_black_queen();
        }
        if castle_permissions.is_white_queen_set() {
            pos.game_state.position_hash ^= pos.zobrist_keys.castle_permissions_white_queen();
        }

        if let Some(_enp) = en_passant_sq {
            pos.game_state.position_hash ^= pos.zobrist_keys.en_passant(&en_passant_sq.unwrap());
        }

        // validate position
        let bk_bb = pos.board().get_piece_bitboard(&Piece::King, &Colour::Black);
        assert!(!bk_bb.is_empty());
        let wk_bb = pos.board().get_piece_bitboard(&Piece::King, &Colour::White);
        assert!(!wk_bb.is_empty());

        pos
    }

    pub fn side_to_move(&self) -> Colour {
        self.game_state.side_to_move
    }

    pub const fn board(&self) -> &Board {
        &self.board
    }

    pub const fn en_passant_square(&self) -> Option<Square> {
        self.game_state.en_pass_sq
    }

    pub const fn is_en_passant_active(&self) -> bool {
        self.game_state.en_pass_sq.is_some()
    }

    pub const fn castle_permissions(&self) -> CastlePermission {
        self.game_state.castle_perm
    }

    pub const fn move_counter(&self) -> &MoveCounter {
        &self.game_state.move_cntr
    }

    pub const fn position_hash(&self) -> ZobristHash {
        self.game_state.position_hash
    }

    pub const fn occupancy_masks(&self) -> &'a OccupancyMasks {
        self.occ_masks
    }

    pub fn flip_side_to_move(&mut self) {
        self.game_state.side_to_move = self.side_to_move().flip_side();
        self.game_state.position_hash ^= self.zobrist_keys.side();
    }

    pub fn is_repetition(&self) -> bool {
        let start_offset =
            self.move_counter().half_move() as usize - self.game_state.fifty_move_cntr as usize;

        self.position_history
            .contains_position_hash(&self.position_hash(), start_offset)
    }

    pub fn is_king_sq_attacked(&self) -> bool {
        let king_sq = self.board.get_king_sq(&self.side_to_move());
        let opp_side = self.side_to_move().flip_side();
        self.attack_checker
            .is_sq_attacked(self.occ_masks, self.board(), &king_sq, &opp_side)
    }

    fn save_game_state(&mut self, mv: &Move) -> Option<Piece> {
        match mv.move_type() {
            MoveType::Normal | MoveType::Promotion => {
                let to_sq = mv.to_sq();
                let capt_pce = self.board.get_piece_on_square(&to_sq);
                self.position_history.push(&self.game_state, mv, &capt_pce);
                return capt_pce;
            }
            MoveType::EnPassant => {
                self.position_history
                    .push(&self.game_state, mv, &Some(Piece::Pawn));
                return Some(Piece::Pawn);
            }
            MoveType::Castle => {
                self.position_history.push(&self.game_state, mv, &None);
                return None;
            }
        }
    }

    pub fn make_move(&mut self, mv: &Move) -> MoveLegality {
        let capt_pce = self.save_game_state(mv);
        let pce_to_move = self
            .board
            .get_piece_on_square(&mv.from_sq())
            .expect("Unepxected empty square");
        self.update_move_counters(&capt_pce, &pce_to_move);

        match mv.move_type() {
            MoveType::Normal => self.do_normal_move(mv),
            MoveType::Promotion => self.do_promotion_move(mv),
            MoveType::EnPassant => self.do_en_passant(mv),
            MoveType::Castle => self.do_castle_move(mv),
        }

        // update some states based on the move
        self.update_en_passant_sq(mv, &pce_to_move);
        if self.game_state.castle_perm.has_castle_permission() {
            self.update_castle_perms(mv, &pce_to_move, &capt_pce);
        }

        let move_legality = self.get_move_legality(mv);

        self.flip_side_to_move();
        move_legality
    }

    fn do_normal_move(&mut self, mv: &Move) {
        let (from_sq, to_sq) = mv.decode_from_to_sq();

        if let Some(pce) = self.board.get_piece_on_square(&to_sq) {
            // capture
            self.remove_piece_from_board(&pce, &self.side_to_move().flip_side(), &to_sq);
        };

        let pce_to_move = self
            .board
            .get_piece_on_square(&from_sq)
            .expect("Expecting piece on from sq");

        self.move_piece_on_board(&pce_to_move, &self.side_to_move(), &from_sq, &to_sq);

        if self.is_double_pawn_move(mv, &pce_to_move) {
            let s = self.find_en_passant_sq(&mv.from_sq(), &self.side_to_move());
            self.game_state.en_pass_sq = Some(s);
            self.game_state.position_hash ^= self.zobrist_keys.en_passant(&s);
        }
    }

    fn find_en_passant_sq(&self, from_sq: &Square, col: &Colour) -> Square {
        // use the *from_sq* to find the en passant sq
        match col {
            Colour::White => from_sq.north().expect("Invalid north() en passant square"),
            Colour::Black => from_sq.south().expect("Invalid south() en passant square"),
        }
    }

    fn do_promotion_move(&mut self, mv: &Move) {
        let (from_sq, to_sq) = mv.decode_from_to_sq();

        if let Some(pce) = self.board.get_piece_on_square(&to_sq) {
            // capture
            self.remove_piece_from_board(&pce, &self.side_to_move().flip_side(), &to_sq);
        }

        // remove the pawn being moved
        self.remove_piece_from_board(&Piece::Pawn, &self.side_to_move(), &from_sq);
        // add the promoted piece
        let promo_pce = mv.decode_promotion_piece();
        self.add_piece_to_board(&promo_pce, &self.side_to_move(), &to_sq)
    }

    fn do_en_passant(&mut self, mv: &Move) {
        let side_to_move = self.side_to_move();

        let (col_to_move, col_to_capt, capt_sq) = match side_to_move {
            Colour::White => (Colour::White, Colour::Black, mv.to_sq().south()),
            Colour::Black => (Colour::Black, Colour::White, mv.to_sq().north()),
        };

        self.remove_piece_from_board(
            &Piece::Pawn,
            &col_to_capt,
            &capt_sq.expect("Invalid capture square"),
        );
        self.move_piece_on_board(&Piece::Pawn, &col_to_move, &mv.from_sq(), &mv.to_sq());
    }

    pub fn take_move(&mut self) {
        self.flip_side_to_move();

        // restore state
        let (gs, mv, capt_pce) = self.position_history.pop();
        self.game_state = gs;

        match mv.move_type() {
            MoveType::Normal => self.reverse_normal_move(&mv, &capt_pce),
            MoveType::Promotion => self.reverse_promotion_move(&mv, &capt_pce),
            MoveType::EnPassant => self.reverse_en_passant_move(&mv),
            MoveType::Castle => self.reverse_castle_move(&mv),
        }
    }

    fn reverse_normal_move(&mut self, mv: &Move, capt_pce: &Option<Piece>) {
        let pce_moved = self
            .board
            .get_piece_on_square(&mv.to_sq())
            .expect("Unexpected empty square");

        // revert move
        self.board
            .move_piece(&mv.to_sq(), &mv.from_sq(), &pce_moved, &self.side_to_move());

        if capt_pce.is_some() {
            // add back the captured piece
            self.board.add_piece(
                &capt_pce.unwrap(),
                &self.side_to_move().flip_side(),
                &mv.to_sq(),
            );
        }

        if self.is_double_pawn_move(mv, &pce_moved) {
            self.game_state.en_pass_sq = None;
        }
    }
    fn reverse_promotion_move(&mut self, mv: &Move, capt_pce: &Option<Piece>) {
        // remove promoted piece
        let prom_piece = mv.decode_promotion_piece();
        self.board
            .remove_piece(&prom_piece, &self.side_to_move(), &mv.to_sq());

        // put the moved piece back to it's original square
        self.board
            .add_piece(&Piece::Pawn, &self.side_to_move(), &mv.from_sq());

        // replace the captured piece
        if capt_pce.is_some() {
            self.board.add_piece(
                &capt_pce.unwrap(),
                &self.side_to_move().flip_side(),
                &mv.to_sq(),
            );
        }
    }

    fn reverse_en_passant_move(&mut self, mv: &Move) {
        match self.side_to_move() {
            Colour::White => {
                self.board
                    .move_piece(&mv.to_sq(), &mv.from_sq(), &Piece::Pawn, &Colour::White);

                let capt_sq = mv.to_sq().south();
                self.board.add_piece(
                    &Piece::Pawn,
                    &Colour::Black,
                    &capt_sq.expect("Invalid capture square"),
                );
            }
            Colour::Black => {
                self.board
                    .move_piece(&mv.to_sq(), &mv.from_sq(), &Piece::Pawn, &Colour::Black);

                let capt_sq = mv.to_sq().north().expect("Invalid north() square");
                self.board.add_piece(&Piece::Pawn, &Colour::White, &capt_sq);
            }
        }
    }

    fn do_castle_move(&mut self, mv: &Move) {
        let colour = self.side_to_move();

        let (from_sq, to_sq) = mv.decode_from_to_sq();

        match (from_sq, to_sq) {
            (Square::E1, Square::G1) => {
                // white king castle
                self.move_piece_on_board(&Piece::King, &Colour::White, &Square::E1, &Square::G1);
                self.move_piece_on_board(&Piece::Rook, &Colour::White, &Square::H1, &Square::F1);
            }
            (Square::E8, Square::G8) => {
                // black king castle
                self.move_piece_on_board(&Piece::King, &Colour::Black, &Square::E8, &Square::G8);
                self.move_piece_on_board(&Piece::Rook, &Colour::Black, &Square::H8, &Square::F8);
            }
            (Square::E1, Square::C1) => {
                // white queen castle
                self.move_piece_on_board(&Piece::King, &Colour::White, &Square::E1, &Square::C1);
                self.move_piece_on_board(&Piece::Rook, &Colour::White, &Square::A1, &Square::D1);
            }
            (Square::E8, Square::C8) => {
                // black queen castle
                self.move_piece_on_board(&Piece::King, &Colour::Black, &Square::E8, &Square::C8);
                self.move_piece_on_board(&Piece::Rook, &Colour::Black, &Square::A8, &Square::D8);
            }
            _ => {
                eprintln!("Invalid Castle move");
                process::exit(1);
            }
        }

        self.clear_castle_permissions_for_colour(&colour);
    }

    fn reverse_castle_move(&mut self, mv: &Move) {
        let (from_sq, to_sq) = mv.decode_from_to_sq();

        match (from_sq, to_sq) {
            (Square::E1, Square::G1) => {
                // white king castle
                self.board
                    .move_piece(&Square::G1, &Square::E1, &Piece::King, &Colour::White);
                self.board
                    .move_piece(&Square::F1, &Square::H1, &Piece::Rook, &Colour::White);
            }
            (Square::E8, Square::G8) => {
                // black king castle
                self.board
                    .move_piece(&Square::G8, &Square::E8, &Piece::King, &Colour::Black);
                self.board
                    .move_piece(&Square::F8, &Square::H8, &Piece::Rook, &Colour::Black);
            }
            (Square::E1, Square::C1) => {
                // white queen castle
                self.board
                    .move_piece(&Square::C1, &Square::E1, &Piece::King, &Colour::White);
                self.board
                    .move_piece(&Square::D1, &Square::A1, &Piece::Rook, &Colour::White);
            }
            (Square::E8, Square::C8) => {
                // black queen castle
                self.board
                    .move_piece(&Square::C8, &Square::E8, &Piece::King, &Colour::Black);
                self.board
                    .move_piece(&Square::D8, &Square::A8, &Piece::Rook, &Colour::Black);
            }
            _ => {
                eprintln!("Invalid castle move");
                process::exit(1);
            }
        }
    }

    fn get_move_legality(&self, mv: &Move) -> MoveLegality {
        // check if move results in king being in check
        let king_sq = self.board().get_king_sq(&self.game_state.side_to_move);
        let attacking_side = self.game_state.side_to_move.flip_side();

        if self.attack_checker.is_sq_attacked(
            self.occ_masks,
            self.board(),
            &king_sq,
            &attacking_side,
        ) {
            return MoveLegality::Illegal;
        }

        // check castle through attacked squares (or king was in check before the castle move)
        if mv.move_type() == MoveType::Castle {
            let squares_to_check = if mv.to_sq().file() == File::G {
                match self.game_state.side_to_move {
                    Colour::White => &CASTLE_SQUARES_KING_WHITE,
                    Colour::Black => &CASTLE_SQUARES_KING_BLACK,
                }
            } else if mv.to_sq().file() == File::C {
                match self.game_state.side_to_move {
                    Colour::White => &CASTLE_SQUARES_QUEEN_WHITE,
                    Colour::Black => &CASTLE_SQUARES_QUEEN_BLACK,
                }
            } else {
                eprintln!("Invalid move");
                process::exit(1);
            };

            let is_invalid_castle = self.attack_checker.is_castle_squares_attacked(
                self.occ_masks,
                self.board(),
                squares_to_check,
                &attacking_side,
            );

            if is_invalid_castle {
                return MoveLegality::Illegal;
            } else {
                return MoveLegality::Legal;
            }
        }
        MoveLegality::Legal
    }

    fn is_double_pawn_move(&self, mv: &Move, pce_moved: &Piece) -> bool {
        if *pce_moved != Piece::Pawn {
            return false;
        }

        let from_rank = mv.from_sq().rank();
        let to_rank = mv.to_sq().rank();

        match (from_rank, to_rank) {
            (Rank::R2, Rank::R4) => true,
            (Rank::R7, Rank::R5) => true,
            _ => false,
        }
    }

    fn update_en_passant_sq(&mut self, mv: &Move, pce_moved: &Piece) {
        // clear en passant
        if self.game_state.en_pass_sq.is_some() && !self.is_double_pawn_move(mv, pce_moved) {
            self.game_state.position_hash ^= self
                .zobrist_keys
                .en_passant(&self.game_state.en_pass_sq.unwrap());
            self.game_state.en_pass_sq = None;
        }
    }

    // remove castle permissions based on the move
    fn update_castle_perms(&mut self, mv: &Move, pce_moved: &Piece, capt_pce: &Option<Piece>) {
        if mv.move_type() == MoveType::Castle {
            // permissions already adjusted when move made
            return;
        }

        // check if rook has just been captured
        if *capt_pce == Some(Piece::Rook) {
            match mv.to_sq() {
                Square::A1 => self.game_state.castle_perm.clear_queen_white(),
                Square::H1 => self.game_state.castle_perm.clear_king_white(),
                Square::A8 => self.game_state.castle_perm.clear_queen_black(),
                Square::H8 => self.game_state.castle_perm.clear_king_black(),
                _ => (),
            }
        }

        // check if king or rook have moved
        match pce_moved {
            Piece::King => match self.side_to_move() {
                Colour::White => self.game_state.castle_perm.clear_white_king_and_queen(),
                Colour::Black => self.game_state.castle_perm.clear_black_king_and_queen(),
            },
            Piece::Rook => match self.side_to_move() {
                Colour::White => {
                    match mv.from_sq() {
                        Square::A1 => self.game_state.castle_perm.clear_queen_white(),
                        Square::H1 => self.game_state.castle_perm.clear_king_white(),
                        _ => (),
                    };
                }
                Colour::Black => {
                    match mv.from_sq() {
                        Square::A8 => self.game_state.castle_perm.clear_queen_black(),
                        Square::H8 => self.game_state.castle_perm.clear_king_black(),
                        _ => (),
                    };
                }
            },
            _ => (),
        }
    }

    fn remove_piece_from_board(&mut self, pce: &Piece, colour: &Colour, sq: &Square) {
        self.board.remove_piece(&pce, &colour, &sq);
        self.game_state.position_hash ^= self.zobrist_keys.piece_square(&pce, &colour, &sq);
    }

    fn add_piece_to_board(&mut self, pce: &Piece, colour: &Colour, sq: &Square) {
        self.board.add_piece(&pce, &colour, &sq);
        self.game_state.position_hash ^= self.zobrist_keys.piece_square(&pce, &colour, &sq);
    }

    fn move_piece_on_board(
        &mut self,
        pce: &Piece,
        colour: &Colour,
        from_sq: &Square,
        to_sq: &Square,
    ) {
        self.game_state.position_hash ^= self.zobrist_keys.piece_square(&pce, &colour, &from_sq);
        self.game_state.position_hash ^= self.zobrist_keys.piece_square(&pce, &colour, &to_sq);
        self.board.move_piece(&from_sq, &to_sq, &pce, &colour);
    }

    fn update_move_counters(&mut self, capt_pce: &Option<Piece>, pce_moved: &Piece) {
        let full_move_incr = self.game_state.move_cntr.incr_half_move();

        if full_move_incr {
            // handle 50 move rule
            if capt_pce.is_some() || *pce_moved == Piece::Pawn {
                self.game_state.fifty_move_cntr = 0;
            } else {
                self.game_state.fifty_move_cntr += 1;
            }
        }
    }
    fn clear_castle_permissions_for_colour(&mut self, col: &Colour) {
        match col {
            Colour::White => {
                self.game_state.castle_perm.clear_white_king_and_queen();
                self.game_state.position_hash ^= self.zobrist_keys.castle_permissions_white_king();
                self.game_state.position_hash ^= self.zobrist_keys.castle_permissions_white_queen();
            }
            Colour::Black => {
                self.game_state.castle_perm.clear_black_king_and_queen();
                self.game_state.position_hash ^= self.zobrist_keys.castle_permissions_black_king();
                self.game_state.position_hash ^= self.zobrist_keys.castle_permissions_black_queen();
            }
        }
    }
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

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl fmt::Debug for Position<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("Board : {}\n", self.board()));
        debug_str.push_str(&format!("SideToMove : {}\n", self.game_state.side_to_move));
        if self.game_state.en_pass_sq.is_none() {
            debug_str.push_str("En pass Sq : -\n");
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

#[cfg(test)]
mod tests {
    use crate::board::colour::Colour;
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::board::piece::Piece;
    use crate::board::square::Square;
    use crate::io::fen;
    use crate::moves::mov::*;
    use crate::position::attack_checker::AttackChecker;
    use crate::position::game_position::process;

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let before_hash = pos.game_state.position_hash;

        let mv = Move::encode_move(&Square::E5, &Square::E6);

        // check before move
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::E5,
            Piece::Pawn,
            Colour::White
        ));

        pos.make_move(&mv);

        assert!(pos.board().is_sq_empty(&Square::E5));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::E6,
            Piece::Pawn,
            Colour::White
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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        // initially no history
        assert_eq!(pos.position_history.len(), 0);
        let mv = Move::encode_move(&Square::E5, &Square::E6);
        pos.make_move(&mv);

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        // initially correct side
        assert_eq!(pos.game_state.side_to_move, Colour::White);
        let mv = Move::encode_move(&Square::E5, &Square::E6);
        pos.make_move(&mv);

        assert_eq!(pos.game_state.side_to_move, Colour::Black);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_reset_on_capture_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 5 11";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.game_state.move_cntr.half_move() == 5);
        assert!(pos.game_state.move_cntr.full_move() == 11);

        // set to some random value
        pos.game_state.fifty_move_cntr = 21;

        let mv = Move::encode_move(&Square::B5, &Square::C6);
        pos.make_move(&mv);

        assert_eq!(0, pos.game_state.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_reset_on_pawn_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 5 11";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        if let Some((piece, _colour)) = pos.board.get_piece_and_colour_on_square(&Square::E5) {
            assert_eq!(piece, Piece::Pawn);
        } else {
            eprintln!("Piece not found");
            process::exit(1);
        }

        assert!(pos.game_state.move_cntr.half_move() == 5);
        assert!(pos.game_state.move_cntr.full_move() == 11);

        // set to some value
        pos.game_state.fifty_move_cntr = 21;

        let mv = Move::encode_move(&Square::E5, &Square::E6);
        pos.make_move(&mv);

        assert_eq!(0, pos.game_state.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_fifty_move_cntr_incremented_on_non_pawn_and_non_capture_move() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 5 11";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        if let Some((piece, _colour)) = pos.board.get_piece_and_colour_on_square(&Square::C4) {
            assert_eq!(piece, Piece::Bishop);
        } else {
            eprintln!("Piece not found");
            process::exit(1);
        }

        assert!(pos.game_state.move_cntr.half_move() == 5);
        assert!(pos.game_state.move_cntr.full_move() == 11);

        // set to some value
        pos.game_state.fifty_move_cntr = 21;
        let expected_cntr_val = pos.game_state.fifty_move_cntr + 1;

        let mv = Move::encode_move(&Square::C4, &Square::D5);
        pos.make_move(&mv);

        assert_eq!(expected_cntr_val, pos.game_state.fifty_move_cntr);
    }

    #[test]
    pub fn make_move_half_move_cntr_incremented() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 21 32";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        if let Some((piece, _colour)) = pos.board.get_piece_and_colour_on_square(&Square::C4) {
            assert_eq!(piece, Piece::Bishop);
        } else {
            eprintln!("Piece not found");
            process::exit(1);
        }

        let expected_half_move = pos.game_state.move_cntr.half_move() + 1;

        let mv = Move::encode_move(&Square::C4, &Square::D5);
        pos.make_move(&mv);

        assert_eq!(expected_half_move, pos.game_state.move_cntr.half_move());
    }

    #[test]
    pub fn make_move_double_pawn_move_en_passant_square_set_white_moves() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::F2,
            Piece::Pawn,
            Colour::White
        ));

        // set to some value
        let mv = Move::encode_move(&Square::F2, &Square::F4);
        pos.make_move(&mv);

        assert_eq!(pos.game_state.en_pass_sq.unwrap(), Square::F3);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::F4,
            Piece::Pawn,
            Colour::White
        ));

        assert!(is_sq_empty(&pos, Square::F2));
    }

    #[test]
    pub fn make_move_double_pawn_move_en_passant_square_set_black_moves() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2PPK1/1RB5/pPR1N2p/P1r1rP1P/P2q3n b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::D7,
            Piece::Pawn,
            Colour::Black
        ));

        // set to some value
        let mv = Move::encode_move(&Square::D7, &Square::D5);
        pos.make_move(&mv);

        assert_eq!(pos.game_state.en_pass_sq, Some(Square::D6));

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::D5,
            Piece::Pawn,
            Colour::Black
        ));

        assert!(is_sq_empty(&pos, Square::D7));
    }

    #[test]
    pub fn make_move_king_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_white_king_set());
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::E1,
            Piece::King,
            Colour::White
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::H1,
            Piece::Rook,
            Colour::White
        ));
        let mv = Move::encode_move_castle_kingside_white();
        pos.make_move(&mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::E1));
        assert!(is_sq_empty(&pos, Square::H1));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::G1,
            Piece::King,
            Colour::White
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::F1,
            Piece::Rook,
            Colour::White
        ));

        assert!(!pos.castle_permissions().is_white_king_set());
    }

    #[test]
    pub fn make_move_king_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_black_king_set());
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::E8,
            Piece::King,
            Colour::Black
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::H8,
            Piece::Rook,
            Colour::Black
        ));
        let mv = Move::encode_move_castle_kingside_black();
        pos.make_move(&mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::E8));
        assert!(is_sq_empty(&pos, Square::H8));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::G8,
            Piece::King,
            Colour::Black
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::F8,
            Piece::Rook,
            Colour::Black
        ));

        assert!(!pos.castle_permissions().is_black_king_set());
    }

    #[test]
    pub fn make_move_queen_side_castle_white() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R w KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_white_queen_set());
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::E1,
            Piece::King,
            Colour::White
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::A1,
            Piece::Rook,
            Colour::White
        ));
        let mv = Move::encode_move_castle_queenside_white();
        pos.make_move(&mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::E1));
        assert!(is_sq_empty(&pos, Square::A1));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::C1,
            Piece::King,
            Colour::White
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::D1,
            Piece::Rook,
            Colour::White
        ));
        assert!(!pos.castle_permissions().is_white_queen_set());
    }

    #[test]
    pub fn make_move_queen_side_castle_black() {
        let fen = "r3k2r/pppq1ppp/2np1n2/4pb2/1bB1P1Q1/2NPB3/PPP1NPPP/R3K2R b KQkq - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_black_queen_set());
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::E8,
            Piece::King,
            Colour::Black
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::A8,
            Piece::Rook,
            Colour::Black
        ));
        let mv = Move::encode_move_castle_queenside_black();
        pos.make_move(&mv);

        // check old squares are no long occupied
        assert!(is_sq_empty(&pos, Square::E8));
        assert!(is_sq_empty(&pos, Square::A8));
        // check new squares are occupied
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::C8,
            Piece::King,
            Colour::Black
        ));
        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::D8,
            Piece::Rook,
            Colour::Black
        ));

        assert!(!pos.castle_permissions().is_black_queen_set());
    }

    #[test]
    pub fn make_move_en_passant_black() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/pPBP1P2/2R1NpP1/2r1r2P/R2q3n b - b3 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert_eq!(pos.en_passant_square(), Some(Square::B3));
        let mv = Move::encode_move_en_passant(&Square::A4, &Square::B3);
        pos.make_move(&mv);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::B3,
            Piece::Pawn,
            Colour::Black
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::B4,
            Piece::Pawn,
            Colour::White
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::A4,
            Piece::Pawn,
            Colour::Black
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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert_eq!(pos.en_passant_square(), Some(Square::D6));
        let mv = Move::encode_move_en_passant(&Square::E5, &Square::D6);
        pos.make_move(&mv);

        assert!(is_piece_on_square_as_expected(
            &pos,
            Square::D6,
            Piece::Pawn,
            Colour::White
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::D5,
            Piece::Pawn,
            Colour::Black
        ));

        assert!(!is_piece_on_square_as_expected(
            &pos,
            Square::D5,
            Piece::Pawn,
            Colour::White
        ));

        assert_eq!(pos.en_passant_square(), None);
    }

    #[test]
    pub fn make_move_promotion_capture_white_to_move() {
        let target_prom_role = vec![Piece::Bishop, Piece::Knight, Piece::Queen, Piece::Rook];

        for target in target_prom_role {
            let fen = "kn3b1p/2p1Pp2/1p5p/1B1pb1K1/pPBP1P2/N1R1NpPQ/P1r1r2P/R2q3n w - - 0 1";
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            // check pre-conditions
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::F8,
                Piece::Bishop,
                Colour::Black
            ));

            let mv = Move::encode_move_with_promotion(&Square::E7, &Square::F8, &target);
            pos.make_move(&mv);

            assert!(is_sq_empty(&pos, Square::E7));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::F8,
                target,
                Colour::White
            ));
        }
    }

    #[test]
    pub fn make_move_promotion_capture_black_to_move() {
        let target_prom_role = vec![Piece::Bishop, Piece::Knight, Piece::Queen, Piece::Rook];

        for target in target_prom_role {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/2R5 b - - 0 1";
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            // check pre-conditions
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::C1,
                Piece::Rook,
                Colour::White
            ));

            let mv = Move::encode_move_with_promotion(&Square::D2, &Square::C1, &target);
            pos.make_move(&mv);

            assert!(is_sq_empty(&pos, Square::D2));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::C1,
                target,
                Colour::Black
            ));
        }
    }

    #[test]
    pub fn make_move_promotion_black_to_move() {
        let target_prom_role = vec![Piece::Bishop, Piece::Knight, Piece::Queen, Piece::Rook];

        for target in target_prom_role {
            let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 b - - 0 1";
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );
            // check pre-conditions
            assert!(is_sq_empty(&pos, Square::D1));

            let mv = Move::encode_move_with_promotion(&Square::D2, &Square::D1, &target);
            pos.make_move(&mv);

            assert!(is_sq_empty(&pos, Square::D2));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::D1,
                target,
                Colour::Black
            ));
        }
    }

    #[test]
    pub fn make_move_promotion_white_to_move() {
        let target_prom_role = vec![Piece::Bishop, Piece::Knight, Piece::Queen, Piece::Rook];

        let fen = "3b2KN/PP1P4/1Bb1p3/rk5P/5RP1/4p3/3ppnBp/R7 w - - 0 1";
        for target in target_prom_role {
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            // check pre-conditions
            assert!(is_sq_empty(&pos, Square::B8));

            let mv = Move::encode_move_with_promotion(&Square::B7, &Square::B8, &target);
            pos.make_move(&mv);

            assert!(is_sq_empty(&pos, Square::B7));
            assert!(is_piece_on_square_as_expected(
                &pos,
                Square::B8,
                target,
                Colour::White
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
            let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
                fen::decompose_fen(fen);

            let zobrist_keys = ZobristKeys::new();
            let occ_masks = OccupancyMasks::new();
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            let mv = Move::encode_move_castle_kingside_white();

            let move_legality = pos.make_move(&mv);
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
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            let mv = Move::encode_move_castle_queenside_white();

            let move_legality = pos.make_move(&mv);
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
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            let mv = Move::encode_move_castle_kingside_black();

            let move_legality = pos.make_move(&mv);
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
            let attack_checker = AttackChecker::new();

            let mut pos = Position::new(
                board,
                castle_permissions,
                move_cntr,
                en_pass_sq,
                side_to_move,
                &zobrist_keys,
                &occ_masks,
                &attack_checker,
            );

            let mv = Move::encode_move_castle_queenside_black();

            let move_legality = pos.make_move(&mv);
            assert_eq!(move_legality, MoveLegality::Illegal);
        }
    }

    #[test]
    pub fn make_move_king_white_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_white_king_set());
        assert!(pos.castle_permissions().is_white_queen_set());

        let mv = Move::encode_move(&Square::E1, &Square::E2);

        let move_legality = pos.make_move(&mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!pos.castle_permissions().is_white_king_set());
        assert!(!pos.castle_permissions().is_white_queen_set());
    }

    #[test]
    pub fn make_move_king_white_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_white_king_set());
        assert!(pos.castle_permissions().is_white_queen_set());

        let mv = Move::encode_move(&Square::H1, &Square::G1);

        let move_legality = pos.make_move(&mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!pos.castle_permissions().is_white_king_set());
        assert!(pos.castle_permissions().is_white_queen_set());
    }

    #[test]
    pub fn make_move_white_queens_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_white_king_set());
        assert!(pos.castle_permissions().is_white_queen_set());

        let mv = Move::encode_move(&Square::A1, &Square::B1);

        let move_legality = pos.make_move(&mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(pos.castle_permissions().is_white_king_set());
        assert!(!pos.castle_permissions().is_white_queen_set());
    }

    #[test]
    pub fn make_move_king_black_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_black_king_set());
        assert!(pos.castle_permissions().is_black_queen_set());

        let mv = Move::encode_move(&Square::E8, &Square::E7);

        let move_legality = pos.make_move(&mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!pos.castle_permissions().is_black_king_set());
        assert!(!pos.castle_permissions().is_black_queen_set());
    }

    #[test]
    pub fn make_move_king_black_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_black_king_set());
        assert!(pos.castle_permissions().is_black_queen_set());

        let mv = Move::encode_move(&Square::H8, &Square::G8);

        let move_legality = pos.make_move(&mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(!pos.castle_permissions().is_black_king_set());
        assert!(pos.castle_permissions().is_black_queen_set());
    }

    #[test]
    pub fn make_move_black_queens_rook_moved_castle_permissions_cleared() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        assert!(pos.castle_permissions().is_black_king_set());
        assert!(pos.castle_permissions().is_black_queen_set());

        let mv = Move::encode_move(&Square::A8, &Square::B8);

        let move_legality = pos.make_move(&mv);
        assert_eq!(move_legality, MoveLegality::Legal);

        assert!(pos.castle_permissions().is_black_king_set());
        assert!(!pos.castle_permissions().is_black_queen_set());
    }

    #[test]
    pub fn make_move_take_move_position_and_board_restored_white_to_move() {
        let fen = "1b1kN3/Qp1P2p1/q2P1Nn1/PP3r2/3rPnb1/1p1pp3/B1P1P2B/R3K2R w KQ - 5 8";

        let ml = vec![
            Move::encode_move_castle_kingside_white(),
            Move::encode_move_castle_queenside_white(),
            Move::encode_move(&Square::E8, &Square::G7),
            Move::encode_move(&Square::B5, &Square::B6),
            Move::encode_move(&Square::C2, &Square::C4),
        ];

        let (board1, move_cntr1, castle_permissions1, side_to_move1, en_pass_sq1) =
            fen::decompose_fen(fen);

        let zobrist_keys1 = ZobristKeys::new();
        let occ_masks1 = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos1 = Position::new(
            board1,
            castle_permissions1,
            move_cntr1,
            en_pass_sq1,
            side_to_move1,
            &zobrist_keys1,
            &occ_masks1,
            &attack_checker,
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
            &attack_checker,
        );

        for mv in ml {
            println!("move: {}", mv);
            pos1.make_move(&mv);
            assert_ne!(pos1, pos2);

            pos1.take_move();

            assert_eq!(pos1, pos2);
        }
    }

    #[test]
    pub fn make_move_take_move_position_and_board_restored_black_to_move() {
        let fen = "r3k2r/1pb2p2/qQ1P2n1/PPPN2N1/4Pnb1/1p1pp3/B1P1P2B/R3K2R b kq - 3 11";

        let ml = vec![
            Move::encode_move_castle_kingside_black(),
            Move::encode_move_castle_queenside_black(),
            Move::encode_move(&Square::C7, &Square::B6),
            Move::encode_move(&Square::F7, &Square::F6),
            Move::encode_move(&Square::F7, &Square::F6),
        ];

        let (board1, move_cntr1, castle_permissions1, side_to_move1, en_pass_sq1) =
            fen::decompose_fen(fen);

        let zobrist_keys1 = ZobristKeys::new();
        let occ_masks1 = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let mut pos1 = Position::new(
            board1,
            castle_permissions1,
            move_cntr1,
            en_pass_sq1,
            side_to_move1,
            &zobrist_keys1,
            &occ_masks1,
            &attack_checker,
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
            &attack_checker,
        );

        // initial states are the same
        assert_eq!(pos1, pos2);

        for mv in ml {
            println!("board pre-move : {}", pos1.board());
            println!("making move : {}", mv);

            pos1.make_move(&mv);
            assert_ne!(pos1, pos2);
            println!("board post-move : {}", pos1.board());

            pos1.take_move();
            println!("board after take-move : {}", pos1.board());

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&Piece::Pawn, &Colour::White, &Square::B2);
        expected_hash ^= zobrist_keys.piece_square(&Piece::Pawn, &Colour::White, &Square::B4);
        expected_hash ^= zobrist_keys.en_passant(&Square::B3);
        expected_hash ^= zobrist_keys.side();

        let wp_double_mv = Move::encode_move(&Square::B2, &Square::B4);
        pos.make_move(&wp_double_mv);

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&Piece::Pawn, &Colour::Black, &Square::B7);
        expected_hash ^= zobrist_keys.piece_square(&Piece::Pawn, &Colour::Black, &Square::B5);
        expected_hash ^= zobrist_keys.en_passant(&Square::B6);
        expected_hash ^= zobrist_keys.side();

        let bp_double_mv = Move::encode_move(&Square::B7, &Square::B5);
        pos.make_move(&bp_double_mv);

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&Piece::Knight, &Colour::White, &Square::F2);
        expected_hash ^= zobrist_keys.piece_square(&Piece::Knight, &Colour::White, &Square::G4);
        expected_hash ^= zobrist_keys.side();

        let wp_double_mv = Move::encode_move(&Square::F2, &Square::G4);
        pos.make_move(&wp_double_mv);

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let init_hash = pos.position_hash();

        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&Piece::Knight, &Colour::Black, &Square::F6);
        expected_hash ^= zobrist_keys.piece_square(&Piece::Knight, &Colour::Black, &Square::D7);
        expected_hash ^= zobrist_keys.side();

        let wp_double_mv = Move::encode_move(&Square::D7, &Square::F6);
        pos.make_move(&wp_double_mv);

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let init_hash = pos.position_hash();

        // remove white pawn on b4
        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&Piece::Pawn, &Colour::White, &Square::B4);
        // move a4->b3
        expected_hash ^= zobrist_keys.piece_square(&Piece::Pawn, &Colour::Black, &Square::A4);
        expected_hash ^= zobrist_keys.piece_square(&Piece::Pawn, &Colour::Black, &Square::B3);
        expected_hash ^= zobrist_keys.en_passant(&Square::B3);
        expected_hash ^= zobrist_keys.side();

        assert_eq!(pos.en_passant_square(), Some(Square::B3));
        let mv = Move::encode_move_en_passant(&Square::A4, &Square::B3);
        pos.make_move(&mv);

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
        let attack_checker = AttackChecker::new();

        let mut pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let init_hash = pos.position_hash();

        // remove black pawn
        let mut expected_hash =
            init_hash ^ zobrist_keys.piece_square(&Piece::Pawn, &Colour::Black, &Square::D5);
        // move e5->d6
        expected_hash ^= zobrist_keys.piece_square(&Piece::Pawn, &Colour::White, &Square::E5);
        expected_hash ^= zobrist_keys.piece_square(&Piece::Pawn, &Colour::White, &Square::D6);
        expected_hash ^= zobrist_keys.en_passant(&Square::D6);
        expected_hash ^= zobrist_keys.side();

        assert_eq!(pos.en_passant_square(), Some(Square::D6));
        let mv = Move::encode_move_en_passant(&Square::E5, &Square::D6);
        pos.make_move(&mv);

        assert!(init_hash != pos.position_hash());
        assert!(expected_hash == pos.position_hash());
    }

    fn is_piece_on_square_as_expected(pos: &Position, sq: Square, pce: Piece, col: Colour) -> bool {
        if let Some((piece, colour)) = pos.board.get_piece_and_colour_on_square(&sq) {
            if piece != pce {
                return false;
            }

            if col != colour {
                return false;
            }

            return true;
        }
        false
    }

    fn is_sq_empty(pos: &Position, sq: Square) -> bool {
        let pce = pos.board.get_piece_and_colour_on_square(&sq);
        if pce.is_some() {
            return false;
        }

        true
    }
}
