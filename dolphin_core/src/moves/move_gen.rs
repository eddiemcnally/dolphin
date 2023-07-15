use crate::board::bitboard::Bitboard;
use crate::board::colour::Colour;
use crate::board::occupancy_masks::OccupancyMasks;
use crate::board::piece::Piece;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::moves::mov::Move;
use crate::moves::move_list::MoveList;
use crate::position::game_position::Position;

pub struct MoveGenerator {}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator {}
    }

    pub fn generate_moves(&self, pos: &Position, move_list: &mut MoveList) -> u16 {
        let move_cnt_start = move_list.len();

        let all_bb = pos.board().get_bitboard();

        match pos.side_to_move() {
            Colour::White => {
                // opposite colour bitboard
                let all_opposing_bb = pos.board().get_colour_bb(Colour::Black);

                // pawn moves
                self.generate_white_pawn_moves(pos, all_bb, all_opposing_bb, move_list);

                // bishop and queen - conflate for diagonal move gen
                let wbq_bb = pos.board().get_white_bishop_queen_bitboard();
                self.generate_diagonal_moves(pos, wbq_bb, all_bb, move_list);

                // knight
                let wn_bb = pos.board().get_piece_bitboard(Piece::Knight, Colour::White);
                self.generate_knight_moves(pos, wn_bb, move_list);

                //  rook and queen - conflate for horizontal and vertical move gen
                let wrq_bb = pos.board().get_white_rook_queen_bitboard();
                self.generate_rank_file_moves(pos, wrq_bb, all_bb, move_list);

                // king
                let king_sq = pos.board().get_king_sq(Colour::White);
                self.generate_king_moves(pos, king_sq, move_list);

                // castle moves
                if pos.castle_permissions().has_white_castle_permission() {
                    self.generate_white_castle_moves(pos, move_list);
                }
            }
            Colour::Black => {
                // opposite colour bitboard
                let all_opposing_bb = pos.board().get_colour_bb(Colour::White);

                // pawn
                self.generate_black_pawn_moves(pos, all_bb, all_opposing_bb, move_list);

                // bishop and queen - conflate for diagonal move gen
                let bbq_bb = pos.board().get_black_bishop_queen_bitboard();
                self.generate_diagonal_moves(pos, bbq_bb, all_bb, move_list);

                // knight
                let bn_bb = pos.board().get_piece_bitboard(Piece::Knight, Colour::Black);
                self.generate_knight_moves(pos, bn_bb, move_list);

                //  rook and queen - conflate for horizontal and vertical move gen
                let brq_bb = pos.board().get_black_rook_queen_bitboard();
                self.generate_rank_file_moves(pos, brq_bb, all_bb, move_list);

                // king
                let king_sq = pos.board().get_king_sq(Colour::Black);
                self.generate_king_moves(pos, king_sq, move_list);

                // castle moves
                if pos.castle_permissions().has_black_castle_permission() {
                    self.generate_black_castle_moves(pos, move_list);
                }
            }
        }
        let move_cnt_end = move_list.len();

        (move_cnt_end - move_cnt_start) as u16
    }

    fn generate_white_pawn_moves(
        &self,
        pos: &Position,
        all_bb: Bitboard,
        all_opposing_bb: Bitboard,
        move_list: &mut MoveList,
    ) {
        for from_sq in pos
            .board()
            .get_piece_bitboard(Piece::Pawn, Colour::White)
            .iterator()
        {
            let rank = from_sq.rank();

            match rank {
                Rank::R1 => panic!("Invalid Rank 1"),
                Rank::R8 => panic!("Invalid Rank 8"),
                Rank::R2 | Rank::R3 | Rank::R4 | Rank::R5 | Rank::R6 => {
                    if rank == Rank::R2 {
                        // double pawn moves
                        let double_first_move_sq_mask = pos
                            .occupancy_masks()
                            .get_occ_mask_white_pawns_double_move_mask(from_sq);

                        if (all_bb & double_first_move_sq_mask).is_empty() {
                            // both squares free
                            let to_sq = from_sq.plus_2_ranks().unwrap();

                            let mv = Move::encode_move_double_pawn_first(from_sq, to_sq);
                            move_list.push(mv);
                        }
                    }

                    // quiet move
                    let quiet_to_sq = from_sq.plus_1_rank();
                    if !all_bb.is_set(quiet_to_sq.unwrap()) {
                        let mv = Move::encode_move_quiet(from_sq, quiet_to_sq.unwrap());
                        move_list.push(mv);
                    }

                    // capture moves
                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_white_pawn_attack_squares(from_sq);

                    (capt_mask & all_opposing_bb).iterator().for_each(|to_sq| {
                        let mv = Move::encode_move_capture(from_sq, to_sq);
                        move_list.push(mv);
                    });

                    // en passant move
                    if let Some(en_sq) = pos.en_passant_square() {
                        if capt_mask.is_set(en_sq) {
                            // en passant sq can be "captured"
                            let en_pass_mv = Move::encode_move_en_passant(from_sq, en_sq);
                            move_list.push(en_pass_mv);
                        }
                    }
                }
                Rank::R7 => {
                    // quiet promotion
                    let quiet_to_sq = from_sq.plus_1_rank();
                    if !all_bb.is_set(quiet_to_sq.unwrap()) {
                        // free square ahead
                        self.encode_promotion_moves(from_sq, quiet_to_sq.unwrap(), move_list);
                    }

                    // capture promotion
                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_white_pawn_attack_squares(from_sq);
                    (capt_mask & all_opposing_bb).iterator().for_each(|to_sq| {
                        self.encode_promotion_capture_moves(from_sq, to_sq, move_list);
                    });
                }
            };
        }
    }

    fn generate_black_pawn_moves(
        &self,
        pos: &Position,
        all_bb: Bitboard,
        all_opposing_bb: Bitboard,
        move_list: &mut MoveList,
    ) {
        for from_sq in pos
            .board()
            .get_piece_bitboard(Piece::Pawn, Colour::Black)
            .iterator()
        {
            match from_sq.rank() {
                Rank::R1 => panic!("Invalid Rank 1"),
                Rank::R8 => panic!("Invalid Rank 8"),
                Rank::R3 | Rank::R4 | Rank::R5 | Rank::R6 | Rank::R7 => {
                    if from_sq.rank() == Rank::R7 {
                        let double_first_move_sq_mask = pos
                            .occupancy_masks()
                            .get_occ_mask_black_pawns_double_move_mask(from_sq);

                        if (all_bb & double_first_move_sq_mask).is_empty() {
                            // both squares free
                            let to_sq = from_sq.minus_2_ranks().unwrap();

                            let mv = Move::encode_move_double_pawn_first(from_sq, to_sq);
                            move_list.push(mv);
                        }
                    }

                    // quiet moves + capture move
                    let quiet_to_sq = from_sq.minus_1_rank();
                    if !all_bb.is_set(quiet_to_sq.unwrap()) {
                        let mv = Move::encode_move_quiet(from_sq, quiet_to_sq.unwrap());
                        move_list.push(mv);
                    }

                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_black_pawn_attack_squares(from_sq);

                    (capt_mask & all_opposing_bb).iterator().for_each(|to_sq| {
                        let mv = Move::encode_move_capture(from_sq, to_sq);
                        move_list.push(mv);
                    });

                    // en passant move
                    if let Some(en_sq) = pos.en_passant_square() {
                        if capt_mask.is_set(en_sq) {
                            // en passant sq can be "captured"
                            let en_pass_mv = Move::encode_move_en_passant(from_sq, en_sq);
                            move_list.push(en_pass_mv);
                        }
                    }
                }
                Rank::R2 => {
                    // quiet promotion
                    let quiet_to_sq = from_sq.minus_1_rank();
                    if !all_bb.is_set(quiet_to_sq.unwrap()) {
                        // free square ahead
                        self.encode_promotion_moves(from_sq, quiet_to_sq.unwrap(), move_list);
                    }

                    // capture promotion
                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_black_pawn_attack_squares(from_sq);
                    let capt_bb = capt_mask & all_opposing_bb;
                    capt_bb.iterator().for_each(|to_sq| {
                        self.encode_promotion_capture_moves(from_sq, to_sq, move_list);
                    });
                }
            };
        }
    }

    // generates diagonal and anti-diagonal moves for queen and bishop
    // see Hyperbola Quintessence
    fn generate_diagonal_moves(
        &self,
        pos: &Position,
        pce_bb: Bitboard,
        all_bb: Bitboard,
        move_list: &mut MoveList,
    ) {
        let occ_col_bb = pos.board().get_colour_bb(pos.side_to_move());

        pce_bb.iterator().for_each(|from_sq| {
            let diagonal_masks = pos.occupancy_masks().get_diag_antidiag_mask(from_sq);
            let slider_bb = from_sq.get_square_as_bb();

            // diagonal moves
            let diag1 = (all_bb & diagonal_masks.get_diag_mask())
                .overflowing_sub(slider_bb.overflowing_mul(2).0)
                .0;
            let diag2 = ((all_bb & diagonal_masks.get_diag_mask())
                .reverse_bits()
                .overflowing_sub(slider_bb.reverse_bits().overflowing_mul(2).0))
            .0
            .reverse_bits();
            let diag = Bitboard::new(diag1 ^ diag2);

            // anti-diagonal moves
            let antidiag1 = (all_bb & diagonal_masks.get_anti_diag_mask())
                .overflowing_sub(slider_bb.overflowing_mul(2).0)
                .0;
            let antidiag2 = ((all_bb & diagonal_masks.get_anti_diag_mask())
                .reverse_bits()
                .overflowing_sub(slider_bb.reverse_bits().overflowing_mul(2).0))
            .0
            .reverse_bits();

            let antidiag = Bitboard::new(antidiag1 ^ antidiag2);

            let all_moves = (diag & diagonal_masks.get_diag_mask())
                | (antidiag & diagonal_masks.get_anti_diag_mask());
            let excl_same_colour = all_moves & !occ_col_bb;

            excl_same_colour.iterator().for_each(|to_sq| {
                let mv = if pos.board().is_sq_empty(to_sq) {
                    Move::encode_move_quiet(from_sq, to_sq)
                } else {
                    Move::encode_move_capture(from_sq, to_sq)
                };

                move_list.push(mv);
            });
        });
    }

    // generates sliding rank and file moves for queen and rook
    // see Hyperbola Quintessence
    fn generate_rank_file_moves(
        &self,
        pos: &Position,
        pce_bb: Bitboard,
        occ_sq_bb: Bitboard,
        move_list: &mut MoveList,
    ) {
        let occ_col_bb = pos.board().get_colour_bb(pos.side_to_move());

        pce_bb.iterator().for_each(|from_sq| {
            let horiz_mask = pos.occupancy_masks().get_horizontal_mask(from_sq);
            let vertical_mask = pos.occupancy_masks().get_vertical_mask(from_sq);

            let slider_bb = from_sq.get_square_as_bb();
            let slider_bb_reverse = slider_bb.reverse_bits();

            // horizontal moves
            let horiz1 = occ_sq_bb.overflowing_sub(slider_bb.overflowing_mul(2).0).0;
            let horiz2 = (occ_sq_bb
                .reverse_bits()
                .overflowing_sub(slider_bb_reverse.overflowing_mul(2).0)
                .0)
                .reverse_bits();
            let horiz = Bitboard::new(horiz1 ^ horiz2);

            // vertical moves
            let vert1 = (occ_sq_bb & vertical_mask)
                .overflowing_sub(slider_bb.overflowing_mul(2).0)
                .0;
            let vert2 = ((occ_sq_bb & vertical_mask)
                .reverse_bits()
                .overflowing_sub(slider_bb_reverse.overflowing_mul(2).0))
            .0
            .reverse_bits();
            let vert = Bitboard::new(vert1 ^ vert2);

            let all_moves_mask = (horiz & horiz_mask) | (vert & vertical_mask);

            let all_excl_same_col = all_moves_mask & !occ_col_bb;

            all_excl_same_col.iterator().for_each(|to_sq| {
                let mv = if pos.board().is_sq_empty(to_sq) {
                    Move::encode_move_quiet(from_sq, to_sq)
                } else {
                    Move::encode_move_capture(from_sq, to_sq)
                };

                move_list.push(mv);
            });
        });
    }

    fn generate_knight_moves(&self, pos: &Position, knight_bb: Bitboard, move_list: &mut MoveList) {
        let opposite_side = pos.side_to_move().flip_side();
        let opp_occ_sq_bb = pos.board().get_colour_bb(opposite_side);

        knight_bb.iterator().for_each(|from_sq| {
            let occ_mask = pos.occupancy_masks().get_occupancy_mask_knight(from_sq);

            // generate capture moves
            // AND'ing with opposite colour pieces with the occupancy mask, will
            // give all pieces that can be captured by the piece on this square
            (opp_occ_sq_bb & occ_mask).iterator().for_each(|to_sq| {
                let mov = Move::encode_move_capture(from_sq, to_sq);
                move_list.push(mov);
            });

            // generate quiet moves
            let unoccupied_squares_bb = !pos.board().get_bitboard();
            let quiet_move_bb = unoccupied_squares_bb & occ_mask;
            quiet_move_bb.iterator().for_each(|to_sq| {
                let mov = Move::encode_move_quiet(from_sq, to_sq);
                move_list.push(mov);
            });
        });
    }

    fn generate_king_moves(&self, pos: &Position, from_sq: Square, move_list: &mut MoveList) {
        let opposite_side = pos.side_to_move().flip_side();
        let opp_occ_sq_bb = pos.board().get_colour_bb(opposite_side);

        let occ_mask = pos.occupancy_masks().get_occupancy_mask_king(from_sq);

        // generate capture moves
        // ----------------------
        // AND'ing with opposite colour pieces with the occupancy mask, will
        // give all pieces that can be captured by the piece on this square
        let capt_bb = opp_occ_sq_bb & occ_mask;
        capt_bb.iterator().for_each(|to_sq| {
            let mov = Move::encode_move_capture(from_sq, to_sq);
            move_list.push(mov);
        });

        // generate quiet moves
        let unoccupied_squares_bb = !pos.board().get_bitboard();
        let quiet_move_bb = unoccupied_squares_bb & occ_mask;
        quiet_move_bb.iterator().for_each(|to_sq| {
            let mov = Move::encode_move_quiet(from_sq, to_sq);
            move_list.push(mov);
        });
    }

    fn generate_white_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if cp.is_white_king_set() && (bb & OccupancyMasks::CASTLE_MASK_WK).is_empty() {
            let mv = Move::encode_move_castle_kingside_white();
            move_list.push(mv);
        }
        if cp.is_white_queen_set() && (bb & OccupancyMasks::CASTLE_MASK_WQ).is_empty() {
            let mv = Move::encode_move_castle_queenside_white();
            move_list.push(mv);
        }
    }

    fn generate_black_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if cp.is_black_king_set() && (bb & OccupancyMasks::CASTLE_MASK_BK).is_empty() {
            let mv = Move::encode_move_castle_kingside_black();
            move_list.push(mv);
        }
        if cp.is_black_queen_set() && (bb & OccupancyMasks::CASTLE_MASK_BQ).is_empty() {
            let mv = Move::encode_move_castle_queenside_black();
            move_list.push(mv);
        }
    }

    fn encode_promotion_moves(&self, from_sq: Square, to_sq: Square, move_list: &mut MoveList) {
        for pce in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
            move_list.push(Move::encode_move_with_promotion(from_sq, to_sq, pce));
        }
    }

    fn encode_promotion_capture_moves(
        &self,
        from_sq: Square,
        to_sq: Square,
        move_list: &mut MoveList,
    ) {
        [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen]
            .into_iter()
            .for_each(|pce| {
                move_list.push(Move::encode_move_with_promotion_capture(
                    from_sq, to_sq, pce,
                ));
            });
    }
}

#[cfg(test)]
pub mod tests {
    use crate::board::occupancy_masks::OccupancyMasks;
    use crate::board::piece::Piece;
    use crate::board::square::*;
    use crate::io::fen;
    use crate::moves::mov::Move;
    use crate::moves::move_gen::MoveGenerator;
    use crate::moves::move_list::MoveList;
    use crate::position::attack_checker::AttackChecker;
    use crate::position::game_position::Position;
    use crate::position::zobrist_keys::ZobristKeys;

    #[test]
    pub fn move_gen_white_king_knight_move_list_as_expected() {
        let fen = "1n1k2b1/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/3q3n w - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);
        // check the capture moves
        let mut mv = Move::encode_move_capture(Square::E3, Square::D1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E3, Square::C2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::A6, Square::B8);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::A6, Square::C7);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::G5, Square::H6);
        assert!(move_list.contains(mv));

        // check the quiet moves
        mv = Move::encode_move_quiet(Square::A6, Square::C5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::F1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::G2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::G4);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::F5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::D5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::G6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::F6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::F5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::G4);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::H4);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::H5);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_black_king_knight_move_list_as_expected() {
        let fen = "1n1k2b1/1PppQpb1/N1p4p/1B2P1K1/1RB2P2/pPR1Np2/P1r1rP1P/3q3n b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the capture moves
        let mut mv = Move::encode_move_capture(Square::H1, Square::F2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::D8, Square::E7);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::B8, Square::A6);
        assert!(move_list.contains(mv));

        // check the quiet moves
        mv = Move::encode_move_quiet(Square::D8, Square::C8);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D8, Square::E8);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::H1, Square::G3);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_white_bishop_move_list_as_expected() {
        let fen = "1n1k2b1/1PppQpb1/N1p4p/4P1K1/1RB1BP2/pPR1Np2/P1r1rP1P/3q3n w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the quiet moves
        let mut mv = Move::encode_move_quiet(Square::C4, Square::B5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C4, Square::D5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C4, Square::E6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C4, Square::D3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::D5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::D3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::F5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::G6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::H7);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::E4, Square::C2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E4, Square::F3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E4, Square::C6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::C4, Square::E2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::C4, Square::F7);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_black_bishop_move_list_as_expected() {
        let fen = "1nbk4/NP1pQpP1/2p4p/p5K1/1RBbBP2/pPR1Np2/P1r1rP1P/3q3n b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the quiet moves
        let mut mv = Move::encode_move_quiet(Square::D4, Square::C5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D4, Square::B6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D4, Square::E5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D4, Square::F6);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::C8, Square::B7);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::D4, Square::C3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::D4, Square::E3);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_black_rook_move_list_as_expected() {
        let fen = "1nbk4/NP1pQpP1/2p4p/p2Bb1K1/1R3P2/pPR2p1P/P3rP1N/Br4qn b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the quiet moves
        let mut mv = Move::encode_move_quiet(Square::B1, Square::C1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::D1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::E1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::F1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::B2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::E1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::E3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::E4);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::D2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::C2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::B2);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::B1, Square::A1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E2, Square::F2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E2, Square::A2);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_white_rook_move_list_as_expected() {
        let fen = "1nbk4/NP1pQpP1/2p4p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/1r4qn w - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the quiet moves
        let mut mv = Move::encode_move_quiet(Square::B4, Square::A4);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B4, Square::B5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B4, Square::B6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::D3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::E3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::C2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::C1);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::C3, Square::F3);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_white_queen_move_list_as_expected() {
        let fen = "1nbk4/NP1p1pP1/2p1Q2p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/1r4qn w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the quiet moves
        let mut mv = Move::encode_move_quiet(Square::E6, Square::E7);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::E8);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::D6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::F6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::G6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::F5);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::G4);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::E6, Square::C6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::H6);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::D7);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::F7);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::E5);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_black_queen_move_list_as_expected() {
        let fen = "1nbk4/NP1p1pP1/2p1Q2p/p2Bb1K1/1RB2P2/pPR2p1P/P3rP1N/1r4qn b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // check the quiet moves
        let mut mv = Move::encode_move_quiet(Square::G1, Square::F1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::E1);
        assert!(move_list.contains(mv));

        mv = Move::encode_move_quiet(Square::G1, Square::D1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::C1);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::G2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::G3);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::G4);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::G1, Square::F2);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::G1, Square::H2);
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_white_king_castle_move_move_list_as_expected() {
        let fen = "r2qk2r/pb1npp1p/1ppp1npb/8/4P3/1PNP1PP1/PBP1N1BP/R2QK2R w K - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Move::encode_move_castle_kingside_white();
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_white_queen_castle_move_move_list_as_expected() {
        let fen = "r3k2r/pbqnpp1p/1ppp1npb/8/4P3/1PNP1PP1/PBPQN1BP/R3K2R w Q - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Move::encode_move_castle_queenside_white();
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_black_king_castle_move_move_list_as_expected() {
        let fen = "r2qk2r/pb1npp1p/1ppp1npb/8/4P3/1PNP1PP1/PBP1N1BP/R2QK2R b k - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Move::encode_move_castle_kingside_black();
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_black_queen_castle_move_move_list_as_expected() {
        let fen = "r3k2r/pbqnpp1p/1ppp1npb/8/4P3/1PNP1PP1/PBPQN1BP/R3K2R b q - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let cp = pos.castle_permissions();
        assert!(cp.is_black_queen_set());

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mv = Move::encode_move_castle_queenside_black();
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_all_castle_options_available_list_as_expected() {
        // --- WHITE
        let fen = "r3k2r/pbqnpp1p/1ppp1npb/8/4P3/1PNP1PP1/PBPQN1BP/R3K2R w KQkq - 0 1";
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

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mut mv = Move::encode_move_castle_queenside_white();
        assert!(move_list.contains(mv));

        mv = Move::encode_move_castle_kingside_white();
        assert!(move_list.contains(mv));

        // --- BLACK
        pos.flip_side_to_move();

        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        mv = Move::encode_move_castle_queenside_black();
        assert!(move_list.contains(mv));

        mv = Move::encode_move_castle_kingside_black();
        assert!(move_list.contains(mv));
    }

    #[test]
    pub fn move_gen_white_promotion_moves_as_expected() {
        let fen = "2b1rkr1/PPpP1pbP/n1p4p/2NpP1p1/1RBqBP2/pPR1NpQ1/P4P1P/5K1n w - - 0 1";

        let white_promotion_pces: [Piece; 4] =
            [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen];

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        let mut from_sq = Square::A7;
        let mut to_sq = Square::A8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *pce)));
        }

        from_sq = Square::B7;
        to_sq = Square::B8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *pce)));
        }

        from_sq = Square::D7;
        to_sq = Square::D8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *pce)));
        }

        from_sq = Square::H7;
        to_sq = Square::H8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *pce)));
        }
        // CAPTURE promotion
        from_sq = Square::B7;
        to_sq = Square::C8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *pce
            )));
        }
        from_sq = Square::D7;
        to_sq = Square::C8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *pce
            )));
        }

        from_sq = Square::D7;
        to_sq = Square::E8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *pce
            )));
        }

        from_sq = Square::H7;
        to_sq = Square::G8;
        for pce in white_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *pce
            )));
        }
    }

    #[test]
    pub fn move_gen_black_promotion_moves_as_expected() {
        let fen = "2b1rkr1/PPpP1pbP/n6p/2NpPn2/1RBqBP2/4N1Q1/ppPpRp1P/B4K2 b - - 0 1";
        let black_promotion_pces: [Piece; 4] =
            [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen];

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // QUITE promotion
        let mut from_sq = Square::B2;
        let mut to_sq = Square::B1;
        for pce in black_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *pce)));
        }

        from_sq = Square::D2;
        to_sq = Square::D1;
        for pce in black_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *pce)));
        }

        // CAPTURE promotion
        from_sq = Square::B2;
        to_sq = Square::A1;
        for pce in black_promotion_pces.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *pce
            )));
        }
    }

    #[test]
    pub fn move_gen_white_first_moves_as_expected() {
        let fen = "4k2n/rbppBn1q/pP1pp3/1BQ5/P2N3p/pr2b3/P1NPPPPP/2R2R1K w - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // double first moves
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::F2, Square::F4)));
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::G2, Square::G4)));
        let num_double_pawn_moves = move_list
            .iterator()
            .filter(|&n| (*n).is_double_pawn())
            .count();
        assert!(num_double_pawn_moves == 2);

        // single first move
        assert!(move_list.contains(Move::encode_move_quiet(Square::D2, Square::D3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F2, Square::F3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G2, Square::G3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H2, Square::H3)));

        // capture on first move
        assert!(move_list.contains(Move::encode_move_capture(Square::A2, Square::B3)));
        assert!(move_list.contains(Move::encode_move_capture(Square::D2, Square::E3)));
        assert!(move_list.contains(Move::encode_move_capture(Square::F2, Square::E3)));
    }

    #[test]
    pub fn move_gen_black_first_moves_as_expected() {
        let fen = "4k2n/rbpp1ppq/pPNBp3/6n1/P7/prQBb3/P1NPPPPP/2R2R1K b - - 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // double first moves
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::F7, Square::F5)));
        let num_double_pawn_moves = move_list
            .iterator()
            .filter(|&n| (*n).is_double_pawn())
            .count();
        assert!(num_double_pawn_moves == 1);

        // single first move
        assert!(move_list.contains(Move::encode_move_quiet(Square::F7, Square::F6)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G7, Square::G6)));

        // capture on first move
        assert!(move_list.contains(Move::encode_move_capture(Square::C7, Square::B6)));
        assert!(move_list.contains(Move::encode_move_capture(Square::C7, Square::D6)));
        assert!(move_list.contains(Move::encode_move_capture(Square::D7, Square::C6)));
    }

    #[test]
    pub fn move_gen_white_misc_pawn_moves_as_expected() {
        let fen = "2b1rkr1/P1p2pb1/n1p3pp/2NpPPP1/pPBq2BP/2R1NpQ1/P1PP1P1P/R4K1n w - d6 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // quiet moves
        assert!(move_list.contains(Move::encode_move_quiet(Square::B4, Square::B5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F5, Square::F6)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H4, Square::H5)));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(Square::F5, Square::G6)));
        assert!(move_list.contains(Move::encode_move_capture(Square::G5, Square::H6)));

        // en passant move
        assert!(move_list.contains(Move::encode_move_en_passant(Square::E5, Square::D6)));
    }

    #[test]
    pub fn move_gen_black_misc_pawn_moves_as_expected() {
        let fen = "2b1rkr1/P1p1qpb1/n5pN/2p3P1/pPBRpPBp/5pQ1/P1PPP1P1/R4K1N b - b3 0 1";
        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        // quiet moves
        assert!(move_list.contains(Move::encode_move_quiet(Square::A4, Square::A3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E4, Square::E3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F3, Square::F2)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H4, Square::H3)));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(Square::C5, Square::B4)));
        assert!(move_list.contains(Move::encode_move_capture(Square::C5, Square::D4)));
        assert!(move_list.contains(Move::encode_move_capture(Square::F3, Square::E2)));
        assert!(move_list.contains(Move::encode_move_capture(Square::F3, Square::G2)));
        assert!(move_list.contains(Move::encode_move_capture(Square::H4, Square::G3)));

        // en passant move
        assert!(move_list.contains(Move::encode_move_en_passant(Square::A4, Square::B3)));
    }

    #[test]
    pub fn move_gen_all_moves_white_position_as_expected() {
        let fen = "3rr1k1/pp3pp1/1qn2np1/8/3p4/PP3P2/2P1NQPP/R1B1K2R w K - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        move_list.print();

        assert!(move_list.len() == 34);

        // quiet moves
        assert!(move_list.contains(Move::encode_move_quiet(Square::A1, Square::A2)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::A1, Square::B1)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::C1, Square::B2)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C1, Square::D2)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C1, Square::E3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C1, Square::F4)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C1, Square::G5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C1, Square::H6)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::E1, Square::D1)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E1, Square::D2)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E1, Square::F1)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::H1, Square::G1)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H1, Square::F1)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::A3, Square::A4)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::B3, Square::B4)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::C2, Square::C3)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::E2, Square::C3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E2, Square::G1)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E2, Square::G3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E2, Square::F4)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::F2, Square::E3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F2, Square::G1)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F2, Square::G3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F2, Square::H4)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::F3, Square::F4)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::G2, Square::G3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H2, Square::H3)));

        // castle move
        assert!(move_list.contains(Move::encode_move_castle_kingside_white()));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(Square::E2, Square::D4)));
        assert!(move_list.contains(Move::encode_move_capture(Square::F2, Square::D4)));

        // double pawn first move
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::C2, Square::C4)));
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::G2, Square::G4)));
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::H2, Square::H4)));
    }

    #[test]
    pub fn move_gen_all_moves_black_position_as_expected() {
        let fen = "3rr1k1/p4pp1/1pn2np1/2P4q/1P1p4/P4P2/4NQPP/R1B1K2R b - - 0 1";

        let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) =
            fen::decompose_fen(fen);

        let zobrist_keys = ZobristKeys::new();
        let occ_masks = OccupancyMasks::new();
        let attack_checker = AttackChecker::new();

        let pos = Position::new(
            board,
            castle_permissions,
            move_cntr,
            en_pass_sq,
            side_to_move,
            &zobrist_keys,
            &occ_masks,
            &attack_checker,
        );

        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        move_list.print();

        assert!(move_list.len() == 45);

        // quiet moves
        assert!(move_list.contains(Move::encode_move_quiet(Square::A7, Square::A6)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::B6, Square::B5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D4, Square::D3)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::C6, Square::B8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C6, Square::E7)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C6, Square::E5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::C6, Square::A5)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::D7)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::D6)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::D5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::C8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::B8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::A8)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::F8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E7)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E6)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E4)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E3)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::F6, Square::D7)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F6, Square::D5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F6, Square::E4)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F6, Square::G4)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F6, Square::H7)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::G6, Square::G5)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::H6)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::H7)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::H8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::H4)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::H3)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::G4)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::G5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::F5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::E5)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H5, Square::D5)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::G8, Square::F8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G8, Square::H8)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G8, Square::H7)));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(Square::B6, Square::C5)));
        assert!(move_list.contains(Move::encode_move_capture(Square::C6, Square::B4)));
        assert!(move_list.contains(Move::encode_move_capture(Square::E8, Square::E2)));
        assert!(move_list.contains(Move::encode_move_capture(Square::H5, Square::H2)));
        assert!(move_list.contains(Move::encode_move_capture(Square::H5, Square::F3)));
        assert!(move_list.contains(Move::encode_move_capture(Square::H5, Square::C5)));

        // double pawn first move
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::A7, Square::A5)));
    }
}
