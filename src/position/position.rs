use board::board::Board;
use board::piece::Colour;
use board::square::Square;
use position::castle_permissions::CastlePermission;

struct MoveCounter {
    half_move: u16,
    full_move: u16,
}

pub struct Position {
    // pieces and squares
    pub board: Board,
    // side to move
    side_to_move: Colour,
    // the en passant square
    en_pass_sq: Option<Square>,
    // castle permissions
    castle_perm: CastlePermission,

    move_cntr: MoveCounter,
}

impl Position {
    pub fn set_board(&mut self, brd: Board) {
        self.board = brd;
    }
    pub fn get_side_to_move(&self) -> Colour {
        self.side_to_move
    }
}

#[cfg(test)]
mod tests {}
