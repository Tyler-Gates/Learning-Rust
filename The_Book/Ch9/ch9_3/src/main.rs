use std::net::IpAddr;
//default to Result, not panic!
//In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a Result.
fn main() {

    //this is ok because we know this will always have an Ok Result
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    //tedious and potentially hurts performance
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
        }
    }

}

//when failure is unexpected use panic!, when failure is expected return a result so you can handle it

//creating a type
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

