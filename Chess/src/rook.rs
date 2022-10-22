use crate::Piece;
use crate::Color;


#[derive(PartialEq)]
pub struct Rook{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    color: Color,
}

impl Rook {

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Rook {
        Rook { color, rank, file, moves: [[0u8;8];8] }
    }

    // sets active all possible move positions on a board
    pub fn moves(mut self, board: &[[Piece;8];8]) {
        self.moves = [[0u8;8];8];

        //Checking left first
        for i in (0..self.rank).rev() {
            if board[self.file as usize][i] == Piece::Empty {
                self.moves[self.file][i] = 1;
            }
            else if Piece::get_color(&board[self.file][i]) != &self.color {
                self.moves[self.file][i] = 1;
            }
            else {
                break;
            }
        }
        //checking right
        for i in self.rank..7 {
            if board[self.file][i] == Piece::Empty {
                self.moves[self.file][i] = 1;
            }
            else if Piece::get_color(&board[self.file][i]) != &self.color {
                self.moves[self.file][i] = 1;
            }
            else {
                break;
            }
        }
        //checks up
        for i in self.file..7 {
            if board[self.rank][i] == Piece::Empty {
                self.moves[self.rank][i] = 1;
            }
            else if Piece::get_color(&board[self.rank][i]) != &self.color {
                self.moves[self.rank][i] = 1;
            }
            else {
                break;
            }
        }
        //checks down
        for i in (0..self.file).rev() {
            if board[self.rank][i] == Piece::Empty {
                self.moves[self.rank][i] = 1;
            }
            else if Piece::get_color(&board[self.rank][i]) != &self.color {
                self.moves[self.rank][i] = 1;
            }
            else {
                break;
            }
        }
    }
}