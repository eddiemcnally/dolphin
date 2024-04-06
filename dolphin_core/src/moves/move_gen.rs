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

        if pos.side_to_move() == Colour::White {
            self.generate_white_pawn_moves(pos, move_list);

            // castle moves
            if pos.castle_permissions().has_white_castle_permission() {
                self.generate_white_castle_moves(pos, move_list);
            }
        } else {
            // pawn
            self.generate_black_pawn_moves(pos, move_list);

            // castle moves
            if pos.castle_permissions().has_black_castle_permission() {
                self.generate_black_castle_moves(pos, move_list);
            }
        }

        self.generate_non_sliding_moves(pos, move_list);
        self.generate_sliding_moves(pos, move_list);

        let move_cnt_end = move_list.len();

        (move_cnt_end - move_cnt_start) as u16
    }

    fn generate_white_pawn_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let all_bb = pos.board().get_bitboard();
        let all_opposing_bb = pos.board().get_colour_bb(Colour::Black);
        let wp_bb = pos.board().get_piece_bitboard(Piece::Pawn, Colour::White);

        wp_bb.iterator().for_each(|from_sq| {
            let rank = from_sq.rank();

            match rank {
                Rank::R1 | Rank::R8 => (),
                Rank::R2 | Rank::R3 | Rank::R4 | Rank::R5 | Rank::R6 => {
                    // quiet move
                    let quiet_to_sq = from_sq.north().expect("Invalid nort() square");
                    if all_bb.is_clear(quiet_to_sq) {
                        let mv = Move::encode_move_quiet(from_sq, quiet_to_sq, Piece::Pawn);
                        move_list.push(mv);
                    }

                    // capture moves
                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_white_pawn_attack_squares(from_sq);

                    (capt_mask & all_opposing_bb).iterator().for_each(|to_sq| {
                        let mv = Move::encode_move_capture(from_sq, to_sq, Piece::Pawn);
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
                    let quiet_to_sq = from_sq.north().expect("Invalid north() square");
                    if all_bb.is_clear(quiet_to_sq) {
                        // free square ahead
                        self.encode_promotion_moves(from_sq, quiet_to_sq, move_list);
                    }

                    // capture promotion
                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_white_pawn_attack_squares(from_sq);
                    let all_opposing_bb = pos.board().get_colour_bb(Colour::Black);
                    (capt_mask & all_opposing_bb).iterator().for_each(|to_sq| {
                        self.encode_promotion_capture_moves(from_sq, to_sq, move_list);
                    });
                }
                _ => (),
            };
        });

        // do any double pawn first moves
        (wp_bb & OccupancyMasks::RANK_2_BB)
            .iterator()
            .for_each(|from_sq| {
                // double pawn moves
                let double_first_move_sq_mask = pos
                    .occupancy_masks()
                    .get_occ_mask_white_pawns_double_move_mask(from_sq);

                if (all_bb & double_first_move_sq_mask).is_empty() {
                    // both squares free
                    let to_sq = from_sq
                        .north()
                        .expect("Invalid north() square")
                        .north()
                        .expect("Invalid north/north square");

                    let mv = Move::encode_move_double_pawn_first(from_sq, to_sq);
                    move_list.push(mv);
                }
            });
    }

    fn generate_black_pawn_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let all_bb = pos.board().get_bitboard();
        let all_opposing_bb = pos.board().get_colour_bb(Colour::White);
        let bp_bb = pos.board().get_piece_bitboard(Piece::Pawn, Colour::Black);

        bp_bb.iterator().for_each(|from_sq| {
            let rank = from_sq.rank();

            match rank {
                Rank::R1 | Rank::R8 => (),
                Rank::R3 | Rank::R4 | Rank::R5 | Rank::R6 | Rank::R7 => {
                    // quiet moves + capture move
                    let quiet_to_sq = from_sq.south().expect("Invalid south() square");
                    if !all_bb.is_set(quiet_to_sq) {
                        let mv = Move::encode_move_quiet(from_sq, quiet_to_sq, Piece::Pawn);
                        move_list.push(mv);
                    }

                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_black_pawn_attack_squares(from_sq);

                    (capt_mask & all_opposing_bb).iterator().for_each(|to_sq| {
                        let mv = Move::encode_move_capture(from_sq, to_sq, Piece::Pawn);
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
                    let quiet_to_sq = from_sq.south().expect("Invalid south() square");
                    if !all_bb.is_set(quiet_to_sq) {
                        // free square ahead
                        self.encode_promotion_moves(from_sq, quiet_to_sq, move_list);
                    }

                    // capture promotion
                    let capt_mask = pos
                        .occupancy_masks()
                        .get_occ_mask_black_pawn_attack_squares(from_sq);
                    let all_opposing_bb = pos.board().get_colour_bb(Colour::White);
                    let capt_bb = capt_mask & all_opposing_bb;
                    capt_bb.iterator().for_each(|to_sq| {
                        self.encode_promotion_capture_moves(from_sq, to_sq, move_list);
                    });
                }
                _ => (),
            };
        });

        // do any double pawn first moves
        (bp_bb & OccupancyMasks::RANK_7_BB)
            .iterator()
            .for_each(|from_sq| {
                let double_first_move_sq_mask = pos
                    .occupancy_masks()
                    .get_occ_mask_black_pawns_double_move_mask(from_sq);

                if (all_bb & double_first_move_sq_mask).is_empty() {
                    // both squares free
                    let to_sq = from_sq
                        .south()
                        .expect("Invalid south() square")
                        .south()
                        .expect("Invalud south/south square");

                    let mv = Move::encode_move_double_pawn_first(from_sq, to_sq);
                    move_list.push(mv);
                }
            });
    }

    fn generate_sliding_moves(&self, pos: &Position, move_list: &mut MoveList) {
        // rank/file moves
        [Piece::Rook, Piece::Queen].into_iter().for_each(|piece| {
            pos.board()
                .get_piece_bitboard(piece, pos.side_to_move())
                .iterator()
                .for_each(|from_sq| {
                    let rank_file_to_sq = self.hyperbola_quintessence(
                        pos,
                        pos.occupancy_masks()
                            .get_horizontal_mask(from_sq)
                            .into_u64(),
                        pos.occupancy_masks().get_vertical_mask(from_sq).into_u64(),
                        from_sq,
                    );
                    self.gen_capt_or_quiet_moves(pos, move_list, from_sq, rank_file_to_sq, piece);
                });
        });

        // diagonal/anti-diagonal moves
        [Piece::Bishop, Piece::Queen].into_iter().for_each(|piece| {
            pos.board()
                .get_piece_bitboard(piece, pos.side_to_move())
                .iterator()
                .for_each(|from_sq| {
                    let diag_to_sq = self.hyperbola_quintessence(
                        pos,
                        pos.occupancy_masks().get_diagonal_mask(from_sq).into_u64(),
                        pos.occupancy_masks()
                            .get_antidiagonal_mask(from_sq)
                            .into_u64(),
                        from_sq,
                    );
                    self.gen_capt_or_quiet_moves(pos, move_list, from_sq, diag_to_sq, piece);
                });
        });
    }

    #[inline(always)]
    fn gen_capt_or_quiet_moves(
        &self,
        pos: &Position,
        move_list: &mut MoveList,
        from_sq: Square,
        to_sq_bb: Bitboard,
        piece: Piece,
    ) {
        let opp_col_bb = pos.board().get_colour_bb(pos.side_to_move().flip_side());
        to_sq_bb.iterator().for_each(|to_sq| {
            let mv = if opp_col_bb.is_set(to_sq) {
                Move::encode_move_capture(from_sq, to_sq, piece)
            } else {
                Move::encode_move_quiet(from_sq, to_sq, piece)
            };
            move_list.push(mv);
        });
    }

    #[inline(always)]
    fn hyperbola_quintessence(
        &self,
        pos: &Position,
        dir_1_mask: u64,
        dir_2_mask: u64,
        square: Square,
    ) -> Bitboard {
        let all_bb = pos.board().get_bitboard().into_u64();
        let col_bb = pos.board().get_colour_bb(pos.side_to_move()).into_u64();
        let slider_bb = Bitboard::from_square(square).into_u64();

        let dir_1_a = (all_bb & dir_1_mask).wrapping_sub(slider_bb.wrapping_shl(1));
        let dir_1_b = ((all_bb & dir_1_mask)
            .reverse_bits()
            .wrapping_sub(slider_bb.reverse_bits().wrapping_shl(1)))
        .reverse_bits();
        let dir_1_moves = dir_1_a ^ dir_1_b;

        let dir_2_a = (all_bb & dir_2_mask).wrapping_sub(slider_bb.wrapping_shl(1));
        let dir_2_b = ((all_bb & dir_2_mask)
            .reverse_bits()
            .wrapping_sub(slider_bb.reverse_bits().wrapping_shl(1)))
        .reverse_bits();
        let dir_2_moves = dir_2_a ^ dir_2_b;

        let all_moves = (dir_1_moves & dir_1_mask) | (dir_2_moves & dir_2_mask);
        // return all moves excluding same colour pieces
        Bitboard::new(all_moves & !col_bb)
    }

    fn generate_non_sliding_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let opposite_side = pos.side_to_move().flip_side();
        let opp_occ_sq_bb = pos.board().get_colour_bb(opposite_side);
        let unoccupied_squares_bb = !pos.board().get_bitboard();

        [Piece::King, Piece::Knight].into_iter().for_each(|piece| {
            let pce_bb = pos.board().get_piece_bitboard(piece, pos.side_to_move());

            pce_bb.iterator().for_each(|from_sq| {
                let occ_mask = if piece == Piece::Knight {
                    pos.occupancy_masks().get_occupancy_mask_knight(from_sq)
                } else {
                    pos.occupancy_masks().get_occupancy_mask_king(from_sq)
                };

                // generate capture moves
                // AND'ing with opposite colour pieces with the occupancy mask, will
                // give all pieces that can be captured by the piece on this square
                (opp_occ_sq_bb & occ_mask).iterator().for_each(|to_sq| {
                    let mv = Move::encode_move_capture(from_sq, to_sq, piece);
                    move_list.push(mv);
                });

                // generate quiet moves
                let quiet_move_bb = unoccupied_squares_bb & occ_mask;
                quiet_move_bb.iterator().for_each(|to_sq| {
                    let mov = Move::encode_move_quiet(from_sq, to_sq, piece);
                    move_list.push(mov);
                });
            });
        })
    }

    fn generate_white_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if cp.is_white_king_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_WK).is_empty() {
            let mv = Move::encode_move_castle_kingside_white();
            move_list.push(mv);
        }
        if cp.is_white_queen_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_WQ).is_empty() {
            let mv = Move::encode_move_castle_queenside_white();
            move_list.push(mv);
        }
    }

    fn generate_black_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if cp.is_black_king_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_BK).is_empty() {
            let mv = Move::encode_move_castle_kingside_black();
            move_list.push(mv);
        }
        if cp.is_black_queen_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_BQ).is_empty() {
            let mv = Move::encode_move_castle_queenside_black();
            move_list.push(mv);
        }
    }

    fn encode_promotion_moves(&self, from_sq: Square, to_sq: Square, move_list: &mut MoveList) {
        for role in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
            move_list.push(Move::encode_move_with_promotion(from_sq, to_sq, role));
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
                let mv = Move::encode_move_with_promotion_capture(from_sq, to_sq, pce);
                move_list.push(mv);
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
        let mut mv = Move::encode_move_capture(Square::E3, Square::D1, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E3, Square::C2, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::A6, Square::B8, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::A6, Square::C7, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::G5, Square::H6, Piece::King);
        assert!(move_list.contains(mv));

        // check the quiet moves
        mv = Move::encode_move_quiet(Square::A6, Square::C5, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::F1, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::G2, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::G4, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::F5, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E3, Square::D5, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::G6, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::F6, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::F5, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::G4, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::H4, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G5, Square::H5, Piece::King);
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
        let mut mv = Move::encode_move_capture(Square::H1, Square::F2, Piece::Knight);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::D8, Square::E7, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::B8, Square::A6, Piece::Knight);
        assert!(move_list.contains(mv));

        // check the quiet moves
        mv = Move::encode_move_quiet(Square::D8, Square::C8, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D8, Square::E8, Piece::King);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::H1, Square::G3, Piece::Knight);
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
        let mut mv = Move::encode_move_quiet(Square::C4, Square::B5, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C4, Square::D5, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C4, Square::E6, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C4, Square::D3, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::D5, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::D3, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::F5, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::G6, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E4, Square::H7, Piece::Bishop);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::E4, Square::C2, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E4, Square::F3, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E4, Square::C6, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::C4, Square::E2, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::C4, Square::F7, Piece::Bishop);
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
        let mut mv = Move::encode_move_quiet(Square::D4, Square::C5, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D4, Square::B6, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D4, Square::E5, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::D4, Square::F6, Piece::Bishop);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::C8, Square::B7, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::D4, Square::C3, Piece::Bishop);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::D4, Square::E3, Piece::Bishop);
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
        let mut mv = Move::encode_move_quiet(Square::B1, Square::C1, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::D1, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::E1, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::F1, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B1, Square::B2, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::E1, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::E3, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::E4, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::D2, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::C2, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E2, Square::B2, Piece::Rook);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::B1, Square::A1, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E2, Square::F2, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E2, Square::A2, Piece::Rook);
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
        let mut mv = Move::encode_move_quiet(Square::B4, Square::A4, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B4, Square::B5, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::B4, Square::B6, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::D3, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::E3, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::C2, Piece::Rook);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::C3, Square::C1, Piece::Rook);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::C3, Square::F3, Piece::Rook);
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
        let mut mv = Move::encode_move_quiet(Square::E6, Square::E7, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::E8, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::D6, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::F6, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::G6, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::F5, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::E6, Square::G4, Piece::Queen);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::E6, Square::C6, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::H6, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::D7, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::F7, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::E6, Square::E5, Piece::Queen);
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
        let mut mv = Move::encode_move_quiet(Square::G1, Square::F1, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::E1, Piece::Queen);
        assert!(move_list.contains(mv));

        mv = Move::encode_move_quiet(Square::G1, Square::D1, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::C1, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::G2, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::G3, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_quiet(Square::G1, Square::G4, Piece::Queen);
        assert!(move_list.contains(mv));

        // check the capture moves
        mv = Move::encode_move_capture(Square::G1, Square::F2, Piece::Queen);
        assert!(move_list.contains(mv));
        mv = Move::encode_move_capture(Square::G1, Square::H2, Piece::Queen);
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

        let white_promotion_roles: [Piece; 4] =
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
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *role)));
        }

        from_sq = Square::B7;
        to_sq = Square::B8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *role)));
        }

        from_sq = Square::D7;
        to_sq = Square::D8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *role)));
        }

        from_sq = Square::H7;
        to_sq = Square::H8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *role)));
        }
        // CAPTURE promotion
        from_sq = Square::B7;
        to_sq = Square::C8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *role
            )));
        }
        from_sq = Square::D7;
        to_sq = Square::C8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *role,
            )));
        }

        from_sq = Square::D7;
        to_sq = Square::E8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *role,
            )));
        }

        from_sq = Square::H7;
        to_sq = Square::G8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *role,
            )));
        }
    }

    #[test]
    pub fn move_gen_black_promotion_moves_as_expected() {
        let fen = "2b1rkr1/PPpP1pbP/n6p/2NpPn2/1RBqBP2/4N1Q1/ppPpRp1P/B4K2 b - - 0 1";
        let black_promotion_roles: [Piece; 4] =
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
        for role in black_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *role)));
        }

        from_sq = Square::D2;
        to_sq = Square::D1;
        for role in black_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion(from_sq, to_sq, *role)));
        }

        // CAPTURE promotion
        from_sq = Square::B2;
        to_sq = Square::A1;
        for role in black_promotion_roles.iter() {
            assert!(move_list.contains(Move::encode_move_with_promotion_capture(
                from_sq, to_sq, *role,
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

        // single first move
        assert!(move_list.contains(Move::encode_move_quiet(Square::D2, Square::D3, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F2, Square::F3, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G2, Square::G3, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H2, Square::H3, Piece::Pawn)));

        // capture on first move
        assert!(move_list.contains(Move::encode_move_capture(
            Square::A2,
            Square::B3,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::D2,
            Square::E3,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::F2,
            Square::E3,
            Piece::Pawn,
        )));
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

        // single first move
        assert!(move_list.contains(Move::encode_move_quiet(Square::F7, Square::F6, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G7, Square::G6, Piece::Pawn)));

        // capture on first move
        assert!(move_list.contains(Move::encode_move_capture(
            Square::C7,
            Square::B6,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::C7,
            Square::D6,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::D7,
            Square::C6,
            Piece::Pawn,
        )));
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
        assert!(move_list.contains(Move::encode_move_quiet(Square::B4, Square::B5, Piece::Pawn,)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F5, Square::F6, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H4, Square::H5, Piece::Pawn)));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(
            Square::F5,
            Square::G6,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::G5,
            Square::H6,
            Piece::Pawn,
        )));

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
        assert!(move_list.contains(Move::encode_move_quiet(Square::A4, Square::A3, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E4, Square::E3, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::F3, Square::F2, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H4, Square::H3, Piece::Pawn)));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(
            Square::C5,
            Square::B4,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::C5,
            Square::D4,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::F3,
            Square::E2,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::F3,
            Square::G2,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::H4,
            Square::G3,
            Piece::Pawn,
        )));

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
        assert!(move_list.contains(Move::encode_move_quiet(Square::A1, Square::A2, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::A1, Square::B1, Piece::Rook)));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C1,
            Square::B2,
            Piece::Bishop
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C1,
            Square::D2,
            Piece::Bishop
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C1,
            Square::E3,
            Piece::Bishop
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C1,
            Square::F4,
            Piece::Bishop
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C1,
            Square::G5,
            Piece::Bishop
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C1,
            Square::H6,
            Piece::Bishop
        )));

        assert!(move_list.contains(Move::encode_move_quiet(Square::E1, Square::D1, Piece::King)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E1, Square::D2, Piece::King)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E1, Square::F1, Piece::King)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::H1, Square::G1, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H1, Square::F1, Piece::Rook)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::A3, Square::A4, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::B3, Square::B4, Piece::Pawn)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::C2, Square::C3, Piece::Pawn)));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::E2,
            Square::C3,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::E2,
            Square::G1,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::E2,
            Square::G3,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::E2,
            Square::F4,
            Piece::Knight
        )));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F2,
            Square::E3,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F2,
            Square::G1,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F2,
            Square::G3,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F2,
            Square::H4,
            Piece::Queen
        )));

        assert!(move_list.contains(Move::encode_move_quiet(Square::F3, Square::F4, Piece::Pawn)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::G2, Square::G3, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::H2, Square::H3, Piece::Pawn)));

        // castle move
        assert!(move_list.contains(Move::encode_move_castle_kingside_white()));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(
            Square::E2,
            Square::D4,
            Piece::Knight,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::F2,
            Square::D4,
            Piece::Queen,
        )));

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
        assert!(move_list.contains(Move::encode_move_quiet(Square::A7, Square::A6, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::B6, Square::B5, Piece::Pawn)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D4, Square::D3, Piece::Pawn)));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C6,
            Square::B8,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C6,
            Square::E7,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C6,
            Square::E5,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::C6,
            Square::A5,
            Piece::Knight
        )));

        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::D7, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::D6, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::D5, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::C8, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::B8, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::D8, Square::A8, Piece::Rook)));

        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::F8, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E7, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E6, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E5, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E4, Piece::Rook)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::E8, Square::E3, Piece::Rook)));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F6,
            Square::D7,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F6,
            Square::D5,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F6,
            Square::E4,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F6,
            Square::G4,
            Piece::Knight
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::F6,
            Square::H7,
            Piece::Knight
        )));

        assert!(move_list.contains(Move::encode_move_quiet(Square::G6, Square::G5, Piece::Pawn)));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::H6,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::H7,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::H8,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::H4,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::H3,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::G4,
            Piece::Queen
        )));

        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::G5,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::F5,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::E5,
            Piece::Queen
        )));
        assert!(move_list.contains(Move::encode_move_quiet(
            Square::H5,
            Square::D5,
            Piece::Queen
        )));

        assert!(move_list.contains(Move::encode_move_quiet(Square::G8, Square::F8, Piece::King)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G8, Square::H8, Piece::King)));
        assert!(move_list.contains(Move::encode_move_quiet(Square::G8, Square::H7, Piece::King)));

        // capture moves
        assert!(move_list.contains(Move::encode_move_capture(
            Square::B6,
            Square::C5,
            Piece::Pawn,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::C6,
            Square::B4,
            Piece::Knight,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::E8,
            Square::E2,
            Piece::Rook,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::H5,
            Square::H2,
            Piece::Queen,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::H5,
            Square::F3,
            Piece::Queen,
        )));
        assert!(move_list.contains(Move::encode_move_capture(
            Square::H5,
            Square::C5,
            Piece::Queen,
        )));

        // double pawn first move
        assert!(move_list.contains(Move::encode_move_double_pawn_first(Square::A7, Square::A5)));
    }
}
