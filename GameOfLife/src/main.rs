use crate::board::Board;
mod board;

fn main() {
    let x = Board::random_state_instantiation(3,3);
    println!("{}", x);
}
