use super::game::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}

impl Color {
    pub fn opposite(&self) -> Color {
        if *self == Color::Black {
            return Color::White;
        } else {
            return Color::Black;
        }
    }
}
