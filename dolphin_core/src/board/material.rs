use crate::board::colour;
use crate::board::colour::Colour;
use std::fmt;

use super::types::ToInt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Material {
    score: [u32; colour::NUM_COLOURS],
}

impl Material {
    pub fn new(white: u32, black: u32) -> Material {
        let mut met = Material::default();
        met.score[colour::offset(Colour::White)] = white;
        met.score[colour::offset(Colour::Black)] = black;
        met
    }

    pub const fn get_black(&self) -> u32 {
        self.score[colour::offset(Colour::Black)]
    }
    pub const fn get_white(&self) -> u32 {
        self.score[colour::offset(Colour::White)]
    }

    pub fn get_material_for_colour(&self, colour: Colour) -> u32 {
        self.score[colour.to_usize()]
    }

    pub fn set_material_for_colour(&mut self, colour: Colour, score: u32) {
        self.score[colour.to_usize()] = score;
    }

    pub const fn get_net_material(&self) -> i32 {
        self.get_white().wrapping_sub(self.get_black()) as i32
    }
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&("White: ".to_owned() + &self.get_white().to_string()));
        debug_str.push_str(&("Black: ".to_owned() + &self.get_black().to_string()));

        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
pub mod tests {

    use crate::board::game_board::Board;
    use crate::board::material::Material;
    use crate::board::piece;
    use crate::board::square::*;

    #[test]
    pub fn add_remove_white_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = &piece::WHITE_BISHOP;
        let pce2 = &piece::WHITE_QUEEN;

        board.add_piece(pce1, SQUARE_A1);
        board.add_piece(pce2, SQUARE_D3);
        let material_after_add = Material::new(pce1.value() + pce2.value(), 0);

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(pce1, SQUARE_A1);

        let material_after_remove = pce2.value();

        assert_eq!(
            material_after_remove,
            board.get_material().get_net_material() as u32
        );
    }

    #[test]
    pub fn add_remove_black_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = &piece::BLACK_BISHOP;
        let pce2 = &piece::BLACK_QUEEN;

        board.add_piece(pce1, SQUARE_A1);
        board.add_piece(pce2, SQUARE_D3);
        let material_after_add = Material::new(0, pce1.value() + pce2.value());

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(pce1, SQUARE_A1);

        let material_after_remove = Material::new(0, pce2.value());

        assert_eq!(material_after_remove, board.get_material());
    }

    #[test]
    pub fn move_white_piece_material_unchanged() {
        let pce = &piece::WHITE_KNIGHT;
        let from_sq = SQUARE_D4;
        let to_sq = SQUARE_C6;

        let mut board = Board::new();

        board.add_piece(pce, from_sq);
        let start_material = board.get_material();

        let expected_start_material = Material::new(pce.value(), 0);

        assert_eq!(start_material, expected_start_material);

        board.move_piece(from_sq, to_sq, pce);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }

    #[test]
    pub fn move_black_piece_material_unchanged() {
        let pce = &piece::BLACK_KNIGHT;
        let from_sq = SQUARE_D4;
        let to_sq = SQUARE_C6;

        let mut board = Board::new();

        board.add_piece(pce, SQUARE_D4);
        let start_material = board.get_material();

        let expected_start_material = Material::new(0, pce.value());

        assert_eq!(start_material, expected_start_material);

        board.move_piece(from_sq, to_sq, pce);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }
}