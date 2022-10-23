use crate::king::King;
use crate::queen::Queen;
use crate::rook::Rook;
use crate::knight::Knight;
use crate::bishop::Bishop;
use crate::pawn::Pawn;

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
    pub fn get_color(&self) -> &Color {
        match self {
            Piece::K(piece) => {return piece.get_color();}
            Piece::Q(piece) => {return piece.get_color();}
            Piece::R(piece) => {return piece.get_color();}
            Piece::N(piece) => {return piece.get_color();}
            Piece::B(piece) => {return piece.get_color();}
            Piece::P(piece) => {return piece.get_color();}
            Piece::Empty=> {return &Color::None;}
        }
    }

    pub fn get_moves(&self) -> &[[u8;8];8] {
        match self {
            Piece::K(piece) => {return piece.get_moves();}
            Piece::Q(piece) => {return piece.get_moves();}
            Piece::R(piece) => {return piece.get_moves();}
            Piece::N(piece) => {return piece.get_moves();}
            Piece::B(piece) => {return piece.get_moves();}
            Piece::P(piece) => {return piece.get_moves();}
            Piece::Empty=> {return &[[0u8;8];8];}
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


#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
    None
}

pub struct Board {
    positions: [[Piece;8];8],
}

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
}
