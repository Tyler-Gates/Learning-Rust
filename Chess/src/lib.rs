use crate::king::King;
use crate::queen::Queen;
use crate::rook::Rook;
use crate::knight::Knight;
use crate::bishop::Bishop;
use crate::pawn::Pawn;
use std::fmt;
use wasm_bindgen::prelude::*;

mod king;
mod queen;
mod rook;
mod knight;
mod bishop;
mod pawn;

#[derive(PartialEq)]
pub enum Piece {
    K(King),
    Q(Queen),
    R(Rook),
    N(Knight),
    B(Bishop),
    P(Pawn),
    Empty
}

impl Piece {
    pub fn get_piece(&self) -> &str {
        match self {
            Piece::K(piece) => {return "King";}
            Piece::Q(piece) => {return "Queen";}
            Piece::R(piece) => {return "Rook";}
            Piece::N(piece) => {return "Knight";}
            Piece::B(piece) => {return "Bishop";}
            Piece::P(piece) => {return "Pawn";}
            Piece::Empty => {return "Empty";}
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Piece::K(piece) => {return piece.get_color();}
            Piece::Q(piece) => {return piece.get_color();}
            Piece::R(piece) => {return piece.get_color();}
            Piece::N(piece) => {return piece.get_color();}
            Piece::B(piece) => {return piece.get_color();}
            Piece::P(piece) => {return piece.get_color();}
            Piece::Empty=> {return Color::None;}
        }
    }

    pub fn get_moves(&self) -> String {
        match self {
            Piece::K(piece) => {return piece.get_moves();}
            Piece::Q(piece) => {return piece.get_moves();}
            Piece::R(piece) => {return piece.get_moves();}
            Piece::N(piece) => {return piece.get_moves();}
            Piece::B(piece) => {return piece.get_moves();}
            Piece::P(piece) => {return piece.get_moves();}
            Piece::Empty=> {return "".to_string();}
        }
    }

    pub fn is_protected(&mut self, value: bool) {
        match self {
            Piece::K(piece) => (),
            Piece::Q(piece) => {piece.set_protected(value);}
            Piece::R(piece) => {piece.set_protected(value);}
            Piece::N(piece) => {piece.set_protected(value);}
            Piece::B(piece) => {piece.set_protected(value);}
            Piece::P(piece) => {piece.set_protected(value);}
            Piece::Empty=> ()
        }
    }

    pub fn get_protected(&self) -> bool {
        match self {
            Piece::K(_) => return false,
            Piece::Q(piece) => {return piece.get_protected();}
            Piece::R(piece) => {return piece.get_protected();}
            Piece::N(piece) => {return piece.get_protected();}
            Piece::B(piece) => {return piece.get_protected();}
            Piece::P(piece) => {return piece.get_protected();}
            Piece::Empty=> return false
        }
    }

    pub fn is_king(&self) -> bool {
        match self {
            Piece::K(_) => return true,
            Piece::Q(_) => return false,
            Piece::R(_) => return false,
            Piece::N(_) => return false,
            Piece::B(_) => return false,
            Piece::P(_) => return false,
            Piece::Empty=> return false
        }
    }
}


#[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub enum Color {
    Black,
    White,
    None
}

#[wasm_bindgen]
pub struct Board {
    positions: [[Piece;8];8],
}

#[wasm_bindgen]
impl Board {
    
    //Initializes the start state of a chess game.
    pub fn new() -> Board {
        use crate::Piece::*;
        use crate::Color::*;
        let positions: [[Piece;8];8] = [
            [R(Rook::new(0,0,Black)),N(Knight::new(1,0,Black)),B(Bishop::new(2,0,Black)),Q(Queen::new(3,0,Black)),K(King::new(4,0,Black)),B(Bishop::new(5,0,Black)),N(Knight::new(6,0,Black)),R(Rook::new(7,0,Black))],
            [P(Pawn::new(0,1,Black)),P(Pawn::new(1,1,Black)),P(Pawn::new(2,1,Black)),P(Pawn::new(3,1,Black)),P(Pawn::new(4,1,Black)),P(Pawn::new(5,1,Black)),P(Pawn::new(6,1,Black)),P(Pawn::new(7,1,Black))],
            [(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty)],
            [(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty)],
            [(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty)],
            [(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty),(Empty)],
            [P(Pawn::new(0,6,White)),P(Pawn::new(1,6,White)),P(Pawn::new(2,6,White)),P(Pawn::new(3,6,White)),P(Pawn::new(4,6,White)),P(Pawn::new(5,6,White)),P(Pawn::new(6,6,White)),P(Pawn::new(7,6,White))],
            [R(Rook::new(0,7,White)),N(Knight::new(1,7,White)),B(Bishop::new(2,7,White)),Q(Queen::new(3,7,White)),K(King::new(4,7,White)),B(Bishop::new(5,7,White)),N(Knight::new(6,7,White)),R(Rook::new(7,7,White))]
        ];

        Board { positions }
    }

    pub fn get_board(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut filledin = String::new();
        for i in 0..8 {
            for j in 0..8 {
              if self.positions[i as usize][j as usize] == Piece::Empty {
                write!(f, "{}", '0');
              }
              else if Piece::get_piece(&self.positions[i as usize][j as usize]) == "Pawn"{
                if Piece::get_color(&self.positions[i as usize][j as usize]) == Color::White {
                    write!(f, "{}", '1');
                }
                else {
                    write!(f, "{}", '7');
                }
              }
              else if Piece::get_piece(&self.positions[i as usize][j as usize]) == "Rook" {
                if Piece::get_color(&self.positions[i as usize][j as usize]) == Color::White {
                    write!(f, "{}", '2');
                }
                else {
                    write!(f, "{}", '8');
                }
              }
              else if Piece::get_piece(&self.positions[i as usize][j as usize]) == "Knight" {
                if Piece::get_color(&self.positions[i as usize][j as usize]) == Color::White {
                    write!(f, "{}", '3');
                }
                else {
                    write!(f, "{}", '9');
                }
              }
              else if Piece::get_piece(&self.positions[i as usize][j as usize]) == "Bishop" {
                if Piece::get_color(&self.positions[i as usize][j as usize]) == Color::White {
                    write!(f, "{}", '4');
                }
                else {
                    write!(f, "{}", 'A');
                }
              }
              else if Piece::get_piece(&self.positions[i as usize][j as usize]) == "Queen" {
                if Piece::get_color(&self.positions[i as usize][j as usize]) == Color::White {
                    write!(f, "{}", '5');
                }
                else {
                    write!(f, "{}", 'B');
                }
              }
              else if Piece::get_piece(&self.positions[i as usize][j as usize]) == "King" {
                if Piece::get_color(&self.positions[i as usize][j as usize]) == Color::White {
                    write!(f, "{}", '6');
                }
                else {
                    write!(f, "{}", 'C');
                }
              }
            }
          }
          

        Ok(())
    }
}