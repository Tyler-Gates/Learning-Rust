use crate::Piece;
use crate::Color;

#[derive(PartialEq)]
pub struct Bishop{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    color: Color,
}


impl Bishop {

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Bishop {
        Bishop { color, rank, file, moves: [[0u8;8];8] }
    }
}