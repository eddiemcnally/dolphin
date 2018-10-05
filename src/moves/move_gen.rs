use board::bitboard::BitBoard;
use board::bitboard::BitManipulation;
use board::occupancy_masks::get_occupancy_mask;
use board::piece;
use moves::mv::Move;
use position::Position;
use std::vec::Vec;

pub fn generate_all_moves(pos: Position) -> Vec<Move> {
    let side_to_move = pos.get_side_to_move();
    let mut move_list: Vec<Move> = Vec::new();

    return move_list;
}

fn generate_knight_moves(pos: Position, piece: piece::Piece, move_list: &mut Vec<Move>) {
    let mut bb: BitBoard = pos.board.get_bitboard(piece);

    while bb != 0 {
        let sq = bb.pop_1st_bit();

        let occmask = get_occupancy_mask(piece, sq);

        // get occupancy mask for knight at this square
        //        uint64_t mask = get_knight_occ_mask(knight_sq);

        // AND'ing with opposite colour pieces, will give all
        // pieces that can be captured
        //      uint64_t opp_pieces = get_bitboard_for_colour(bb, opposite_col);
        //       uint64_t capture_squares = mask & opp_pieces;
        //
    }
}
