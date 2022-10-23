use crate::Piece;
use crate::Color;

#[derive(PartialEq)]
pub struct Pawn{
    moves: [[u8;8];8],
    rank: usize,
    file: usize,
    protected: bool,
    color: Color,
}


impl Pawn {

    pub fn set_protected(&mut self, value: bool) {
        self.protected = value;
    }

    pub fn get_protected(&self) -> bool {
        self.protected
    }

    pub fn get_moves(&self) -> &[[u8;8];8] {
        &self.moves
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn new(rank: usize, file: usize, color: Color) -> Pawn {
        Pawn { color, rank, file, moves: [[0u8;8];8], protected: false }
    }

    pub fn moves(mut self, board: &mut [[Piece;8];8]) {
        self.moves = [[0u8;8];8];

        //sets which way to move
        let color: isize = if self.color == Color::White { 1 } else { -1 };
        //sets the opposing color
        let enemy = if color == 1 { Color::Black } else { Color::White };

        //validates a move is possible.. might not be needed as the piece will change once reaching the end of the board
        if ( color == 1 && self.rank < 6 ) || ( color == -1 && self.rank > 0 ) {

            //checks moving forward one
            if board[(self.rank as isize + color) as usize][self.file] == Piece::Empty {
                self.moves[(self.rank as isize + color) as usize][self.file] = 1;
            }

            //checks the double move start
            if (self.rank == 1 && color == 1) || (self.rank == 6 && color == -1){
                if board[(self.rank as isize + (2 * color)) as usize][self.file] == Piece::Empty {
                    self.moves[(self.rank as isize + (2 * color)) as usize][self.file] = 1;
                }
            }

            //checks taking diagonal right 
            if self.file < 7 {
                if board[(self.rank as isize + color) as usize][self.file + 1].get_color() == &enemy {
                    self.moves[(self.rank as isize + color) as usize][self.file + 1] = 1;
                }
                if Piece::get_color(&board[(self.rank as isize + color) as usize][self.file + 1]) == &self.color {
                    Piece::is_protected(&mut board[(self.rank as isize + color) as usize][self.file + 1], true);
                }
            }

            //checks taking diagnoal left
            if self.file > 0 {
                if board[(self.rank as isize + color) as usize][self.file - 1].get_color() == &enemy {
                    self.moves[(self.rank as isize + color) as usize][self.file - 1] = 1;
                }
                if Piece::get_color(&board[(self.rank as isize + color) as usize][self.file - 1]) == &self.color {
                    Piece::is_protected(&mut board[(self.rank as isize + color) as usize][self.file - 1], true);
                }
            }
        }
    }

}
