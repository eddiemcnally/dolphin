use dolphin_core::{
    board::occupancy_masks::OccupancyMasks,
    io::fen,
    position::{attack_checker::AttackChecker, game_position::Position, zobrist_keys::ZobristKeys},
    search_engine::search::Search,
};

fn main() {
    let fen = "2kr4/8/8/1p6/1Kn5/1P1q4/P7/8 w - - 0 1";

    let (board, move_cntr, castle_permissions, side_to_move, en_pass_sq) = fen::decompose_fen(fen);

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

    let mut search = Search::new(10000000000, 6);
    search.search(&mut pos);
}
