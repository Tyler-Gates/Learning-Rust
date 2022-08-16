//Cargo knows to look for integration test files in this directory. /tests/..

//can run --test <name of integration test file> to run specific integration test file
//ex: cargo test --test integration_test

use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

//if the project only contains a src/main.rs and DOESNT have a lb.rs file we can't create integration tests in the tests directory 
// and bring functions defined in the src/main.rs file into scope with a use statement. 

// Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.