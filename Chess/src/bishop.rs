use crate::Piece;
use crate::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct Bishop{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    protected: bool,
    color: Color,
}

#[wasm_bindgen]
impl Bishop {

    pub fn set_protected(&mut self, value: bool) {
        self.protected = value;
    }

    pub fn get_protected(&self) -> bool {
        self.protected
    }

    pub fn get_moves(&self) -> [[u8;8];8] {
        self.moves
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Bishop {
        Bishop { color, rank, file, moves: [[0u8;8];8], protected: false }
    }

    pub fn moves(mut self, board: &mut [[Piece;8];8]) {
        self.moves = [[0u8;8];8];

        let diaglimit = if self.rank > self.file { self.file } else { self.rank };
        //check diagonal up/left
        for i in 1..=diaglimit {
            if Piece::get_color(&board[self.rank - i][self.file - i]) != &self.color {
                self.moves[self.rank - i][self.file - i] = 1;
            }
            else {
                if Piece::get_color(&board[self.rank - i][self.file - i]) == &self.color {
                    Piece::is_protected(&mut board[self.rank - i][self.file - i], true);
                }
                break;
            }
        }

        let diaglimit = if self.rank > (8 - self.file) { 8 - self.file } else { self.rank };
        //check diagonal up/right
        for i in 1..=diaglimit {
            if Piece::get_color(&board[self.rank - i][self.file + i]) != &self.color {
                self.moves[self.rank - i][self.file + i] = 1;
            }
            else {
                if Piece::get_color(&board[self.rank - i][self.file + i]) == &self.color {
                    Piece::is_protected(&mut board[self.rank - i][self.file + i], true);
                }
                break;
            }
        }

        let diaglimit = if (8 - self.rank) > (8 - self.file) { 8 - self.file } else { 8 - self.rank };
        //check diagonal down/right
        for i in 1..=diaglimit {
            if Piece::get_color(&board[self.rank + i][self.file + i]) != &self.color {
                self.moves[self.rank + i][self.file + i] = 1;
            }
            else {
                if Piece::get_color(&board[self.rank + i][self.file + i]) == &self.color {
                    Piece::is_protected(&mut board[self.rank + i][self.file + i], true);
                }
                break;
            }
        }

        let diaglimit = if (8 - self.rank) > self.file { self.file } else { 8 - self.rank };
        //check diagonal down/left
        for i in 1..=diaglimit {
            if Piece::get_color(&board[self.rank + i][self.file - i]) != &self.color {
                self.moves[self.rank + i][self.file - i] = 1;
            }
            else {
                if Piece::get_color(&board[self.rank + i][self.file - i]) == &self.color {
                    Piece::is_protected(&mut board[self.rank + i][self.file - i], true);
                }
                break;
            }
        }
    }
}