
use ch19_5::{vect, HelloMacro};
use hello_macro_derive::HelloMacro;


//macro vs functions
//functions must declare the type and number of parameters it has
//macros can take a vriable number of parameters

//functions don't exapand before compile time, they get called at runtime
//macros are expanded before compile interprets the code (compiler time), so they could 
//   implement a trait on a given type for example, because this needs to be done before compile time.

//macros are generally more difficult to read, understand, and maintain than function definitions.
//macros must be defines or brought into scope before you call them in a file.


//the derive call here called the macro and implemented the hellomacro trait!
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let x = vect![1,2,3];
    //the above macro gets replaced with
    let x1 = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };
    println!("{:#?}",x);

    Pancakes::hello_macro();
}

