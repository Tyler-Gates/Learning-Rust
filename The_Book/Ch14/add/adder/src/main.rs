use add_one;
use add_two;


//can use -p to specify which package in the workspace we want to run ex: cargo run -p adder
fn main() {
    let num = 10;
    println!(
        "Hello, world! {num} plus one is {}!",
        add_one::add_one(num)
    );
    println!(
        "Hello, world! {num} plus two is {}!",
        add_two::add_two(num)
    );
}
