use rand::random;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<bool>>,
}


impl Board {
    pub fn random_state_instantiation(width: u32, height: u32) -> Board {
        let mut board: Vec<Vec<bool>> = Vec::new();
        for i in 0..height {
            let mut temp = Vec::new();
            for j in 0..width {
                temp.push(rand::random::<bool>());
            }
            board.push(temp);
        }
        Board { board }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.board.len() {
            let mut text = String::new();
            for j in 0..self.board[0].len() {
                if self.board[i][j] {
                    text.push('#');
                }
                else{
                    text.push(' ');
                }
                text.push(' ');
            }
            write!(f, "{}\n", text);
        }
        Ok(())
    }
}