use board::board::Board;
use board::piece::Colour;
use board::square::Square;
use input::fen::ParsedFen;
use position::castle_permissions::CastlePermission;

pub struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

pub struct Position {
    // pieces and squares
    board: Board,
    // side to move
    side_to_move: Colour,
    // the en passant square
    en_pass_sq: Option<Square>,
    // castle permissions
    castle_perm: CastlePermission,

    move_cntr: MoveCounter,
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
        }
    }

    pub fn side_to_move(&self) -> Colour {
        self.side_to_move
    }

    pub fn board(&self) -> &Board{
        &self.board
    }

    pub fn en_passant_square(&self) -> Option<Square>{
        self.en_pass_sq
    }

    pub fn castle_permissions(&self) -> CastlePermission{
        self.castle_perm
    }

    pub fn move_counter(&self) -> &MoveCounter{
        &self.move_cntr
    }

    pub fn flip_side_to_move(&mut self) {
        match self.side_to_move {
            Colour::White => self.side_to_move = Colour::Black,
            Colour::Black => self.side_to_move = Colour::White,            
        }
    }
}

#[cfg(test)]
mod tests {}
