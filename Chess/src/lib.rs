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
            Piece::K(f) => {return f.get_color();}
            Piece::Q(f) => {return f.get_color();}
            Piece::R(f) => {return f.get_color();}
            Piece::N(f) => {return f.get_color();}
            Piece::B(f) => {return f.get_color();}
            Piece::P(f) => {return f.get_color();}
            Piece::Empty=> {return &Color::None;}
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
