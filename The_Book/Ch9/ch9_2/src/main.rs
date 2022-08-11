use std::fs::File;
use std::fs;
use std::io::{self, Read, ErrorKind};

//The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. and only works in functions with a compatible return type
//trying to use ? in main for instance would cause the code to panic because main has a return type of ()
//fn main() -> Result<(), Box<dyn Error>> { 
    //this would work with ?


fn main() {
    let f = File::open("hello.txt");
    //unwraps provide error and value handling, and expect lets you provide your own custom message for error or handle the item
    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");
    
    //primitive match expressions to handle errors
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    //closures and unwrap to handle errors
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

//propagates the error by letting the code calling function deal with it via returning a result
//gets demolished by adding ? before the semicolon
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//same as above but uses the ?
//NOTE: When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. 
fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;//here
    let mut s = String::new();
    f.read_to_string(&mut s)?;//here
    Ok(s)
}


//same as above but chained and simplified
fn read_username_from_file_super_simplified() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

//std library contains a function that makes it very simple
fn read_username_from_file_most_simplified() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

//? can also be used on Options in a function that returns an option like below
//CANT MIX AND MATCH
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}