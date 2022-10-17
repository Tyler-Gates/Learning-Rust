use std::fmt;
use wasm_bindgen::prelude::*;
use js_sys::Math::*;



#[wasm_bindgen]
pub struct Board {
    board: Vec<Vec<u8>>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Board {
    pub fn random_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<u8>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                let random = js_sys::Math::floor(js_sys::Math::random() * 2.0) as usize;
                if random == 1 {
                    temp.push(1);
                }
                else {
                    temp.push(0);
                }
            }
            board.push(temp);
        }
        Board { board, width, height }
    }

    pub fn dead_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<u8>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                temp.push(0);
            }
            board.push(temp);
        }
        Board { board, width, height }
    }

    pub fn set_cell(mut self, x: usize, y: usize) -> Self {
        if self.board[y][x] == 0 || self.board[y][x] == 2{
            self.board[y][x] = 1;
        }
        else {
            self.board[y][x] = 0;
        }
        self
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

    pub fn next_board_state(mut self) -> Self {
        let mut newboard: Vec<Vec<u8>> = Vec::new();
        let mut top = false;
        let mut left = false;
        let mut right = false;
        let mut bottom = false;

        for i in 0..self.board.len() {
            let mut temp: Vec<u8> = Vec::new();
            for j in 0..self.board[0].len() {
                let mut count = 0;
                left = j == 0;
                top = i == 0;
                right = j == self.board[0].len() - 1;
                bottom = i == self.board.len() - 1;
                if top && left {
                    if self.board[i][j+1] == 1 {
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i][self.width as usize-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][self.width as usize-1] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize -1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize -1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize -1][self.width as usize -1] == 1{
                        count = count + 1;
                    }
                    
                }
                else if top && right {
                    if self.board[i][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i][0] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][0] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize -1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize -1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize -1][0] == 1{
                        count = count + 1;
                    }
                }
                else if bottom && right {
                    if self.board[i][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i][0] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][0] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[0][0] == 1{
                        count = count + 1;
                    }
                }
                else if bottom && left {
                    if self.board[i][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i][self.width as usize-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][self.width as usize-1] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[0][self.width as usize-1] == 1{
                        count = count + 1;
                    }
                }
                else if top {
                    if self.board[i+1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize-1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[self.height as usize-1][j-1] == 1{
                        count = count + 1;
                    }
                }
                else if bottom {
                    if self.board[i-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[0][j-1] == 1{
                        count = count + 1;
                    }
                }
                else if right {
                    if self.board[i-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][0] == 1{
                        count = count + 1;
                    }
                    if self.board[i][0] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][0] == 1{
                        count = count + 1;
                    }
                }
                else if left {
                    if self.board[i-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][self.width as usize - 1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][self.width as usize - 1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][self.width as usize - 1] == 1{
                        count = count + 1;
                    }
                }
                else {
                    if self.board[i-1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j-1] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j] == 1{
                        count = count + 1;
                    }
                    if self.board[i+1][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i][j+1] == 1{
                        count = count + 1;
                    }
                    if self.board[i-1][j+1] == 1{
                        count = count + 1;
                    }
                }
                if count <= 1 {
                    if self.board[i][j] == 1 {
                        temp.push(2);
                    }
                    else{
                        temp.push(0);
                    }
                }
                else if count == 2 {
                    if self.board[i][j] == 1 {
                        temp.push(1);
                    }
                    else {
                        temp.push(0);
                    }
                }
                else if count == 3 {
                    temp.push(1);
                }
                else if count > 3 {
                    if self.board[i][j] == 1 {
                        temp.push(2);
                    }
                    else{
                        temp.push(0);
                    }
                }
            }
            newboard.push(temp);
        }
        self.board = newboard;
        self
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut filledin = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
              if self.board[i as usize][j as usize] == 1 {
                write!(f, "{}", '1');
              }
              else if self.board[i as usize][j as usize] == 2{
                write!(f, "{}", '2');
              }
              else {
                write!(f, "{}", '0');
              }
            }
            write!(f, "{}", '\n');
          }
          

        Ok(())
    }
}

impl Board {
    pub fn custom_instantiation(board: Vec<Vec<u8>>) -> Board {
        Board { width: board.len() as u32 , height: board[0].len() as u32, board }
    }

}


// #[cfg(test)]
// mod tests {
//     use crate::Board;

//     #[test]
//     pub fn test_next_on_dead() {
//         let mut deadboard = vec![vec![false,false,false],
//                                  vec![false,false,false],
//                                  vec![false,false,false]];
//         let mut game = Board::custom_instantiation(deadboard.clone());
//         game = game.next_board_state();
//         assert_eq!(game.board, deadboard);
//     }

//     #[test]
//     pub fn test_next_on_one_alive() {
//         let mut oneboard =  vec![vec![false,false,false],
//                                  vec![false,true,false],
//                                  vec![false,false,false]];

//         let mut deadboard = vec![vec![false,false,false],
//                                  vec![false,false,false],
//                                  vec![false,false,false]];
//         let mut game = Board::custom_instantiation(oneboard);
//         game = game.next_board_state();
//         assert_eq!(game.board, deadboard);                    
//     }

//     #[test]
//     pub fn test_next_on_two_alive() {
//         let mut twoboard =  vec![vec![false,false,false],
//                                  vec![false,true,true],
//                                  vec![false,false,false]];

//         let mut deadboard = vec![vec![false,false,false],
//                                  vec![false,false,false],
//                                  vec![false,false,false]];
//         let mut game = Board::custom_instantiation(twoboard);
//         game = game.next_board_state();
//         assert_eq!(game.board, deadboard);                    
//     }

//     #[test]
//     pub fn test_next_on_three_alive() {
//         let mut threeboard =  vec![vec![false,true,false],
//                                  vec![false,true,true],
//                                  vec![false,false,false]];

//         let mut nextboard = vec![vec![true,true,true],
//                                  vec![true,true,true],
//                                  vec![false,false,false]];
//         let mut game = Board::custom_instantiation(threeboard);
//         game = game.next_board_state();
//         assert_eq!(game.board, nextboard);                    
//     }

//     #[test]
//     pub fn test_pattern_rotated_square() {
//         let mut squareboard =  vec![vec![false,true,false],
//                                  vec![true,true,true],
//                                  vec![false,true,false]];

//         let mut nextboard = vec![vec![true,true,true],
//                                  vec![true,false,true],
//                                  vec![true,true,true]];
//         let mut game = Board::custom_instantiation(squareboard);
//         game = game.next_board_state();
//         assert_eq!(game.board, nextboard);                    
//     }

//     #[test]
//     pub fn test_pattern_line() {
//         let mut lineboard =  vec![vec![false,true,false],
//                                  vec![false,true,false],
//                                  vec![false,true,false]];

//         let mut nextboard = vec![vec![false,false,false],
//                                  vec![true,true,true],
//                                  vec![false,false,false]];
//         let mut game = Board::custom_instantiation(lineboard);
//         game = game.next_board_state();
//         assert_eq!(game.board, nextboard);                    
//     }

// }