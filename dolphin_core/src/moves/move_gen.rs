use crate::board::bitboard;
use crate::board::colour::Colour;
use crate::board::occupancy_masks;
use crate::board::piece;
use crate::board::piece::Piece;
use crate::board::square::Square;
use crate::moves::mov::Mov;
use crate::moves::move_list::MoveList;
use crate::position::castle_permissions;
use crate::position::game_position::Position;

pub struct MoveGenerator {}

const WHITE_PROMO_PCES: [&Piece; 4] = [
    &piece::WHITE_KNIGHT,
    &piece::WHITE_BISHOP,
    &piece::WHITE_ROOK,
    &piece::WHITE_QUEEN,
];

const BLACK_PROMO_PCES: [&Piece; 4] = [
    &piece::BLACK_KNIGHT,
    &piece::BLACK_BISHOP,
    &piece::BLACK_ROOK,
    &piece::BLACK_QUEEN,
];

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator {}
    }

    pub fn generate_moves(&self, position: &Position, move_list: &mut MoveList) {
        if position.side_to_move() == Colour::White {
            let rank_file_pce_bb = position.board().get_white_rook_queen_bitboard();
            self.generate_sliding_rank_file_moves(position, move_list, rank_file_pce_bb);

            let sliding_pce_bb = position.board().get_white_bishop_queen_bitboard();
            self.generate_sliding_diagonal_antidiagonal_moves(position, move_list, sliding_pce_bb);

            self.generate_white_pawn_moves(position, move_list);

            self.generate_knight_moves(position, move_list, &piece::WHITE_KNIGHT);

            self.generate_king_moves(position, move_list, &piece::WHITE_KING);

            if castle_permissions::has_white_castle_permission(position.castle_permissions()) {
                self.generate_white_castle_moves(position, move_list);
            }
        } else {
            let rank_file_pce_bb = position.board().get_black_rook_queen_bitboard();
            self.generate_sliding_rank_file_moves(position, move_list, rank_file_pce_bb);

            let sliding_pce_bb = position.board().get_black_bishop_queen_bitboard();
            self.generate_sliding_diagonal_antidiagonal_moves(position, move_list, sliding_pce_bb);

            self.generate_black_pawn_moves(position, move_list);

            self.generate_knight_moves(position, move_list, &piece::BLACK_KNIGHT);

            self.generate_king_moves(position, move_list, &piece::BLACK_KING);

            if castle_permissions::has_black_castle_permission(position.castle_permissions()) {
                self.generate_black_castle_moves(position, move_list);
            }
        }
    }

    fn generate_white_pawn_moves(&self, pos: &Position, move_list: &mut MoveList) {
        // bitboard of entire board
        let all_bb = pos.board().get_bitboard();
        // opposite colour bitboard
        let all_opposing_bb = pos.board().get_colour_bb(Colour::Black);

        let wp_bb = pos.board().get_piece_bitboard(&piece::WHITE_PAWN);

        // =================================================
        // Pawn quiet + capture, exclude possible promotions
        // =================================================
        let mut excl_pawn_promo_bb = wp_bb & !occupancy_masks::RANK_7_BB;
        while excl_pawn_promo_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut excl_pawn_promo_bb);

            // quiet move
            let quiet_to_sq = from_sq.square_plus_1_rank();
            let capt_mask = pos
                .occupancy_masks()
                .get_occ_mask_white_pawn_attack_squares(from_sq);

            if !bitboard::is_set(all_bb, quiet_to_sq.unwrap()) {
                let mv = Mov::encode_move_quiet(from_sq, quiet_to_sq.unwrap());
                move_list.push(mv);
            }

            // capture moves
            let mut capt_bb = capt_mask & all_opposing_bb;
            while capt_bb != 0 {
                let to_sq = bitboard::pop_1st_bit(&mut capt_bb);
                let mv = Mov::encode_move_capture(from_sq, to_sq);
                move_list.push(mv);
            }

            // en passant move
            if let Some(en_sq) = pos.en_passant_square() {
                if bitboard::is_set(capt_mask, en_sq) {
                    // en passant sq can be "captured"
                    let en_pass_mv = Mov::encode_move_en_passant(from_sq, en_sq);
                    move_list.push(en_pass_mv);
                }
            }
        }

        // ================
        // double pawn move
        // ================
        let mut starting_pawn_bb = wp_bb & occupancy_masks::RANK_2_BB;
        while starting_pawn_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut starting_pawn_bb);

            let double_first_move_sq_mask = pos
                .occupancy_masks()
                .get_occ_mask_white_pawns_double_move_mask(from_sq);

            if all_bb & double_first_move_sq_mask == 0 {
                // both squares free
                let to_sq = from_sq.square_plus_2_ranks().unwrap();

                let mv = Mov::encode_move_double_pawn_first(from_sq, to_sq);
                move_list.push(mv);
            }
        }

        // =========
        // promotion
        // =========
        let mut promo_bb = wp_bb & occupancy_masks::RANK_7_BB;
        while promo_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut promo_bb);

            // quiet promotion + capture promotion
            let quiet_to_sq = from_sq.square_plus_1_rank();
            let capt_mask = pos
                .occupancy_masks()
                .get_occ_mask_white_pawn_attack_squares(from_sq);

            if !bitboard::is_set(all_bb, quiet_to_sq.unwrap()) {
                // free square ahead
                self.encode_promotion_moves(
                    pos.side_to_move(),
                    from_sq,
                    quiet_to_sq.unwrap(),
                    move_list,
                );
            }

            let mut capt_bb = capt_mask & all_opposing_bb;

            while capt_bb != 0 {
                let to_sq = bitboard::pop_1st_bit(&mut capt_bb);
                self.encode_promotion_capture_moves(pos.side_to_move(), from_sq, to_sq, move_list);
            }
        }
    }

    fn generate_black_pawn_moves(&self, pos: &Position, move_list: &mut MoveList) {
        // bitboard of entire board
        let all_bb = pos.board().get_bitboard();
        // opposite colour bitboard
        let all_opposing_bb = pos.board().get_colour_bb(Colour::White);

        let bp_bb = pos.board().get_piece_bitboard(&piece::BLACK_PAWN);

        // =================================================
        // Pawn quiet + capture, exclude possible promotions
        // =================================================
        let mut excl_pawn_promo_bb = bp_bb & !occupancy_masks::RANK_2_BB;
        while excl_pawn_promo_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut excl_pawn_promo_bb);

            // quiet moves + capture move
            let quiet_to_sq = from_sq.square_minus_1_rank();
            let capt_mask = pos
                .occupancy_masks()
                .get_occ_mask_black_pawn_attack_squares(from_sq);

            if !bitboard::is_set(all_bb, quiet_to_sq.unwrap()) {
                let mv = Mov::encode_move_quiet(from_sq, quiet_to_sq.unwrap());
                move_list.push(mv);
            }

            let mut capt_bb = capt_mask & all_opposing_bb;
            while capt_bb != 0 {
                let to_sq = bitboard::pop_1st_bit(&mut capt_bb);
                let mv = Mov::encode_move_capture(from_sq, to_sq);
                move_list.push(mv);
            }

            // en passant move
            if let Some(en_sq) = pos.en_passant_square() {
                if bitboard::is_set(capt_mask, en_sq) {
                    // en passant sq can be "captured"
                    let en_pass_mv = Mov::encode_move_en_passant(from_sq, en_sq);
                    move_list.push(en_pass_mv);
                }
            }
        }

        // ================
        // double pawn move
        // ================
        let mut starting_pawn_bb = bp_bb & occupancy_masks::RANK_7_BB;
        while starting_pawn_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut starting_pawn_bb);

            let double_first_move_sq_mask = pos
                .occupancy_masks()
                .get_occ_mask_black_pawns_double_move_mask(from_sq);

            if all_bb & double_first_move_sq_mask == 0 {
                // both squares free
                let to_sq = from_sq.square_minus_2_ranks().unwrap();

                let mv = Mov::encode_move_double_pawn_first(from_sq, to_sq);
                move_list.push(mv);
            }
        }

        // =========
        // promotion
        // =========
        let mut promo_bb = bp_bb & occupancy_masks::RANK_2_BB;
        while promo_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut promo_bb);

            // quiet promotion + capture promotion
            let quiet_to_sq = from_sq.square_minus_1_rank();
            let capt_mask = pos
                .occupancy_masks()
                .get_occ_mask_black_pawn_attack_squares(from_sq);

            if !bitboard::is_set(all_bb, quiet_to_sq.unwrap()) {
                // free square ahead
                self.encode_promotion_moves(
                    pos.side_to_move(),
                    from_sq,
                    quiet_to_sq.unwrap(),
                    move_list,
                );
            }

            let mut capt_bb = capt_mask & all_opposing_bb;

            while capt_bb != 0 {
                let to_sq = bitboard::pop_1st_bit(&mut capt_bb);
                self.encode_promotion_capture_moves(pos.side_to_move(), from_sq, to_sq, move_list);
            }
        }
    }

    // generates diagonal and anti-diagonal moves for queen and bishop
    // see Hyperbola Quintessence
    fn generate_sliding_diagonal_antidiagonal_moves(
        &self,
        pos: &Position,
        move_list: &mut MoveList,
        sliding_pce_bb: u64,
    ) {
        let mut pce_bb = sliding_pce_bb;

        let occ_sq_bb = pos.board().get_bitboard();
        let occ_col_bb = pos.board().get_colour_bb(pos.side_to_move());

        while pce_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut pce_bb);

            let diagonal_masks = pos.occupancy_masks().get_diag_antidiag_mask(from_sq);
            let slider_bb = bitboard::to_mask(from_sq);

            // diagonal moves
            let diag1 = (occ_sq_bb & diagonal_masks.get_diag_mask())
                .overflowing_sub(slider_bb.overflowing_mul(2).0)
                .0;
            let diag2 = ((occ_sq_bb & diagonal_masks.get_diag_mask())
                .reverse_bits()
                .overflowing_sub(slider_bb.reverse_bits().overflowing_mul(2).0))
            .0
            .reverse_bits();
            let diag = diag1 ^ diag2;

            // anti-diagonal moves
            let antidiag1 = (occ_sq_bb & diagonal_masks.get_anti_diag_mask())
                .overflowing_sub(slider_bb.overflowing_mul(2).0)
                .0;
            let antidiag2 = ((occ_sq_bb & diagonal_masks.get_anti_diag_mask())
                .reverse_bits()
                .overflowing_sub(slider_bb.reverse_bits().overflowing_mul(2).0))
            .0
            .reverse_bits();

            let antidiag = antidiag1 ^ antidiag2;

            let all_moves = (diag & diagonal_masks.get_diag_mask())
                | (antidiag & diagonal_masks.get_anti_diag_mask());
            let mut excl_same_colour = all_moves & !occ_col_bb;

            while excl_same_colour != 0 {
                let to_sq = bitboard::pop_1st_bit(&mut excl_same_colour);
                let mv = if pos.board().is_sq_empty(to_sq) {
                    Mov::encode_move_quiet(from_sq, to_sq)
                } else {
                    Mov::encode_move_capture(from_sq, to_sq)
                };

                move_list.push(mv);
            }
        }
    }

    // generates sliding rank and file moves for queen and rook
    // see Hyperbola Quintessence
    fn generate_sliding_rank_file_moves(
        &self,
        pos: &Position,
        move_list: &mut MoveList,
        rank_file_pce_bb: u64,
    ) {
        let mut pce_bb = rank_file_pce_bb;

        let occ_sq_bb = pos.board().get_bitboard();
        let occ_col_bb = pos.board().get_colour_bb(pos.side_to_move());

        while pce_bb != 0 {
            let from_sq = bitboard::pop_1st_bit(&mut pce_bb);
            let horiz_mask = pos.occupancy_masks().get_horizontal_mask(from_sq);
            let vertical_mask = pos.occupancy_masks().get_vertical_mask(from_sq);

            let slider_bb = bitboard::to_mask(from_sq);
            let slider_bb_reverse = slider_bb.reverse_bits();

            // horizontal moves
            let horiz1 = occ_sq_bb.overflowing_sub(slider_bb.overflowing_mul(2).0).0;
            let horiz2 = (occ_sq_bb
                .reverse_bits()
                .overflowing_sub(slider_bb_reverse.overflowing_mul(2).0)
                .0)
                .reverse_bits();
            let horiz = horiz1 ^ horiz2;

            // vertical moves
            let vert1 = (occ_sq_bb & vertical_mask)
                .overflowing_sub(slider_bb.overflowing_mul(2).0)
                .0;
            let vert2 = ((occ_sq_bb & vertical_mask)
                .reverse_bits()
                .overflowing_sub(slider_bb_reverse.overflowing_mul(2).0))
            .0
            .reverse_bits();
            let vert = vert1 ^ vert2;

            let all_moves_mask = (horiz & horiz_mask) | (vert & vertical_mask);

            let mut all_excl_same_col = all_moves_mask & !occ_col_bb;

            while all_excl_same_col != 0 {
                let to_sq = bitboard::pop_1st_bit(&mut all_excl_same_col);
                let mv = if pos.board().is_sq_empty(to_sq) {
                    Mov::encode_move_quiet(from_sq, to_sq)
                } else {
                    Mov::encode_move_capture(from_sq, to_sq)
                };

                move_list.push(mv);
            }
        }
    }

    fn generate_knight_moves(
        &self,
        pos: &Position,
        move_list: &mut MoveList,
        knight: &'static Piece,
    ) {
        let mut pce_bb = pos.board().get_piece_bitboard(knight);
        if pce_bb == 0 {
            return;
        }

        let opposite_side = pos.side_to_move().flip_side();
        let opp_occ_sq_bb = pos.board().get_colour_bb(opposite_side);

        while pce_bb != 0 {
            let sq = bitboard::pop_1st_bit(&mut pce_bb);
            let occ_mask = pos.occupancy_masks().get_occupancy_mask_knight(sq);

            // generate capture moves
            // AND'ing with opposite colour pieces with the occupancy mask, will
            // give all pieces that can be captured by the piece on this square
            let capt_bb = opp_occ_sq_bb & occ_mask;
            self.encode_multiple_capture_moves(capt_bb, sq, move_list);

            // generate quiet moves
            let unoccupied_squares_bb = !pos.board().get_bitboard();
            let quiet_move_bb = unoccupied_squares_bb & occ_mask;
            self.encode_multiple_quiet_moves(quiet_move_bb, sq, move_list);
        }
    }

    fn generate_king_moves(&self, pos: &Position, move_list: &mut MoveList, king: &'static Piece) {
        let opposite_side = pos.side_to_move().flip_side();
        let opp_occ_sq_bb = pos.board().get_colour_bb(opposite_side);

        let mut pce_bb = pos.board().get_piece_bitboard(king);
        let sq = bitboard::pop_1st_bit(&mut pce_bb);
        let occ_mask = pos.occupancy_masks().get_occupancy_mask_king(sq);

        // generate capture moves
        // ----------------------
        // AND'ing with opposite colour pieces with the occupancy mask, will
        // give all pieces that can be captured by the piece on this square
        let capt_bb = opp_occ_sq_bb & occ_mask;
        self.encode_multiple_capture_moves(capt_bb, sq, move_list);

        // generate quiet moves
        let unoccupied_squares_bb = !pos.board().get_bitboard();
        let quiet_move_bb = unoccupied_squares_bb & occ_mask;
        self.encode_multiple_quiet_moves(quiet_move_bb, sq, move_list);
    }

    fn generate_white_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if castle_permissions::is_white_king_set(cp) && (bb & occupancy_masks::CASTLE_MASK_WK == 0)
        {
            let mv = Mov::encode_move_castle_kingside_white();
            move_list.push(mv);
        }
        if castle_permissions::is_white_queen_set(cp) && (bb & occupancy_masks::CASTLE_MASK_WQ == 0)
        {
            let mv = Mov::encode_move_castle_queenside_white();
            move_list.push(mv);
        }
    }

    fn generate_black_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if castle_permissions::is_black_king_set(cp) && (bb & occupancy_masks::CASTLE_MASK_BK == 0)
        {
            let mv = Mov::encode_move_castle_kingside_black();
            move_list.push(mv);
        }
        if castle_permissions::is_black_queen_set(cp) && (bb & occupancy_masks::CASTLE_MASK_BQ == 0)
        {
            let mv = Mov::encode_move_castle_queenside_black();
            move_list.push(mv);
        }
    }

    fn encode_promotion_moves(
        &self,
        side_to_move: Colour,
        from_sq: Square,
        to_sq: Square,
        move_list: &mut MoveList,
    ) {
        match side_to_move {
            Colour::White => {
                for pce in WHITE_PROMO_PCES {
                    move_list.push(Mov::encode_move_with_promotion(from_sq, to_sq, pce));
                }
            }
            Colour::Black => {
                for pce in BLACK_PROMO_PCES {
                    move_list.push(Mov::encode_move_with_promotion(from_sq, to_sq, pce));
                }
            }
        };
    }

    fn encode_promotion_capture_moves(
        &self,
        side_to_move: Colour,
        from_sq: Square,
        to_sq: Square,
        move_list: &mut MoveList,
    ) {
        match side_to_move {
            Colour::White => {
                for pce in WHITE_PROMO_PCES {
                    move_list.push(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce));
                }
            }
            Colour::Black => {
                for pce in BLACK_PROMO_PCES {
                    move_list.push(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce));
                }
            }
        };
    }

    fn encode_multiple_capture_moves(
        &self,
        capt_bb: u64,
        from_sq: Square,
        move_list: &mut MoveList,
    ) {
        let mut bb = capt_bb;
        while bb != 0 {
            let to_sq = bitboard::pop_1st_bit(&mut bb);
            let mov = Mov::encode_move_capture(from_sq, to_sq);
            move_list.push(mov);
        }
    }

    fn encode_multiple_quiet_moves(
        &self,
        quiet_move_bb: u64,
        from_sq: Square,
        move_list: &mut MoveList,
    ) {
        let mut bb = quiet_move_bb;
        while bb != 0 {
            let to_sq = bitboard::pop_1st_bit(&mut bb);
            let mov = Mov::encode_move_quiet(from_sq, to_sq);
            move_list.push(mov);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::board::piece;
    use crate::board::piece::Piece;
    use crate::board::square::Square;
    use crate::io::fen;
    use crate::moves::mov;
    use crate::moves::mov::Mov;
    use crate::moves::move_gen::MoveGenerator;
    use crate::moves::move_list::MoveList;
    use crate::position::castle_permissions;
    use crate::position::game_position::Position;
    use crate::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn move_gen_white_king_knight_move_list_as_expected() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);
        // check the capture moves
        let mut mv = Mov::encode_move_capture(Square::e3, Square::d1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e3, Square::c2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::a6, Square::b8);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::a6, Square::c7);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::g5, Square::h6);
        assert!(move_list.contains(mv) == true);

        // check the quiet moves
        mv = Mov::encode_move_quiet(Square::a6, Square::c5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e3, Square::f1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e3, Square::g2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e3, Square::g4);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e3, Square::f5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e3, Square::d5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g5, Square::g6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g5, Square::f6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g5, Square::f5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g5, Square::g4);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g5, Square::h4);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g5, Square::h5);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_black_king_knight_move_list_as_expected() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/P2q3n b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the capture moves
        let mut mv = Mov::encode_move_capture(Square::h1, Square::f2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::d8, Square::e7);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::b8, Square::a6);
        assert!(move_list.contains(mv) == true);

        // check the quiet moves
        mv = Mov::encode_move_quiet(Square::d8, Square::c8);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::d8, Square::e8);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::h1, Square::g3);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_white_bishop_move_list_as_expected() {
        let fen = "1n1k2bp/1PppQpb1/N1p4p/4P1K1/1RB1BP2/pPR1Np2/P1r1rP1P/P2q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);

        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::c4, Square::b5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c4, Square::d5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c4, Square::e6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c4, Square::d3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e4, Square::d5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e4, Square::d3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e4, Square::f5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e4, Square::g6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e4, Square::h7);
        assert!(move_list.contains(mv) == true);

        // check the capture moves
        mv = Mov::encode_move_capture(Square::e4, Square::c2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e4, Square::f3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e4, Square::c6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::c4, Square::e2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::c4, Square::f7);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_black_bishop_move_list_as_expected() {
        let fen = "1nbk3p/NP1pQpP1/2p4p/p5K1/1RBbBP2/pPR1Np2/P1r1rP1P/P2q3n b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);
        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::d4, Square::c5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::d4, Square::b6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::d4, Square::e5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::d4, Square::f6);
        assert!(move_list.contains(mv) == true);

        // check the capture moves
        mv = Mov::encode_move_capture(Square::c8, Square::b7);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::d4, Square::c3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::d4, Square::e3);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_black_rook_move_list_as_expected() {
        let fen = "1nbk3p/NP1pQpP1/2p4p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/Pr4qn b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);
        // quiet moves
        // b1->c1,d1,e1,f1,b2
        // e2->e1,e3,e4,d2,c2,b2
        // capture moves
        // b1->a1
        // e2->f2,a2

        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::b1, Square::c1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::b1, Square::d1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::b1, Square::e1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::b1, Square::f1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::b1, Square::b2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e2, Square::e1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e2, Square::e3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e2, Square::e4);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e2, Square::d2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e2, Square::c2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e2, Square::b2);
        assert!(move_list.contains(mv) == true);

        // check the capture moves
        mv = Mov::encode_move_capture(Square::b1, Square::a1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e2, Square::f2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e2, Square::a2);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_white_rook_move_list_as_expected() {
        let fen = "1nbk3p/NP1pQpP1/2p4p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/Pr4qn w - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);
        // quiet moves
        // b4->a4,b5,b6
        // c3->d3,e3,c2,c1
        // capture moves
        // c3->f3

        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::b4, Square::a4);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::b4, Square::b5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::b4, Square::b6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c3, Square::d3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c3, Square::e3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c3, Square::c2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::c3, Square::c1);
        assert!(move_list.contains(mv) == true);

        // check the capture moves
        mv = Mov::encode_move_capture(Square::c3, Square::f3);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_white_queen_move_list_as_expected() {
        let fen = "1nbk3p/NP1p1pP1/2p1Q2p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/Pr4qn w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        ////mov::print_move_list(&move_list);
        // quiet moves
        // e6->e7,e8,d6,f6,g6,f5,g4
        // capture moves
        // e6->c6,h6,d7,f7,e5

        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::e6, Square::e7);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e6, Square::e8);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e6, Square::d6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e6, Square::f6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e6, Square::g6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e6, Square::f5);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::e6, Square::g4);
        assert!(move_list.contains(mv) == true);

        // check the capture moves
        mv = Mov::encode_move_capture(Square::e6, Square::c6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e6, Square::h6);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e6, Square::d7);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e6, Square::f7);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::e6, Square::e5);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_black_queen_move_list_as_expected() {
        let fen = "1nbk3p/NP1p1pP1/2p1Q2p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/Pr4qn b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);
        // quiet moves
        // g1->f1,e1,d1,c1,g2,g3,g4
        // capture moves
        // g1->f2,h2

        // check the quiet moves
        let mut mv = Mov::encode_move_quiet(Square::g1, Square::f1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g1, Square::e1);
        assert!(move_list.contains(mv) == true);

        mv = Mov::encode_move_quiet(Square::g1, Square::d1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g1, Square::c1);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g1, Square::g2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g1, Square::g3);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_quiet(Square::g1, Square::g4);
        assert!(move_list.contains(mv) == true);

        // check the capture moves
        mv = Mov::encode_move_capture(Square::g1, Square::f2);
        assert!(move_list.contains(mv) == true);
        mv = Mov::encode_move_capture(Square::g1, Square::h2);
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_white_king_castle_move_move_list_as_expected() {
        let fen = "r2qk2r/pb1npp1p/1ppp1npb/8/4P3/1PNP1PP1/PBP1N1BP/R2QK2R w K - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Mov::encode_move_castle_kingside_white();
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_white_queen_castle_move_move_list_as_expected() {
        let fen = "r3k2r/pbqnpp1p/1ppp1npb/8/4P3/1PNP1PP1/PBPQN1BP/R3K2R w Q - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Mov::encode_move_castle_queenside_white();
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_black_king_castle_move_move_list_as_expected() {
        let fen = "r2qk2r/pb1npp1p/1ppp1npb/8/4P3/1PNP1PP1/PBP1N1BP/R2QK2R b k - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Mov::encode_move_castle_kingside_black();
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_black_queen_castle_move_move_list_as_expected() {
        let fen = "r3k2r/pbqnpp1p/1ppp1npb/8/4P3/1PNP1PP1/PBPQN1BP/R3K2R b q - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let cp = pos.castle_permissions();
        assert!(castle_permissions::is_black_queen_set(cp));

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Mov::encode_move_castle_queenside_black();
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_all_castle_options_available_list_as_expected() {
        // --- WHITE
        let fen = "r3k2r/pbqnpp1p/1ppp1npb/8/4P3/1PNP1PP1/PBPQN1BP/R3K2R w KQkq - 0 1";
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
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mut mv = Mov::encode_move_castle_queenside_white();
        assert!(move_list.contains(mv) == true);

        mv = Mov::encode_move_castle_kingside_white();
        assert!(move_list.contains(mv) == true);

        // --- BLACK
        pos.flip_side_to_move();

        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        mv = Mov::encode_move_castle_queenside_black();
        assert!(move_list.contains(mv) == true);

        mv = Mov::encode_move_castle_kingside_black();
        assert!(move_list.contains(mv) == true);
    }

    #[test]
    pub fn move_gen_white_promotion_moves_as_expected() {
        let fen = "2b1rkr1/PPpP1pbP/n1p4p/2NpP1p1/1RBqBP2/pPR1NpQ1/P4P1P/P4K1n w - - 0 1";

        let white_promotion_pces: [&'static Piece; 4] = [
            &piece::WHITE_BISHOP,
            &piece::WHITE_KNIGHT,
            &piece::WHITE_ROOK,
            &piece::WHITE_QUEEN,
        ];

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);

        let mut from_sq: Square;
        let mut to_sq: Square;

        from_sq = Square::a7;
        to_sq = Square::a8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Mov::encode_move_with_promotion(from_sq, to_sq, pce)));
        }

        from_sq = Square::b7;
        to_sq = Square::b8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Mov::encode_move_with_promotion(from_sq, to_sq, pce)));
        }

        from_sq = Square::d7;
        to_sq = Square::d8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Mov::encode_move_with_promotion(from_sq, to_sq, pce)));
        }

        from_sq = Square::h7;
        to_sq = Square::h8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Mov::encode_move_with_promotion(from_sq, to_sq, pce)));
        }
        // CAPTURE promotion
        from_sq = Square::b7;
        to_sq = Square::c8;
        for pce in white_promotion_pces.iter() {
            assert!(
                move_list.contains(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce))
            );
        }
        from_sq = Square::d7;
        to_sq = Square::c8;
        for pce in white_promotion_pces.iter() {
            assert!(
                move_list.contains(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce))
            );
        }

        from_sq = Square::d7;
        to_sq = Square::e8;
        for pce in white_promotion_pces.iter() {
            assert!(
                move_list.contains(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce))
            );
        }

        from_sq = Square::h7;
        to_sq = Square::g8;
        for pce in white_promotion_pces.iter() {
            assert!(
                move_list.contains(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce))
            );
        }
    }

    #[test]
    pub fn move_gen_black_promotion_moves_as_expected() {
        let fen = "2b1rkr1/PPpP1pbP/n6p/2NpPn2/1RBqBP2/4N1Q1/ppPpRp1P/P4K2 b - - 0 1";
        let black_promotion_pces: [&'static Piece; 4] = [
            &piece::BLACK_BISHOP,
            &piece::BLACK_KNIGHT,
            &piece::BLACK_ROOK,
            &piece::BLACK_QUEEN,
        ];

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);

        let mut from_sq: Square;
        let mut to_sq: Square;

        // QUITE promotion
        from_sq = Square::b2;
        to_sq = Square::b1;
        for pce in black_promotion_pces.iter() {
            assert!(move_list.contains(Mov::encode_move_with_promotion(from_sq, to_sq, pce)));
        }

        from_sq = Square::d2;
        to_sq = Square::d1;
        for pce in black_promotion_pces.iter() {
            assert!(move_list.contains(Mov::encode_move_with_promotion(from_sq, to_sq, pce)));
        }

        // CAPTURE promotion
        from_sq = Square::b2;
        to_sq = Square::a1;
        for pce in black_promotion_pces.iter() {
            assert!(
                move_list.contains(Mov::encode_move_with_promotion_capture(from_sq, to_sq, pce))
            );
        }
    }

    #[test]
    pub fn move_gen_white_first_moves_as_expected() {
        let fen = "4k2n/rbppBn1q/pP1pp3/1BQ5/P2N3p/pr2b3/P1NPPPPP/2R2R1K w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);

        // double first moves
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::f2, Square::f4)));
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::g2, Square::g4)));
        let num_double_pawn_moves = move_list.iter().filter(|&n| (*n).is_double_pawn()).count();
        assert!(num_double_pawn_moves == 2);

        // single first move
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d2, Square::d3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f2, Square::f3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::g2, Square::g3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h2, Square::h3)));

        // capture on first move
        assert!(move_list.contains(Mov::encode_move_capture(Square::a2, Square::b3)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::d2, Square::e3)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::f2, Square::e3)));
    }

    #[test]
    pub fn move_gen_black_first_moves_as_expected() {
        let fen = "4k2n/rbpp1ppq/pPNBp3/6n1/P7/prQBb3/P1NPPPPP/2R2R1K b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        //mov::print_move_list(&move_list);

        // double first moves
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::f7, Square::f5)));
        let num_double_pawn_moves = move_list.iter().filter(|&n| (*n).is_double_pawn()).count();
        assert!(num_double_pawn_moves == 1);

        // single first move
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f7, Square::f6)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::g7, Square::g6)));

        // capture on first move
        assert!(move_list.contains(Mov::encode_move_capture(Square::c7, Square::b6)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::c7, Square::d6)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::d7, Square::c6)));
    }

    #[test]
    pub fn move_gen_white_misc_pawn_moves_as_expected() {
        let fen = "2b1rkr1/P1p2pb1/n1p3pp/2NpPPP1/pPBq2BP/2R1NpQ1/P1PP1P1P/R4K1n w - d6 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // quiet moves
        assert!(move_list.contains(Mov::encode_move_quiet(Square::b4, Square::b5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f5, Square::f6)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h4, Square::h5)));

        // capture moves
        assert!(move_list.contains(Mov::encode_move_capture(Square::f5, Square::g6)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::g5, Square::h6)));

        // en passant move
        assert!(move_list.contains(Mov::encode_move_en_passant(Square::e5, Square::d6)));
    }

    #[test]
    pub fn move_gen_black_misc_pawn_moves_as_expected() {
        let fen = "2b1rkr1/P1p1qpb1/n5pN/2p3P1/pPBRpPBp/5pQ1/P1PPP1P1/R4K1N b - b3 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // quiet moves
        assert!(move_list.contains(Mov::encode_move_quiet(Square::a4, Square::a3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e4, Square::e3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f3, Square::f2)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h4, Square::h3)));

        // capture moves
        assert!(move_list.contains(Mov::encode_move_capture(Square::c5, Square::b4)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::c5, Square::d4)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::f3, Square::e2)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::f3, Square::g2)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::h4, Square::g3)));

        // en passant move
        assert!(move_list.contains(Mov::encode_move_en_passant(Square::a4, Square::b3)));
    }

    #[test]
    pub fn move_gen_all_moves_white_position_as_expected() {
        let fen = "3rr1k1/pp3pp1/1qn2np1/8/3p4/PP3P2/2P1NQPP/R1B1K2R w K - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        mov::print_move_list(&move_list);

        assert!(move_list.len() == 34);

        // quiet moves
        assert!(move_list.contains(Mov::encode_move_quiet(Square::a1, Square::a2)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::a1, Square::b1)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::c1, Square::b2)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c1, Square::d2)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c1, Square::e3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c1, Square::f4)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c1, Square::g5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c1, Square::h6)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::e1, Square::d1)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e1, Square::d2)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e1, Square::f1)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::h1, Square::g1)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h1, Square::f1)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::a3, Square::a4)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::b3, Square::b4)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::c2, Square::c3)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::e2, Square::c3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e2, Square::g1)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e2, Square::g3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e2, Square::f4)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::f2, Square::e3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f2, Square::g1)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f2, Square::g3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f2, Square::h4)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::f3, Square::f4)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::g2, Square::g3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h2, Square::h3)));

        // castle move
        assert!(move_list.contains(Mov::encode_move_castle_kingside_white()));

        // capture moves
        assert!(move_list.contains(Mov::encode_move_capture(Square::e2, Square::d4)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::f2, Square::d4)));

        // double pawn first move
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::c2, Square::c4)));
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::g2, Square::g4)));
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::h2, Square::h4)));
    }

    #[test]
    pub fn move_gen_all_moves_black_position_as_expected() {
        let fen = "3rr1k1/p4pp1/1pn2np1/2P4q/1P1p4/P4P2/4NQPP/R1B1K2R b - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &&occ_masks,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        mov::print_move_list(&move_list);

        assert!(move_list.len() == 45);

        // quiet moves
        assert!(move_list.contains(Mov::encode_move_quiet(Square::a7, Square::a6)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::b6, Square::b5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d4, Square::d3)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::c6, Square::b8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c6, Square::e7)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c6, Square::e5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::c6, Square::a5)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::d8, Square::d7)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d8, Square::d6)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d8, Square::d5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d8, Square::c8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d8, Square::b8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::d8, Square::a8)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::e8, Square::f8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e8, Square::e7)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e8, Square::e6)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e8, Square::e5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e8, Square::e4)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::e8, Square::e3)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::f6, Square::d7)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f6, Square::d5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f6, Square::e4)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f6, Square::g4)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::f6, Square::h7)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::g6, Square::g5)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::h6)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::h7)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::h8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::h4)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::h3)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::g4)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::g5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::f5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::e5)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::h5, Square::d5)));

        assert!(move_list.contains(Mov::encode_move_quiet(Square::g8, Square::f8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::g8, Square::h8)));
        assert!(move_list.contains(Mov::encode_move_quiet(Square::g8, Square::h7)));

        // capture moves
        assert!(move_list.contains(Mov::encode_move_capture(Square::b6, Square::c5)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::c6, Square::b4)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::e8, Square::e2)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::h5, Square::h2)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::h5, Square::f3)));
        assert!(move_list.contains(Mov::encode_move_capture(Square::h5, Square::c5)));

        // double pawn first move
        assert!(move_list.contains(Mov::encode_move_double_pawn_first(Square::a7, Square::a5)));
    }
}
