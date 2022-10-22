use crate::Piece;
use crate::Color;

#[derive(PartialEq)]
pub struct Pawn{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    color: Color,
}


impl Pawn {
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Pawn {
        Pawn { color, rank, file, moves: [[0u8;8];8] }
    }
}