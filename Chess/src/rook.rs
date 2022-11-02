use crate::Piece;
use crate::Color;
use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub struct Rook{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    protected: bool,
    color: Color,
}

impl fmt::Display for Rook {

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
impl Rook {

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

    pub fn new(rank: usize, file: usize, color: Color) -> Rook {
        Rook { color, rank, file, moves: [[0u8;8];8], protected: false }
    }
}

impl Rook {
    // sets active all possible move positions on a board
    pub fn moves(mut self, board: &mut [[Piece;8];8]) {
        self.moves = [[0u8;8];8];

        //Checking left first
        for i in (0..self.rank).rev() {
            if Piece::get_color(&board[i][self.file]) != self.color {
                self.moves[i][self.file] = 1;
            }
            else {
                if Piece::get_color(&board[i][self.file]) == self.color {
                    Piece::is_protected(&mut board[i][self.file], true);
                }
                break;
            }
        }
        //checking right
        for i in self.rank+1..8 {
            if Piece::get_color(&board[i][self.file]) != self.color {
                self.moves[i][self.file] = 1;
            }
            else {
                if Piece::get_color(&board[i][self.file]) == self.color {
                    Piece::is_protected(&mut board[i][self.file], true);
                }
                break;
            }
        }
        //checks up
        for i in self.file+1..8 {
            if Piece::get_color(&board[self.rank][i]) != self.color {
                self.moves[self.rank][i] = 1;
            }
            else {
                if Piece::get_color(&board[self.rank][i]) == self.color {
                    Piece::is_protected(&mut board[self.rank][i], true);
                }
                break;
            }
        }
        //checks down
        for i in (0..self.file).rev() {
            if Piece::get_color(&board[self.rank][i]) != self.color {
                self.moves[self.rank][i] = 1;
            }
            else {
                if Piece::get_color(&board[self.rank][i]) == self.color {
                    Piece::is_protected(&mut board[self.rank][i], true);
                }
                break;
            }
        }
    }
}