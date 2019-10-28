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

        self.position_key.update_side();

        let from_sq = mv.decode_from_square();
        let to_sq = mv.decode_to_square();
        let piece = self.board().get_piece_on_square(from_sq).unwrap();
        let side_to_move = self.side_to_move;

        if mv.is_quiet() {
            self.position_key.update_piece(piece, from_sq);
            self.position_key.update_piece(piece, to_sq);
            self.board.move_piece(from_sq, to_sq, piece);
        } else if mv.is_double_pawn() {
            self.position_key.update_piece(piece, from_sq);
            self.position_key.update_piece(piece, to_sq);
            self.board.move_piece(from_sq, to_sq, piece);

            let s = find_en_passant_sq(from_sq, side_to_move);
            self.en_pass_sq = Some(s);
        } else if mv.is_king_castle() {
            do_castle_move_king(self, side_to_move);
        } else if mv.is_queen_castle() {
            do_castle_move_queen(self, side_to_move);
        }

        return true;
    }
}

fn find_en_passant_sq(from_sq: Square, col: Colour) -> Square {
    match col {
        Colour::White => from_sq.square_plus_1_rank(),
        Colour::Black => from_sq.square_minus_1_rank(),
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
    position.position_key.update_piece(king, king_from_sq);
    position.position_key.update_piece(king, king_to_sq);
    position.board.move_piece(king_from_sq, king_to_sq, king);
    let rook = Piece::new(PieceRole::Rook, col);
    position.position_key.update_piece(rook, rook_from_sq);
    position.position_key.update_piece(rook, rook_to_sq);
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
    position.position_key.update_piece(queen, queen_from_sq);
    position.position_key.update_piece(queen, queen_to_sq);
    position.board.move_piece(queen_from_sq, queen_to_sq, queen);
    let rook = Piece::new(PieceRole::Rook, col);
    position.position_key.update_piece(rook, rook_from_sq);
    position.position_key.update_piece(rook, rook_to_sq);
    position.board.move_piece(rook_from_sq, rook_to_sq, rook);

    position.castle_perm.set_queen(col, false);
}

#[cfg(test)]
mod tests {}
