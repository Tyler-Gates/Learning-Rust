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

pub enum Piece {
    K(King),
    Q(Queen),
    R(Rook),
    N(Knight),
    B(Bishop),
    P(Pawn),
    Empty
}

pub enum Color {
    Black,
    White,
    None
}

pub struct Board {
    positions: [[(Piece,Color);8];8],
}

impl Board {
    
    pub fn new() -> Board {
        use crate::Piece::*;
        use crate::Color::*;
        let mut positions: [[(Piece,Color);8];8] = [
            [(R(Rook::new()),Black),(N(Knight::new()),Black),(B(Bishop::new()),Black),(Q(Queen::new()),Black),(K(King::new()),Black),(B(Bishop::new()),Black),(N(Knight::new()),Black),(R(Rook::new()),Black)],
            [(P(Pawn::new()),Black),(P(Pawn::new()),Black),(P(Pawn::new()),Black),(P(Pawn::new()),Black),(P(Pawn::new()),Black),(P(Pawn::new()),Black),(P(Pawn::new()),Black),(P(Pawn::new()),Black)],
            [(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None)],
            [(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None)],
            [(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None)],
            [(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None),(Empty,None)],
            [(P(Pawn::new()),White),(P(Pawn::new()),White),(P(Pawn::new()),White),(P(Pawn::new()),White),(P(Pawn::new()),White),(P(Pawn::new()),White),(P(Pawn::new()),White),(P(Pawn::new()),White)],
            [(R(Rook::new()),White),(N(Knight::new()),White),(B(Bishop::new()),White),(Q(Queen::new()),White),(K(King::new()),White),(B(Bishop::new()),White),(N(Knight::new()),White),(R(Rook::new()),White)]
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
