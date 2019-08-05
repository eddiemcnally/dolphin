use board::bitboard;
use board::board::Board;
use board::occupancy_masks;
use board::piece::Piece;
use board::piece::PieceRole;
use board::square::Square;
use moves::mov::Mov;

fn generate_non_sliding_piece_moves(board: Board, pce: Piece, move_list: &mut Vec<Mov>) {
    let mut pce_bb = board.get_piece_bitboard(pce);

    while pce_bb != 0 {
        let sq = bitboard::pop_1st_bit(&mut pce_bb);

        let occ_mask = match pce.role() {
            PieceRole::King => occupancy_masks::get_occupancy_mask_king(sq),
            PieceRole::Knight => occupancy_masks::get_occupancy_mask_knight(sq),
            _ => panic!("Invalid piece role"),
        };

        // generate capture moves
        // ----------------------
        // AND'ing with opposite colour pieces with the occupancy mask, will
        // give all pieces that can be captured by the piece on this square
        let opposite_side = pce.colour().flip_side();
        let opp_occ_sq_bb = board.get_colour_bb(opposite_side);
        let mut capt_bb = opp_occ_sq_bb & occ_mask;
        encode_multiple_capture_moves(&mut capt_bb, sq, move_list);

        // generate quiet moves
        let unoccupied_squares_bb = !board.get_bitboard();
        let mut quiet_move_bb = unoccupied_squares_bb & occ_mask;
        encode_multiple_quiet_moves(&mut quiet_move_bb, sq, move_list);
    }
}

fn encode_multiple_capture_moves(capt_bb: &mut u64, from_sq: Square, move_list: &mut Vec<Mov>) {
    while *capt_bb != 0 {
        let to_sq = bitboard::pop_1st_bit(capt_bb);
        let mov = Mov::encode_move_capture(from_sq, to_sq);
        move_list.push(mov);
    }
}

fn encode_multiple_quiet_moves(quiet_move_bb: &mut u64, from_sq: Square, move_list: &mut Vec<Mov>) {
    while *quiet_move_bb != 0 {
        let to_sq = bitboard::pop_1st_bit(quiet_move_bb);
        let mov = Mov::encode_quiet(from_sq, to_sq);
        move_list.push(mov);
    }
}
