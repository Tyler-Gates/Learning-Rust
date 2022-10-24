use std::fmt;
use wasm_bindgen::prelude::*;
use js_sys::Math::*;


/// A Board is represented here
#[wasm_bindgen]
pub struct Board {
    /// Board must hold it's current state
    /// # Number Representations
    /// * '0' - Represents a dead cell
    /// * '1' - Represents a cell that just died
    /// * '2' - Represents a cell that is currently alive
    /// * '3' - Represents a cell that is about to be born
    board: Vec<Vec<u8>>,
    /// width of board
    width: u32,
    /// height of board
    height: u32,
    /// death toggle priority for when birth toggle is turned off
    death: bool,
}

#[wasm_bindgen]
impl Board {
    /// Returns a board in a random state
    ///
    /// # Arguments
    /// * 'width' - The width of the board
    /// * 'height' - The height of the board
    ///
    /// # Examples
    /// ```
    /// let board = Board.random_state_instantiation(64,64);
    /// ```
    pub fn random_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<u8>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                let random = js_sys::Math::floor(js_sys::Math::random() * 2.0) as usize;
                if random == 1 {
                    temp.push(2);
                }
                else {
                    temp.push(0);
                }
            }
            board.push(temp);
        }
        Board { board, width, height, death: false }
    }

    /// Returns a board in a complete off state
    ///
    /// # Arguments
    /// * 'width' - The width of the board
    /// * 'height' - The height of the board
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// ```
    pub fn dead_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<u8>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                temp.push(0);
            }
            board.push(temp);
        }
        Board { board, width, height, death: false }
    }

    /// Turn a cell on/off by it's x,y coordinates
    ///
    /// # Arguments
    /// * 'x' - The x coordinate
    /// * 'y' - The y coordinate
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// board.set_cell(5,5)
    /// ```
    pub fn set_cell(mut self, x: usize, y: usize) -> Self {
        if self.board[y][x] == 0 || self.board[y][x] == 1{
            self.board[y][x] = 2;
        }
        else {
            self.board[y][x] = 0;
        }
        self
    }

    /// Toggle the death priority boolean
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// board.death_only()
    /// ```
    pub fn death_only(mut self) -> Self {
        self.death = !self.death;
        self
    }

    /// Returns the width of the board
    ///
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// let width = board.width()
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the board
    ///
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// let height = board.height()
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a string representing the board state
    ///
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// let board_string = board.board()
    /// ```
    pub fn board(&self) -> String {
        self.to_string()
    }

    /// Set's the next board state based on current state
    ///
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// board.next_board_state()
    /// ```
    pub fn next_board_state(mut self) -> Self {
        let mut newboard: Vec<Vec<u8>> = Vec::new();

        for i in 0..self.board.len() {
            let mut temp: Vec<u8> = Vec::new();
            for j in 0..self.board[0].len() {
                let left = j == 0;
                let top = i == 0;
                let right = j == self.board[0].len() - 1;
                let bottom = i == self.board.len() - 1;
                let count = self.surrounding_cell_check(top, left, right, bottom, i, j);
                if count <= 1 {
                    if self.board[i][j] == 2 {
                        temp.push(1);
                    }
                    else{
                        temp.push(0);
                    }
                }
                else if count == 2 {
                    if self.board[i][j] == 2 {
                        temp.push(2);
                    }
                    else {
                        temp.push(0);
                    }
                }
                else if count == 3 {
                    temp.push(2);
                }
                else if count > 3 {
                    if self.board[i][j] == 2 {
                        temp.push(1);
                    }
                    else{
                        temp.push(0);
                    }
                }
            }
            newboard.push(temp);
        }
        self.board = newboard;
        self = self.get_new_cells();
        self
    }
}

impl fmt::Display for Board {

    /// Converts the board into a string for JavaScript to handle
    ///
    /// # Number Representations
    /// * '0' - Represents a dead cell
    /// * '1' - Represents a cell that just died
    /// * '2' - Represents a cell that is currently alive
    /// * '3' - Represents a cell that is about to be born
    ///
    /// # Examples
    /// ```
    /// let board = Board.dead_state_instantiation(64,64);
    /// println!({},board);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut filledin = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
              if self.board[i as usize][j as usize] == 2 {
                write!(f, "{}", '2');
              }
              else if self.board[i as usize][j as usize] == 3{
                write!(f, "{}", '3');
              }
              else if self.board[i as usize][j as usize] == 1{
                write!(f, "{}", '1');
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

    /// Returns a board with what is passed
    ///
    /// # Arguements
    /// * 'board' - A Vec<Vec<u8>> that represents the board
    ///
    /// # Examples
    /// ```
    /// // board is set up 
    /// let custom: Vec<Vec<u8>> = Vec::new();
    /// // ... custom board is completed in setup
    /// let board = Board.custom_instantiation(custom);
    /// ```
    pub fn custom_instantiation(board: Vec<Vec<u8>>) -> Board {
        Board { width: board.len() as u32 , height: board[0].len() as u32, board, death: false }
    }

    /// Checks for cells next run that will be alive for the 'about to be born' cell coloring
    /// Called by next_board_state()
    fn get_new_cells(mut self) -> Self {
        let mut newboard: Vec<Vec<u8>> = Vec::new();

        for i in 0..self.board.len() {
            let mut temp: Vec<u8> = Vec::new();
            for j in 0..self.board[0].len() {
                let left = j == 0;
                let top = i == 0;
                let right = j == self.board[0].len() - 1;
                let bottom = i == self.board.len() - 1;
                let count = self.surrounding_cell_check(top, left, right, bottom, i, j);
                if count == 3 {
                    if self.death {
                        if self.board[i][j] == 0 {
                            self.board[i].remove(j);
                            self.board[i].insert(j,3);
                        }
                    }
                    else {
                        if self.board[i][j] != 2 {
                            self.board[i].remove(j);
                            self.board[i].insert(j,3);
                        }
                    }
                }
            }
        }
        self
    }

    /// Returns the count of surrounding cells that are on for a given position (i,j)
    /// used by next_board_state() and get_new_cells()
    ///
    /// # Arguements
    /// * 'top' - Boolean that represents when the cell is bordering the top side
    /// * 'left' - Boolean that represents when the cell is bordering the left side
    /// * 'right' - Boolean that represents when the cell is bordering the right side
    /// * 'bottom' - Boolean that represents when the cell is bordering the bottom side
    /// * 'i' - The x coordinate for a given cell
    /// * 'j' - The y coordinate for a given cell
    ///
    fn surrounding_cell_check(&self, top: bool, left: bool, right: bool, bottom: bool, i: usize, j: usize) -> u32{
        let mut count = 0;
        if top && left {
            if self.board[i][j+1] ==  2 {
                count = count + 1;
            }
            if self.board[i+1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j] ==  2{
                count = count + 1;
            }
            if self.board[i][self.width as usize-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][self.width as usize-1] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize -1][j] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize -1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize -1][self.width as usize -1] ==  2{
                count = count + 1;
            }
            
        }
        else if top && right {
            if self.board[i][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j] ==  2{
                count = count + 1;
            }
            if self.board[i][0] ==  2{
                count = count + 1;
            }
            if self.board[i+1][0] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize -1][j] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize -1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize -1][0] ==  2{
                count = count + 1;
            }
        }
        else if bottom && right {
            if self.board[i][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j] ==  2{
                count = count + 1;
            }
            if self.board[i][0] ==  2{
                count = count + 1;
            }
            if self.board[i-1][0] ==  2{
                count = count + 1;
            }
            if self.board[0][j] ==  2{
                count = count + 1;
            }
            if self.board[0][j-1] ==  2{
                count = count + 1;
            }
            if self.board[0][0] ==  2{
                count = count + 1;
            }
        }
        else if bottom && left {
            if self.board[i][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j] ==  2{
                count = count + 1;
            }
            if self.board[i][self.width as usize-1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][self.width as usize-1] ==  2{
                count = count + 1;
            }
            if self.board[0][j] ==  2{
                count = count + 1;
            }
            if self.board[0][j+1] ==  2{
                count = count + 1;
            }
            if self.board[0][self.width as usize-1] ==  2{
                count = count + 1;
            }
        }
        else if top {
            if self.board[i+1][j] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i][j-1] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize-1][j] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize-1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[self.height as usize-1][j-1] ==  2{
                count = count + 1;
            }
        }
        else if bottom {
            if self.board[i-1][j] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i][j-1] ==  2{
                count = count + 1;
            }
            if self.board[0][j] ==  2{
                count = count + 1;
            }
            if self.board[0][j+1] ==  2{
                count = count + 1;
            }
            if self.board[0][j-1] ==  2{
                count = count + 1;
            }
        }
        else if right {
            if self.board[i-1][j] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j] ==  2{
                count = count + 1;
            }
            if self.board[i-1][0] ==  2{
                count = count + 1;
            }
            if self.board[i][0] ==  2{
                count = count + 1;
            }
            if self.board[i+1][0] ==  2{
                count = count + 1;
            }
        }
        else if left {
            if self.board[i-1][j] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j] ==  2{
                count = count + 1;
            }
            if self.board[i-1][self.width as usize - 1] ==  2{
                count = count + 1;
            }
            if self.board[i][self.width as usize - 1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][self.width as usize - 1] ==  2{
                count = count + 1;
            }
        }
        else {
            if self.board[i-1][j] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j-1] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j] ==  2{
                count = count + 1;
            }
            if self.board[i+1][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i][j+1] ==  2{
                count = count + 1;
            }
            if self.board[i-1][j+1] ==  2{
                count = count + 1;
            }
        }
        count
    }

}


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

        let mut nextboard = vec![vec![true,true,true],
                                 vec![true,true,true],
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