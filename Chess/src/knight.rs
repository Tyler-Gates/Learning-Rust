use crate::Piece;
use crate::Color;
use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub struct Knight{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    protected: bool,
    color: Color,
}

impl fmt::Display for Knight {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut filledin = String::new();
        for i in 0..8 {
            for j in 0..8 {
              if self.moves[i as usize][j as usize] == 0 {
                write!(f, "{}", '0');
              }
              else {
                write!(f, "{}", '1');
              }
            }
        }
        Ok(())
    }
}   

#[wasm_bindgen]
impl Knight {

    pub fn set_protected(&mut self, value: bool) {
        self.protected = value;
    }

    pub fn get_protected(&self) -> bool {
        self.protected
    }

    pub fn get_moves(&self) -> String {
        self.to_string()
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Knight {
        Knight { color, rank, file, moves: [[0u8;8];8], protected: false }
    }
}

impl Knight {
    pub fn moves(mut self, board: &mut [[Piece;8];8]) {
        self.moves = [[0u8;8];8];

        if self.rank < 6 {
            if self.file < 7 {
                if Piece::get_color(&board[self.rank + 2 ][self.file + 1]) != self.color {
                    self.moves[self.rank + 2][self.file + 1] = 1;
                }
                if Piece::get_color(&board[self.rank + 2 ][self.file + 1]) == self.color {
                    Piece::is_protected(&mut board[self.rank + 2 ][self.file + 1], true);
                }
            }
            if self.file > 0 {
                if Piece::get_color(&board[self.rank + 2][self.file - 1]) != self.color {
                    self.moves[self.rank + 2][self.file - 1] = 1;
                }
                if Piece::get_color(&board[self.rank + 2][self.file - 1]) == self.color {
                    Piece::is_protected(&mut board[self.rank + 2][self.file - 1], true);
                }
            }
        }

        if self.rank > 1 {
            if self.file < 7 {
                if Piece::get_color(&board[self.rank - 2][self.file + 1]) != self.color {
                    self.moves[self.rank - 2][self.file + 1] = 1;
                }
                if Piece::get_color(&board[self.rank - 2][self.file + 1]) == self.color {
                    Piece::is_protected(&mut board[self.rank - 2][self.file + 1], true);
                }
            }
            if self.file > 0 {
                if Piece::get_color(&board[self.rank - 2][self.file - 1]) != self.color {
                    self.moves[self.rank - 2][self.file - 1] = 1;
                }
                if Piece::get_color(&board[self.rank - 2][self.file - 1]) == self.color {
                    Piece::is_protected(&mut board[self.rank - 2][self.file - 1], true);
                }
            }
        }

        if self.file > 1 {
            if self.rank < 7 {
                if Piece::get_color(&board[self.rank + 1][self.file - 2]) != self.color {
                    self.moves[self.rank + 1][self.file - 2] = 1;
                }
                if Piece::get_color(&board[self.rank + 1][self.file - 2]) == self.color {
                    Piece::is_protected(&mut board[self.rank + 1][self.file - 2], true);
                }
            }
            if self.rank > 0 {
                if Piece::get_color(&board[self.rank - 1][self.file - 2]) != self.color {
                    self.moves[self.rank - 1][self.file - 2] = 1;
                }
                if Piece::get_color(&board[self.rank - 1][self.file - 2]) == self.color {
                    Piece::is_protected(&mut board[self.rank - 1][self.file - 2], true);
                }
            }
        }
        
        if self.file < 6 {
            if self.rank < 7 {
                if Piece::get_color(&board[self.rank + 1][self.file + 2]) != self.color {
                    self.moves[self.rank + 1][self.file + 2] = 1;
                }
                if Piece::get_color(&board[self.rank + 1][self.file + 2]) == self.color {
                    Piece::is_protected(&mut board[self.rank + 1][self.file + 2], true);
                }
            }
            if self.rank > 0 {
                if Piece::get_color(&board[self.rank - 1][self.file + 2]) != self.color {
                    self.moves[self.rank - 1][self.file + 2] = 1;
                }
                if Piece::get_color(&board[self.rank - 1][self.file + 2]) == self.color {
                    Piece::is_protected(&mut board[self.rank - 1][self.file + 2], true);
                }
            }
        }
    }
}