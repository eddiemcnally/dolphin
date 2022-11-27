use crate::board::colour::Colour;
use crate::moves::mov::Score;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Material {
    score: [Score; Colour::NUM_COLOURS],
}

impl Material {
    pub fn new(white: Score, black: Score) -> Material {
        let mut met = Material::default();
        met.score[Colour::White.to_offset()] = white;
        met.score[Colour::Black.to_offset()] = black;
        met
    }

    pub fn get_black(&self) -> Score {
        self.score[Colour::Black.to_offset()]
    }
    pub fn get_white(&self) -> Score {
        self.score[Colour::White.to_offset()]
    }

    pub fn get_material_for_colour(&self, colour: Colour) -> Score {
        self.score[colour.to_offset()]
    }

    pub fn set_material_for_colour(&mut self, colour: Colour, score: Score) {
        self.score[colour.to_offset()] = score;
    }

    pub fn get_net_material(&self) -> Score {
        self.get_white().wrapping_sub(self.get_black()) as Score
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

    use crate::board::colour::Colour;
    use crate::board::game_board::Board;
    use crate::board::material::Material;
    use crate::board::piece::Piece;
    use crate::board::square::*;
    use crate::moves::mov::Score;

    #[test]
    pub fn add_remove_white_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = Piece::Bishop;
        let pce2 = Piece::Queen;

        board.add_piece(pce1, Colour::White, Square::A1);
        board.add_piece(pce2, Colour::White, Square::D3);
        let material_after_add = Material::new(pce1.value() + pce2.value(), 0);

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(pce1, Colour::White, Square::A1);

        let material_after_remove = pce2.value();

        assert_eq!(
            material_after_remove,
            board.get_material().get_net_material() as Score
        );
    }

    #[test]
    pub fn add_remove_black_pieces_material_as_expected() {
        let mut board = Board::new();

        let pce1 = Piece::Bishop;
        let pce2 = Piece::Queen;

        board.add_piece(pce1, Colour::Black, Square::A1);
        board.add_piece(pce2, Colour::Black, Square::D3);
        let material_after_add = Material::new(0, pce1.value() + pce2.value());

        assert_eq!(material_after_add, board.get_material());

        board.remove_piece(pce1, Colour::Black, Square::A1);

        let material_after_remove = Material::new(0, pce2.value());

        assert_eq!(material_after_remove, board.get_material());
    }

    #[test]
    pub fn move_white_piece_material_unchanged() {
        let pce = Piece::Knight;
        let from_sq = Square::D4;
        let to_sq = Square::C6;

        let mut board = Board::new();

        board.add_piece(pce, Colour::White, from_sq);
        let start_material = board.get_material();

        let expected_start_material = Material::new(pce.value(), 0);

        assert_eq!(start_material, expected_start_material);

        board.move_piece(from_sq, to_sq, pce, Colour::White);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }

    #[test]
    pub fn move_black_piece_material_unchanged() {
        let pce = Piece::Knight;
        let from_sq = Square::D4;
        let to_sq = Square::C6;

        let mut board = Board::new();

        board.add_piece(pce, Colour::Black, Square::D4);
        let start_material = board.get_material();

        let expected_start_material = Material::new(0, pce.value());

        assert_eq!(start_material, expected_start_material);

        board.move_piece(from_sq, to_sq, pce, Colour::Black);
        let end_material = board.get_material();

        assert_eq!(start_material, end_material);
    }
}
