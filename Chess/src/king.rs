use crate::Piece;
use crate::Color;
use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub struct King{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    color: Color,
}

impl fmt::Display for King {

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
impl King {

    pub fn get_moves(&self) -> String {
        self.to_string()
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn new(rank: usize, file: usize, color: Color) -> King {
        King { color, rank, file, moves: [[0u8;8];8] }
    }
}

impl King {
    pub fn moves(mut self, board: &[[Piece;8];8]) {
        self.moves = [[0u8;8];8];

        let mut all_enemy: Vec<String> = Vec::new();
        let mut consolidate_enemy = vec!("0";64);

        let enemy_color = if self.color == Color::White { Color::Black } else { Color::White };

        //gathers all enemy attack moves to stop king from moving into check
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] != Piece::Empty || Piece::get_color(&board[i][j]) != self.color {
                    all_enemy.push(Piece::get_moves(&board[i][j]));
                }
            }
        }

        //condense to one board to have all enemy attack tiles
        for i in 0..all_enemy.len() {
            let current: Vec<char> = all_enemy[i].chars().collect();
            for j in 0..64 {
                if consolidate_enemy[j] == "0" && current[j] == '1' {
                    consolidate_enemy[j] = "1";
                }
            }
        }

        //checks to see if move is possible
        if self.rank < 7 {
            if self.file < 7 {
                if consolidate_enemy[(self.rank * 8) + 8 + 1] == "0" && Piece::get_color(&board[self.rank + 1][self.file + 1]) != self.color && !Piece::get_protected(&board[self.rank - 1][self.file]) {
                    self.moves[self.rank + 1][self.file + 1] = 1;
                }
            }
            if self.file > 0 {
                if consolidate_enemy[(self.rank * 8) + 8 - 1] == "0" && Piece::get_color(&board[self.rank + 1][self.file - 1]) != self.color && !Piece::get_protected(&board[self.rank - 1][self.file]) {
                    self.moves[self.rank + 1][self.file - 1] = 1;
                }
            }
            if consolidate_enemy[(self.rank * 8) + 8] == "0" && Piece::get_color(&board[self.rank + 1][self.file]) != self.color && !Piece::get_protected(&board[self.rank - 1][self.file]) {
                self.moves[self.rank + 1][self.file] = 1;
            }
        }

        if self.rank > 0 {
            if self.file < 7 {
                if consolidate_enemy[(self.rank * 8) - 8 + 1] == "0" && Piece::get_color(&board[self.rank - 1][self.file + 1]) != self.color && !Piece::get_protected(&board[self.rank - 1][self.file]) {
                    self.moves[self.rank - 1][self.file + 1] = 1;
                }
            }
            if self.file > 0 {
                if consolidate_enemy[(self.rank * 8) - 8 - 1] == "0" && Piece::get_color(&board[self.rank - 1][self.file - 1]) != self.color && !Piece::get_protected(&board[self.rank - 1][self.file])  {
                    self.moves[self.rank - 1][self.file - 1] = 1;
                }
            }
            if consolidate_enemy[(self.rank * 8) - 8] == "0" && Piece::get_color(&board[self.rank - 1][self.file]) != self.color && !Piece::get_protected(&board[self.rank - 1][self.file]) {
                self.moves[self.rank - 1][self.file] = 1;
            }
        }
        if self.file < 7 {
            if consolidate_enemy[(self.rank * 8) + self.file + 1] == "0" && Piece::get_color(&board[self.rank][self.file + 1]) != self.color && !Piece::get_protected(&board[self.rank][self.file + 1]) {
                self.moves[self.rank][self.file + 1] = 1;
            }
        }
        if self.file > 0 {
            if consolidate_enemy[(self.rank * 8) + self.file - 1] == "0" && Piece::get_color(&board[self.rank][self.file - 1]) != self.color && !Piece::get_protected(&board[self.rank][self.file - 1]) {
                self.moves[self.rank][self.file - 1] = 1;
            }
        }

        //need to check if enemy king is nearby to stop movement
        King::king_check(self,board);
    }


    //checks outer barrier around a piece to see if it is a king, and if so, adjust the possible moves for king
    //kings cannot be closer than 1 square to another king
    fn king_check(mut self, board: &[[Piece;8];8]) {
        if self.rank - 2 >= 0 {

            if self.file - 2 >= 0 {

                if Piece::is_king(&board[self.rank - 2][self.file - 2]) {
                    self.moves[self.rank - 1][self.file - 1] = 0;
                }
            }

            if self.file - 1 >= 0 {
                if Piece::is_king(&board[self.rank - 2][self.file - 1]) {
                    self.moves[self.rank - 1][self.file - 1] = 0;
                    self.moves[self.rank - 1][self.file] = 0;
                }
            }

            if Piece::is_king(&board[self.rank - 2][self.file]) {
                if self.file - 1 >= 0 {
                    self.moves[self.rank - 1][self.file - 1] = 0;
                }
                if self.file + 1 <= 7{
                    self.moves[self.rank - 1][self.file + 1] = 0;
                }

                self.moves[self.rank - 1][self.file] = 0;
            }

            if self.file + 1 <= 7 {
                if Piece::is_king(&board[self.rank - 2][self.file + 1]) {
                    self.moves[self.rank - 1][self.file + 1] = 0;
                    self.moves[self.rank - 1][self.file] = 0;
                }
            }

            if self.file + 2 <= 7 {
                if Piece::is_king(&board[self.rank - 2][self.file + 2]) {
                    self.moves[self.rank - 1][self.file + 1] = 0;
                }
            }
        }

        if self.rank + 2 <= 7 {

            if self.file - 2 >= 0 {

                if Piece::is_king(&board[self.rank + 2][self.file - 2]) {
                    self.moves[self.rank + 1][self.file - 1] = 0;
                }
            }

            if self.file - 1 >= 0 {
                if Piece::is_king(&board[self.rank + 2][self.file - 1]) {
                    self.moves[self.rank + 1][self.file - 1] = 0;
                    self.moves[self.rank + 1][self.file] = 0;
                }
            }

            if Piece::is_king(&board[self.rank + 2][self.file]) {
                if self.file - 1 >= 0 {
                    self.moves[self.rank + 1][self.file - 1] = 0;
                }
                if self.file + 1 <= 7{
                    self.moves[self.rank + 1][self.file + 1] = 0;
                }

                self.moves[self.rank + 1][self.file] = 0;
            }

            if self.file + 1 <= 7 {
                if Piece::is_king(&board[self.rank + 2][self.file + 1]) {
                    self.moves[self.rank + 1][self.file + 1] = 0;
                    self.moves[self.rank + 1][self.file] = 0;
                }
            }

            if self.file + 2 <= 7 {
                if Piece::is_king(&board[self.rank + 2][self.file + 2]) {
                    self.moves[self.rank + 1][self.file + 1] = 0;
                }
            }
        }

        if self.file + 2 <= 7 {

            if self.rank - 1 >= 0 {
                if Piece::is_king(&board[self.rank - 1][self.file + 2]) {
                    self.moves[self.rank - 1][self.file + 1] = 0;
                    self.moves[self.rank][self.file + 1] = 0;
                }
            }

            if Piece::is_king(&board[self.rank][self.file + 2]) {
                if self.rank - 1 >= 0 {
                    self.moves[self.rank - 1][self.file + 1] = 0;
                }
                if self.rank + 1 <= 7{
                    self.moves[self.rank + 1][self.file + 1] = 0;
                }

                self.moves[self.rank][self.file + 1] = 0;
            }

            if self.rank + 1 <= 7 {
                if Piece::is_king(&board[self.rank + 1][self.file + 2]) {
                    self.moves[self.rank + 1][self.file + 1] = 0;
                    self.moves[self.rank][self.file] = 0;
                }
            }
        }

        if self.file - 2 >= 0 {

            if self.rank - 1 >= 0 {
                if Piece::is_king(&board[self.rank - 1][self.file - 2]) {
                    self.moves[self.rank - 1][self.file - 1] = 0;
                    self.moves[self.rank][self.file - 1] = 0;
                }
            }

            if Piece::is_king(&board[self.rank][self.file - 2]) {
                if self.rank - 1 >= 0 {
                    self.moves[self.rank - 1][self.file - 1] = 0;
                }
                if self.rank + 1 <= 7{
                    self.moves[self.rank + 1][self.file - 1] = 0;
                }

                self.moves[self.rank][self.file - 1] = 0;
            }

            if self.rank + 1 <= 7 {
                if Piece::is_king(&board[self.rank + 1][self.file - 2]) {
                    self.moves[self.rank + 1][self.file - 1] = 0;
                    self.moves[self.rank][self.file] = 0;
                }
            }
        }
    }
}