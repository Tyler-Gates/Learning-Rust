use std::fmt;
use wasm_bindgen::prelude::*;




#[wasm_bindgen]
pub struct Board {
    board: Vec<Vec<bool>>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Board {
    pub fn random_board(width: u32, height:u32) -> Board {
        Board::random_state_instantiation(width,height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn board(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut filledin = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
              if self.board[i as usize][j as usize] == true {
                write!(f, "{}", '◼');
              }
              else {
                write!(f, "{}", '◻');
              }
            }
            write!(f, "{}", '\n');
          }
          

        Ok(())
    }
}

impl Board {
    pub fn custom_instantiation(board: Vec<Vec<bool>>) -> Board {
        Board { width: board.len() as u32 , height: board[0].len() as u32, board }
    }

    pub fn random_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<bool>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                temp.push(true);
            }
            board.push(temp);
        }
        Board { board, width, height }
    }

    pub fn dead_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<bool>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                temp.push(false);
            }
            board.push(temp);
        }
        Board { board, width, height }
    }

    pub fn next_board_state(mut self) -> Self {
        let mut newboard: Vec<Vec<bool>> = Vec::new();
        let mut top = false;
        let mut left = false;
        let mut right = false;
        let mut bottom = false;

        for i in 0..self.board.len() {
            let mut temp: Vec<bool> = Vec::new();
            for j in 0..self.board[0].len() {
                let mut count = 0;
                left = j == 0;
                top = i == 0;
                right = j == self.board[0].len() - 1;
                bottom = i == self.board.len() - 1;
                if top && left {
                    if self.board[i][j+1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j] {
                        count = count + 1;
                    }
                }
                else if top && right {
                    if self.board[i][j-1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j] {
                        count = count + 1;
                    }
                }
                else if bottom && right {
                    if self.board[i][j-1] {
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i-1][j] {
                        count = count + 1;
                    }
                }
                else if bottom && left {
                    if self.board[i][j+1] {
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i-1][j] {
                        count = count + 1;
                    }
                }
                else if top {
                    if self.board[i+1][j] {
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i][j+1] {
                        count = count + 1;
                    }
                    if self.board[i][j-1] {
                        count = count + 1;
                    }
                }
                else if bottom {
                    if self.board[i-1][j] {
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i][j+1] {
                        count = count + 1;
                    }
                    if self.board[i][j-1] {
                        count = count + 1;
                    }
                }
                else if right {
                    if self.board[i-1][j] {
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i][j-1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j] {
                        count = count + 1;
                    }
                }
                else if left {
                    if self.board[i-1][j] {
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i][j+1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j] {
                        count = count + 1;
                    }
                }
                else {
                    if self.board[i-1][j] {
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i][j-1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] {
                        count = count + 1;
                    }
                    if self.board[i+1][j] {
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] {
                        count = count + 1;
                    }
                    if self.board[i][j+1] {
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] {
                        count = count + 1;
                    }
                }
                if count <= 1 {
                    temp.push(false);
                }
                else if count == 2 {
                    if self.board[i][j] == true {
                        temp.push(true);
                    }
                    else {
                        temp.push(false);
                    }
                }
                else if count == 3 {
                    temp.push(true);
                }
                else if count > 3 {
                    temp.push(false);
                }
            }
            newboard.push(temp);
        }
        self.board = newboard;
        self
    }

}

// impl fmt::Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}[2J", 27 as char);
//         for i in 0..self.board.len() {
//             let mut text = String::new();
//             for j in 0..self.board[0].len() {
//                 if self.board[i][j] {
//                     text.push('#');
//                 }
//                 else{
//                     text.push(' ');
//                 }
//                 text.push(' ');
//             }
//             write!(f, "{}\n", text);
//         }
//         Ok(())
//     }
// }



#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    pub fn test_next_on_dead() {
        let mut deadboard = vec![vec![false,false,false],
                                 vec![false,false,false],
                                 vec![false,false,false]];
        let mut game = Board::custom_instantiation(deadboard.clone());
        game = game.next_board_state();
        assert_eq!(game.board, deadboard);
    }

    #[test]
    pub fn test_next_on_one_alive() {
        let mut oneboard =  vec![vec![false,false,false],
                                 vec![false,true,false],
                                 vec![false,false,false]];

        let mut deadboard = vec![vec![false,false,false],
                                 vec![false,false,false],
                                 vec![false,false,false]];
        let mut game = Board::custom_instantiation(oneboard);
        game = game.next_board_state();
        assert_eq!(game.board, deadboard);                    
    }

    #[test]
    pub fn test_next_on_two_alive() {
        let mut twoboard =  vec![vec![false,false,false],
                                 vec![false,true,true],
                                 vec![false,false,false]];

        let mut deadboard = vec![vec![false,false,false],
                                 vec![false,false,false],
                                 vec![false,false,false]];
        let mut game = Board::custom_instantiation(twoboard);
        game = game.next_board_state();
        assert_eq!(game.board, deadboard);                    
    }

    #[test]
    pub fn test_next_on_three_alive() {
        let mut threeboard =  vec![vec![false,true,false],
                                 vec![false,true,true],
                                 vec![false,false,false]];

        let mut nextboard = vec![vec![false,true,true],
                                 vec![false,true,true],
                                 vec![false,false,false]];
        let mut game = Board::custom_instantiation(threeboard);
        game = game.next_board_state();
        assert_eq!(game.board, nextboard);                    
    }

    #[test]
    pub fn test_pattern_rotated_square() {
        let mut squareboard =  vec![vec![false,true,false],
                                 vec![true,true,true],
                                 vec![false,true,false]];

        let mut nextboard = vec![vec![true,true,true],
                                 vec![true,false,true],
                                 vec![true,true,true]];
        let mut game = Board::custom_instantiation(squareboard);
        game = game.next_board_state();
        assert_eq!(game.board, nextboard);                    
    }

    #[test]
    pub fn test_pattern_line() {
        let mut lineboard =  vec![vec![false,true,false],
                                 vec![false,true,false],
                                 vec![false,true,false]];

        let mut nextboard = vec![vec![false,false,false],
                                 vec![true,true,true],
                                 vec![false,false,false]];
        let mut game = Board::custom_instantiation(lineboard);
        game = game.next_board_state();
        assert_eq!(game.board, nextboard);                    
    }

}