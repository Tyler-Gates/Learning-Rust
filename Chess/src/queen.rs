use crate::Piece;
use crate::Color;

#[derive(PartialEq)]
pub struct Queen{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    color: Color,
}

impl Queen {
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Queen {
        Queen { color, rank, file, moves: [[0u8;8];8] }
    }
}