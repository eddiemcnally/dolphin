use board::bitboard;
use board::board::Board;
use board::occupancy_masks;
use board::piece::Piece;
use board::piece::Colour;
use board::piece::PieceRole;
use board::square::Square;
use moves::mov::Mov;



pub fn generate_moves(board: &Board, side_to_move: Colour, move_list: &mut Vec<Mov>){

    let knight = Piece::new(PieceRole::Knight, side_to_move);
    generate_non_sliding_piece_moves(board, knight, move_list);
 
    let king = Piece::new(PieceRole::King, side_to_move);
    generate_non_sliding_piece_moves(board, king, move_list);

    let bishop = Piece::new(PieceRole::Bishop, side_to_move);
    generate_sliding_diagonal_antidiagonal_moves(board, bishop, move_list);

}


// generates diagonal and anti-diagonal moves for queen and bishop
// see Hyperbola Quintessence
fn generate_sliding_diagonal_antidiagonal_moves(board:&Board, pce: Piece, move_list:&mut Vec<Mov>){
    let mut pce_bb = board.get_piece_bitboard(pce);
    let occ_sq_bb = board.get_bitboard();
    let occ_col_bb = board.get_colour_bb(pce.colour());

    while pce_bb != 0{

        let from_sq = bitboard::pop_1st_bit(&mut pce_bb);
        let diag_move_mask = occupancy_masks::get_diagonal_move_mask(from_sq);
        let anti_diag_move_mask = occupancy_masks::get_anti_diagonal_move_mask(from_sq);
        
        let mut slider_bb: u64 = 0;
        bitboard::set_bit(&mut slider_bb, from_sq);

        // diagonal moves
        let diag1 = (occ_sq_bb & diag_move_mask).overflowing_sub(2 * slider_bb).0;
        let diag2 = ((occ_sq_bb & diag_move_mask)
                                .reverse_bits()
                                .overflowing_sub(2 * slider_bb.reverse_bits()))
                                .0
                                .reverse_bits();
        
        let diag = diag1 ^ diag2;

        // anti-diagonal moves
        let antidiag1 = (occ_sq_bb & anti_diag_move_mask).overflowing_sub(2 * slider_bb).0;
        let antidiag2 = ((occ_sq_bb & anti_diag_move_mask)
                            .reverse_bits().overflowing_sub(2 * slider_bb.reverse_bits()))
                            .0
                            .reverse_bits();

        let antidiag = antidiag1 ^ antidiag2;

        let all_moves = (diag & diag_move_mask) | (antidiag & anti_diag_move_mask);
    
        let mut excl_same_colour = all_moves & !occ_col_bb;

        while excl_same_colour != 0{
            let to_sq = bitboard::pop_1st_bit(&mut excl_same_colour);

            if board.is_sq_empty(to_sq){
                let mv = Mov::encode_move_quiet(from_sq, to_sq);
                move_list.push(mv);
            } else{
                let mv = Mov::encode_move_capture(from_sq, to_sq);
                move_list.push(mv);
            }
        }   
    }





}






// generates moves for King and Knight
fn generate_non_sliding_piece_moves(board: &Board, pce: Piece, move_list: &mut Vec<Mov>) {
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
        let mov = Mov::encode_move_quiet(from_sq, to_sq);
        move_list.push(mov);
    }
}





#[cfg(test)]
pub mod tests {
    //use board::piece::Piece;
    use board::piece::Colour;
    //use board::piece::PieceRole;
    use board::square::Square;
    //use input::fen::ParsedFen;
    use input::fen;
    use board::board::Board;
    use moves::move_gen;
    use moves::mov::Mov;
    use moves::mov;


    #[test]
    pub fn test_move_gen_white_king_knight_move_list_as_expected(){

        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let mut move_list:Vec<Mov> = Vec::new();
        let parsed_fen = fen::get_position(&fen);
        let brd = Board::from_fen(&parsed_fen);

        move_gen::generate_moves(&brd, Colour::White, &mut move_list);
       
        // check the capture moves
        let mut mv = Mov::encode_move_capture(Square::e3, Square::d1);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_capture(Square::e3, Square::c2);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_capture(Square::a6, Square::b8);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_capture(Square::a6, Square::c7);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_capture(Square::g5, Square::h6);
        assert!(move_list.contains(&mv) == true);

        // check the quiet moves
        mv = Mov::encode_move_quiet(Square::a6, Square::c5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::f1);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::g2);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::g4);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::f5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::d5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::g6);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::f6);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::f5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::g4);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::h4);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::h5);
        assert!(move_list.contains(&mv) == true);

    }



    #[test]
    pub fn test_move_gen_black_king_knight_move_list_as_expected(){

        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let mut move_list:Vec<Mov> = Vec::new();
        let parsed_fen = fen::get_position(&fen);
        let brd = Board::from_fen(&parsed_fen);

        move_gen::generate_moves(&brd, Colour::Black, &mut move_list);

        // check the capture moves
        let mut mv = Mov::encode_move_capture(Square::h1, Square::f2);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_capture(Square::d8, Square::e7);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_capture(Square::b8, Square::a6);
        assert!(move_list.contains(&mv) == true);

        // check the quiet moves
        mv = Mov::encode_move_quiet(Square::d8, Square::c8);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::d8, Square::e8);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::h1, Square::g3);
        assert!(move_list.contains(&mv) == true);
    }



    #[test]
    pub fn test_move_gen_white_bishop_move_list_as_expected(){

        let fen = "1n1k2bp/1PppQpb1/N1p4p/4P1K1/1RB1BP2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let mut move_list:Vec<Mov> = Vec::new();
        let parsed_fen = fen::get_position(&fen);
        let brd = Board::from_fen(&parsed_fen);

        move_gen::generate_moves(&brd, Colour::White, &mut move_list);

        mov::print_move_list(&move_list);

        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::c4, Square::b5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::c4, Square::d5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::c4, Square::e6);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::c4, Square::d3);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e4, Square::d5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e4, Square::d3);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e4, Square::f5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e4, Square::g6);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e4, Square::h7);
        assert!(move_list.contains(&mv) == true);




        // check the quiet moves
        mv = Mov::encode_move_quiet(Square::e4, Square::d3);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::f1);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::g2);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::g4);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::f5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::e3, Square::d5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::g6);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::f6);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::f5);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::g4);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::h4);
        assert!(move_list.contains(&mv) == true);

        mv = Mov::encode_move_quiet(Square::g5, Square::h5);
        assert!(move_list.contains(&mv) == true);

    }



}



