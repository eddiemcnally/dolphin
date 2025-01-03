use crate::board::bitboard::Bitboard;
use crate::board::colour::Colour;
use crate::board::occupancy_masks::OccupancyMasks;
use crate::board::piece::Piece;
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

        match pos.side_to_move() {
            Colour::White => {
                self.generate_white_pawn_normal_moves(pos, move_list);
                self.gen_white_pawn_promotion_moves(pos, move_list);
                self.generate_white_en_passant_moves(pos, move_list);
                self.generate_white_castle_moves(pos, move_list);
            }
            Colour::Black => {
                self.generate_black_pawn_normal_moves(pos, move_list);
                self.gen_black_pawn_promotion_moves(pos, move_list);
                self.generate_black_en_passant_moves(pos, move_list);
                self.generate_black_castle_moves(pos, move_list);
            }
        }

        self.generate_non_sliding_moves(pos, move_list);
        self.generate_sliding_moves(pos, move_list);

        let move_cnt_end = move_list.len();

        (move_cnt_end - move_cnt_start) as u16
    }

    fn generate_white_pawn_normal_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let wp_bb = pos.board().get_piece_bitboard(&Piece::Pawn, &Colour::White);
        let opposite_bb = pos.board().get_colour_bb(&Colour::Black);
        let empty_bb = !pos.board().get_bitboard();

        // quiet moves
        let wp_r2_6_bb = wp_bb & OccupancyMasks::RANK_2_TO_6_BB;
        let quiet_pawns_bb = (wp_r2_6_bb.north() & empty_bb).south();

        quiet_pawns_bb.iterator().for_each(|from_sq| {
            let mv = Move::encode_move(&from_sq, &from_sq.north().unwrap());
            move_list.push(&mv);
        });

        // double pawn push
        let wp_r2_bb = wp_bb & OccupancyMasks::RANK_2_BB;
        if !wp_r2_bb.is_empty() {
            let north_bb = wp_r2_bb.north() & empty_bb;
            let north_north_bb = north_bb.north() & empty_bb;

            let double_pawn_bb = north_north_bb.south().south();
            double_pawn_bb.iterator().for_each(|from_sq| {
                let mv = Move::encode_move(&from_sq, &from_sq.north().unwrap().north().unwrap());
                move_list.push(&mv);
            });
        }

        // capture
        let wp_r2_6_bb = wp_bb & OccupancyMasks::RANK_2_TO_6_BB;
        let bb_ne = (wp_r2_6_bb.north_east() & opposite_bb).south_west();
        bb_ne.iterator().for_each(|from_sq| {
            let mv = Move::encode_move(&from_sq, &from_sq.north_east().unwrap());
            move_list.push(&mv);
        });
        let bb_nw = (wp_r2_6_bb.north_west() & opposite_bb).south_east();
        bb_nw.iterator().for_each(|from_sq| {
            let mv = Move::encode_move(&from_sq, &from_sq.north_west().unwrap());
            move_list.push(&mv);
        });
    }

    fn generate_white_en_passant_moves(&self, pos: &Position, move_list: &mut MoveList) {
        if let Some(en_sq) = pos.en_passant_square() {
            let wp_bb = pos.board().get_piece_bitboard(&Piece::Pawn, &Colour::White);

            // check south-east
            if let Some(se_sq) = en_sq.south_east() {
                if wp_bb.is_set(&se_sq) {
                    let en_pass_mv = Move::encode_move_en_passant(&se_sq, &en_sq);
                    move_list.push(&en_pass_mv);
                }
            }
            // check south-west
            if let Some(sw_sq) = en_sq.south_west() {
                if wp_bb.is_set(&sw_sq) {
                    let en_pass_mv = Move::encode_move_en_passant(&sw_sq, &en_sq);
                    move_list.push(&en_pass_mv);
                }
            }
        }
    }

    fn gen_white_pawn_promotion_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let wp_bb = pos.board().get_piece_bitboard(&Piece::Pawn, &Colour::White)
            & OccupancyMasks::RANK_7_BB;

        if !wp_bb.is_empty() {
            let empty_bb = !pos.board().get_bitboard();

            // quiet promotion
            let promo_bb = (wp_bb.north() & empty_bb).south();
            promo_bb.iterator().for_each(|from_sq| {
                self.encode_promotion_moves(&from_sq, &from_sq.north().unwrap(), move_list);
            });

            // capture promotion
            let opposite_bb = pos.board().get_colour_bb(&Colour::Black);
            let bb_ne = (wp_bb.north_east() & opposite_bb).south_west();
            bb_ne.iterator().for_each(|from_sq| {
                self.encode_promotion_moves(&from_sq, &from_sq.north_east().unwrap(), move_list);
            });

            let bb_nw = (wp_bb.north_west() & opposite_bb).south_east();
            bb_nw.iterator().for_each(|from_sq| {
                self.encode_promotion_moves(&from_sq, &from_sq.north_west().unwrap(), move_list);
            });
        }
    }

    fn generate_white_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if cp.is_white_king_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_WK).is_empty() {
            let mv = Move::encode_move_castle_kingside_white();
            move_list.push(&mv);
        }
        if cp.is_white_queen_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_WQ).is_empty() {
            let mv = Move::encode_move_castle_queenside_white();
            move_list.push(&mv);
        }
    }

    fn generate_black_pawn_normal_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let bp_bb = pos.board().get_piece_bitboard(&Piece::Pawn, &Colour::Black);
        let empty_bb = !pos.board().get_bitboard();
        let opposite_bb = pos.board().get_colour_bb(&Colour::White);

        // quiet moves
        let bp_r3_7_bb = bp_bb & OccupancyMasks::RANK_3_TO_7_BB;
        let quiet_pawns_bb = (bp_r3_7_bb.south() & empty_bb).north();

        quiet_pawns_bb.iterator().for_each(|from_sq| {
            let mv = Move::encode_move(&from_sq, &from_sq.south().unwrap());
            move_list.push(&mv);
        });

        // double pawn push
        let bp_r7_bb = bp_bb & OccupancyMasks::RANK_7_BB;
        if !bp_r7_bb.is_empty() {
            let south_bb = bp_r7_bb.south() & empty_bb;
            let south_south_bb = south_bb.south() & empty_bb;

            let double_pawn_bb = south_south_bb.north().north();
            double_pawn_bb.iterator().for_each(|from_sq| {
                let mv = Move::encode_move(&from_sq, &from_sq.south().unwrap().south().unwrap());
                move_list.push(&mv);
            });
        }

        // capture
        let bp_r3_7_bb = bp_bb & OccupancyMasks::RANK_3_TO_7_BB;
        let bb_se = (bp_r3_7_bb.south_east() & opposite_bb).north_west();
        bb_se.iterator().for_each(|from_sq| {
            let mv = Move::encode_move(&from_sq, &from_sq.south_east().unwrap());
            move_list.push(&mv);
        });

        let bb_sw = (bp_r3_7_bb.south_west() & opposite_bb).north_east();
        bb_sw.iterator().for_each(|from_sq| {
            let mv = Move::encode_move(&from_sq, &from_sq.south_west().unwrap());
            move_list.push(&mv);
        });
    }

    fn generate_black_en_passant_moves(&self, pos: &Position, move_list: &mut MoveList) {
        if let Some(en_sq) = pos.en_passant_square() {
            let bp_bb = pos.board().get_piece_bitboard(&Piece::Pawn, &Colour::Black);

            // check north-east
            if let Some(ne_sq) = en_sq.north_east() {
                if bp_bb.is_set(&ne_sq) {
                    let en_pass_mv = Move::encode_move_en_passant(&ne_sq, &en_sq);
                    move_list.push(&en_pass_mv);
                }
            }
            // check north-west
            if let Some(nw_sq) = en_sq.north_west() {
                if bp_bb.is_set(&nw_sq) {
                    let en_pass_mv = Move::encode_move_en_passant(&nw_sq, &en_sq);
                    move_list.push(&en_pass_mv);
                }
            }
        }
    }

    fn gen_black_pawn_promotion_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let bp_bb = pos.board().get_piece_bitboard(&Piece::Pawn, &Colour::Black)
            & OccupancyMasks::RANK_2_BB;

        if !bp_bb.is_empty() {
            let empty_bb = !pos.board().get_bitboard();

            // quiet promotion
            let promo_bb = (bp_bb.south() & empty_bb).north();
            promo_bb.iterator().for_each(|from_sq| {
                self.encode_promotion_moves(&from_sq, &from_sq.south().unwrap(), move_list);
            });

            // capture promotion
            let opposite_bb = pos.board().get_colour_bb(&Colour::White);
            let bb_se = (bp_bb.south_east() & opposite_bb).north_west();
            bb_se.iterator().for_each(|from_sq| {
                self.encode_promotion_moves(&from_sq, &from_sq.south_east().unwrap(), move_list);
            });

            let bb_sw = (bp_bb.south_west() & opposite_bb).north_east();
            bb_sw.iterator().for_each(|from_sq| {
                self.encode_promotion_moves(&from_sq, &from_sq.south_west().unwrap(), move_list);
            });
        }
    }

    fn generate_black_castle_moves(&self, pos: &Position, move_list: &mut MoveList) {
        let cp = pos.castle_permissions();
        let bb = pos.board().get_bitboard();

        if cp.is_black_king_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_BK).is_empty() {
            let mv = Move::encode_move_castle_kingside_black();
            move_list.push(&mv);
        }
        if cp.is_black_queen_set() && (bb & OccupancyMasks::CASTLE_MASK_FREE_SQ_BQ).is_empty() {
            let mv = Move::encode_move_castle_queenside_black();
            move_list.push(&mv);
        }
    }

    fn generate_sliding_moves(&self, pos: &Position, move_list: &mut MoveList) {
        // rank/file moves
        [Piece::Rook, Piece::Queen].into_iter().for_each(|piece| {
            pos.board()
                .get_piece_bitboard(&piece, &pos.side_to_move())
                .iterator()
                .for_each(|from_sq| {
                    let rank_file_to_sq = self.hyperbola_quintessence(
                        pos,
                        pos.occupancy_masks()
                            .get_horizontal_mask(&from_sq)
                            .into_u64(),
                        pos.occupancy_masks().get_vertical_mask(&from_sq).into_u64(),
                        &from_sq,
                    );
                    self.gen_multiple_moves(move_list, &from_sq, &rank_file_to_sq);
                });
        });

        // diagonal/anti-diagonal moves
        [Piece::Bishop, Piece::Queen].into_iter().for_each(|piece| {
            pos.board()
                .get_piece_bitboard(&piece, &pos.side_to_move())
                .iterator()
                .for_each(|from_sq| {
                    let diag_to_sq = self.hyperbola_quintessence(
                        pos,
                        pos.occupancy_masks().get_diagonal_mask(&from_sq).into_u64(),
                        pos.occupancy_masks()
                            .get_antidiagonal_mask(&from_sq)
                            .into_u64(),
                        &from_sq,
                    );
                    self.gen_multiple_moves(move_list, &from_sq, &diag_to_sq);
                });
        });
    }

    fn gen_multiple_moves(&self, move_list: &mut MoveList, from_sq: &Square, to_sq_bb: &Bitboard) {
        to_sq_bb.iterator().for_each(|to_sq| {
            let mv = Move::encode_move(&from_sq, &to_sq);
            move_list.push(&mv);
        });
    }

    fn hyperbola_quintessence(
        &self,
        pos: &Position,
        dir_1_mask: u64,
        dir_2_mask: u64,
        square: &Square,
    ) -> Bitboard {
        let all_bb = pos.board().get_bitboard().into_u64();
        let col_bb = pos.board().get_colour_bb(&pos.side_to_move()).into_u64();
        let slider_bb = Bitboard::from_square(&square).into_u64();

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
        let opp_occ_sq_bb = pos.board().get_colour_bb(&opposite_side);
        let unoccupied_squares_bb = !pos.board().get_bitboard();

        [Piece::King, Piece::Knight].into_iter().for_each(|piece| {
            let pce_bb = pos.board().get_piece_bitboard(&piece, &pos.side_to_move());

            pce_bb.iterator().for_each(|from_sq| {
                let occ_mask = if piece == Piece::Knight {
                    pos.occupancy_masks().get_occupancy_mask_knight(&from_sq)
                } else {
                    pos.occupancy_masks().get_occupancy_mask_king(&from_sq)
                };

                // generate capture moves
                // AND'ing with opposite colour pieces with the occupancy mask, will
                // give all pieces that can be captured by the piece on this square
                (opp_occ_sq_bb & occ_mask).iterator().for_each(|to_sq| {
                    let mv = Move::encode_move(&from_sq, &to_sq);
                    move_list.push(&mv);
                });

                // generate quiet moves
                let quiet_move_bb = unoccupied_squares_bb & occ_mask;
                quiet_move_bb.iterator().for_each(|to_sq| {
                    let mov = Move::encode_move(&from_sq, &to_sq);
                    move_list.push(&mov);
                });
            });
        })
    }

    fn encode_promotion_moves(&self, from_sq: &Square, to_sq: &Square, move_list: &mut MoveList) {
        for role in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
            move_list.push(&Move::encode_move_with_promotion(&from_sq, &to_sq, &role));
        }
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
        let mut mv = Move::encode_move(&Square::E3, &Square::D1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E3, &Square::C2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::A6, &Square::B8);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::A6, &Square::C7);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::H6);
        assert!(move_list.contains(&mv));

        // check the quiet moves
        mv = Move::encode_move(&Square::A6, &Square::C5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E3, &Square::F1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E3, &Square::G2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E3, &Square::G4);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E3, &Square::F5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E3, &Square::D5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::G6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::F6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::F5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::G4);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::H4);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G5, &Square::H5);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::H1, &Square::F2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D8, &Square::E7);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B8, &Square::A6);
        assert!(move_list.contains(&mv));

        // check the quiet moves
        mv = Move::encode_move(&Square::D8, &Square::C8);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D8, &Square::E8);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::H1, &Square::G3);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::C4, &Square::B5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C4, &Square::D5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C4, &Square::E6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C4, &Square::D3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::D5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::D3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::F5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::G6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::H7);
        assert!(move_list.contains(&mv));

        // check the capture moves
        mv = Move::encode_move(&Square::E4, &Square::C2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::F3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E4, &Square::C6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C4, &Square::E2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C4, &Square::F7);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::D4, &Square::C5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D4, &Square::B6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D4, &Square::E5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D4, &Square::F6);
        assert!(move_list.contains(&mv));

        // check the capture moves
        mv = Move::encode_move(&Square::C8, &Square::B7);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D4, &Square::C3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::D4, &Square::E3);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::B1, &Square::C1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B1, &Square::D1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B1, &Square::E1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B1, &Square::F1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B1, &Square::B2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::E1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::E3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::E4);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::D2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::C2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::B2);
        assert!(move_list.contains(&mv));

        // check the capture moves
        mv = Move::encode_move(&Square::B1, &Square::A1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::F2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E2, &Square::A2);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::B4, &Square::A4);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B4, &Square::B5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::B4, &Square::B6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C3, &Square::D3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C3, &Square::E3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C3, &Square::C2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::C3, &Square::C1);
        assert!(move_list.contains(&mv));

        // check the capture moves
        mv = Move::encode_move(&Square::C3, &Square::F3);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::E6, &Square::E7);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::E8);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::D6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::F6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::G6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::F5);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::G4);
        assert!(move_list.contains(&mv));

        // check the capture moves
        mv = Move::encode_move(&Square::E6, &Square::C6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::H6);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::D7);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::F7);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::E6, &Square::E5);
        assert!(move_list.contains(&mv));
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
        let mut mv = Move::encode_move(&Square::G1, &Square::F1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G1, &Square::E1);
        assert!(move_list.contains(&mv));

        mv = Move::encode_move(&Square::G1, &Square::D1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G1, &Square::C1);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G1, &Square::G2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G1, &Square::G3);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G1, &Square::G4);
        assert!(move_list.contains(&mv));

        // check the capture moves
        mv = Move::encode_move(&Square::G1, &Square::F2);
        assert!(move_list.contains(&mv));
        mv = Move::encode_move(&Square::G1, &Square::H2);
        assert!(move_list.contains(&mv));
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
        assert!(move_list.contains(&mv));
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
        assert!(move_list.contains(&mv));
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
        assert!(move_list.contains(&mv));
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
        assert!(move_list.contains(&mv));
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
        assert!(move_list.contains(&mv));

        mv = Move::encode_move_castle_kingside_white();
        assert!(move_list.contains(&mv));

        // --- BLACK
        pos.flip_side_to_move();

        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&pos, &mut move_list);

        mv = Move::encode_move_castle_queenside_black();
        assert!(move_list.contains(&mv));

        mv = Move::encode_move_castle_kingside_black();
        assert!(move_list.contains(&mv));
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
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }

        from_sq = Square::B7;
        to_sq = Square::B8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }

        from_sq = Square::D7;
        to_sq = Square::D8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }

        from_sq = Square::H7;
        to_sq = Square::H8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }
        // CAPTURE promotion
        from_sq = Square::B7;
        to_sq = Square::C8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }
        from_sq = Square::D7;
        to_sq = Square::C8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role,)));
        }

        from_sq = Square::D7;
        to_sq = Square::E8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role,)));
        }

        from_sq = Square::H7;
        to_sq = Square::G8;
        for role in white_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role,)));
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
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }

        from_sq = Square::D2;
        to_sq = Square::D1;
        for role in black_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role)));
        }

        // CAPTURE promotion
        from_sq = Square::B2;
        to_sq = Square::A1;
        for role in black_promotion_roles.iter() {
            assert!(move_list.contains(&Move::encode_move_with_promotion(&from_sq, &to_sq, role,)));
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
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::F4)));
        assert!(move_list.contains(&Move::encode_move(&Square::G2, &Square::G4)));

        // single first move
        assert!(move_list.contains(&Move::encode_move(&Square::D2, &Square::D3)));
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::F3)));
        assert!(move_list.contains(&Move::encode_move(&Square::G2, &Square::G3)));
        assert!(move_list.contains(&Move::encode_move(&Square::H2, &Square::H3)));

        // capture on first move
        assert!(move_list.contains(&Move::encode_move(&Square::A2, &Square::B3)));
        assert!(move_list.contains(&Move::encode_move(&Square::D2, &Square::E3)));
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::E3)));
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
        assert!(move_list.contains(&Move::encode_move(&Square::F7, &Square::F5)));

        // single first move
        assert!(move_list.contains(&Move::encode_move(&Square::F7, &Square::F6)));
        assert!(move_list.contains(&Move::encode_move(&Square::G7, &Square::G6)));

        // capture on first move
        assert!(move_list.contains(&Move::encode_move(&Square::C7, &Square::B6)));
        assert!(move_list.contains(&Move::encode_move(&Square::C7, &Square::D6)));
        assert!(move_list.contains(&Move::encode_move(&Square::D7, &Square::C6)));
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
        assert!(move_list.contains(&Move::encode_move(&Square::B4, &Square::B5)));
        assert!(move_list.contains(&Move::encode_move(&Square::F5, &Square::F6)));
        assert!(move_list.contains(&Move::encode_move(&Square::H4, &Square::H5)));

        // capture moves
        assert!(move_list.contains(&Move::encode_move(&Square::F5, &Square::G6)));
        assert!(move_list.contains(&Move::encode_move(&Square::G5, &Square::H6)));

        // en passant move
        assert!(move_list.contains(&Move::encode_move_en_passant(&Square::E5, &Square::D6)));
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
        assert!(move_list.contains(&Move::encode_move(&Square::A4, &Square::A3)));
        assert!(move_list.contains(&Move::encode_move(&Square::E4, &Square::E3)));
        assert!(move_list.contains(&Move::encode_move(&Square::F3, &Square::F2)));
        assert!(move_list.contains(&Move::encode_move(&Square::H4, &Square::H3)));

        // capture moves
        assert!(move_list.contains(&Move::encode_move(&Square::C5, &Square::B4)));
        assert!(move_list.contains(&Move::encode_move(&Square::C5, &Square::D4)));
        assert!(move_list.contains(&Move::encode_move(&Square::F3, &Square::E2)));
        assert!(move_list.contains(&Move::encode_move(&Square::F3, &Square::G2)));
        assert!(move_list.contains(&Move::encode_move(&Square::H4, &Square::G3)));

        // en passant move
        assert!(move_list.contains(&Move::encode_move_en_passant(&Square::A4, &Square::B3)));
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
        assert!(move_list.contains(&Move::encode_move(&Square::A1, &Square::A2)));
        assert!(move_list.contains(&Move::encode_move(&Square::A1, &Square::B1)));

        assert!(move_list.contains(&Move::encode_move(&Square::C1, &Square::B2)));
        assert!(move_list.contains(&Move::encode_move(&Square::C1, &Square::D2)));
        assert!(move_list.contains(&Move::encode_move(&Square::C1, &Square::E3)));
        assert!(move_list.contains(&Move::encode_move(&Square::C1, &Square::F4)));
        assert!(move_list.contains(&Move::encode_move(&Square::C1, &Square::G5)));
        assert!(move_list.contains(&Move::encode_move(&Square::C1, &Square::H6)));

        assert!(move_list.contains(&Move::encode_move(&Square::E1, &Square::D1)));
        assert!(move_list.contains(&Move::encode_move(&Square::E1, &Square::D2)));
        assert!(move_list.contains(&Move::encode_move(&Square::E1, &Square::F1)));

        assert!(move_list.contains(&Move::encode_move(&Square::H1, &Square::G1)));
        assert!(move_list.contains(&Move::encode_move(&Square::H1, &Square::F1)));

        assert!(move_list.contains(&Move::encode_move(&Square::A3, &Square::A4)));
        assert!(move_list.contains(&Move::encode_move(&Square::B3, &Square::B4)));

        assert!(move_list.contains(&Move::encode_move(&Square::C2, &Square::C3)));

        assert!(move_list.contains(&Move::encode_move(&Square::E2, &Square::C3)));
        assert!(move_list.contains(&Move::encode_move(&Square::E2, &Square::G1)));
        assert!(move_list.contains(&Move::encode_move(&Square::E2, &Square::G3)));
        assert!(move_list.contains(&Move::encode_move(&Square::E2, &Square::F4)));

        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::E3)));
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::G1)));
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::G3)));
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::H4)));

        assert!(move_list.contains(&Move::encode_move(&Square::F3, &Square::F4)));

        assert!(move_list.contains(&Move::encode_move(&Square::G2, &Square::G3)));
        assert!(move_list.contains(&Move::encode_move(&Square::H2, &Square::H3)));

        // castle move
        assert!(move_list.contains(&Move::encode_move_castle_kingside_white()));

        // capture moves
        assert!(move_list.contains(&Move::encode_move(&Square::E2, &Square::D4)));
        assert!(move_list.contains(&Move::encode_move(&Square::F2, &Square::D4)));

        // double pawn first move
        assert!(move_list.contains(&Move::encode_move(&Square::C2, &Square::C4)));
        assert!(move_list.contains(&Move::encode_move(&Square::G2, &Square::G4)));
        assert!(move_list.contains(&Move::encode_move(&Square::H2, &Square::H4)));
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
        assert!(move_list.contains(&Move::encode_move(&Square::A7, &Square::A6)));
        assert!(move_list.contains(&Move::encode_move(&Square::B6, &Square::B5)));
        assert!(move_list.contains(&Move::encode_move(&Square::D4, &Square::D3)));

        assert!(move_list.contains(&Move::encode_move(&Square::C6, &Square::B8)));
        assert!(move_list.contains(&Move::encode_move(&Square::C6, &Square::E7)));
        assert!(move_list.contains(&Move::encode_move(&Square::C6, &Square::E5)));
        assert!(move_list.contains(&Move::encode_move(&Square::C6, &Square::A5)));

        assert!(move_list.contains(&Move::encode_move(&Square::D8, &Square::D7)));
        assert!(move_list.contains(&Move::encode_move(&Square::D8, &Square::D6)));
        assert!(move_list.contains(&Move::encode_move(&Square::D8, &Square::D5)));
        assert!(move_list.contains(&Move::encode_move(&Square::D8, &Square::C8)));
        assert!(move_list.contains(&Move::encode_move(&Square::D8, &Square::B8)));
        assert!(move_list.contains(&Move::encode_move(&Square::D8, &Square::A8)));

        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::F8)));
        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::E7)));
        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::E6)));
        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::E5)));
        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::E4)));
        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::E3)));

        assert!(move_list.contains(&Move::encode_move(&Square::F6, &Square::D7)));
        assert!(move_list.contains(&Move::encode_move(&Square::F6, &Square::D5)));
        assert!(move_list.contains(&Move::encode_move(&Square::F6, &Square::E4)));
        assert!(move_list.contains(&Move::encode_move(&Square::F6, &Square::G4)));
        assert!(move_list.contains(&Move::encode_move(&Square::F6, &Square::H7)));

        assert!(move_list.contains(&Move::encode_move(&Square::G6, &Square::G5)));

        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::H6)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::H7)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::H8)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::H4)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::H3)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::G4)));

        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::G5)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::F5)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::E5)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::D5)));

        assert!(move_list.contains(&Move::encode_move(&Square::G8, &Square::F8)));
        assert!(move_list.contains(&Move::encode_move(&Square::G8, &Square::H8)));
        assert!(move_list.contains(&Move::encode_move(&Square::G8, &Square::H7)));

        // capture moves
        assert!(move_list.contains(&Move::encode_move(&Square::B6, &Square::C5)));
        assert!(move_list.contains(&Move::encode_move(&Square::C6, &Square::B4)));
        assert!(move_list.contains(&Move::encode_move(&Square::E8, &Square::E2)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::H2)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::F3)));
        assert!(move_list.contains(&Move::encode_move(&Square::H5, &Square::C5)));

        // double pawn first move
        assert!(move_list.contains(&Move::encode_move(&Square::A7, &Square::A5)));
    }
}
