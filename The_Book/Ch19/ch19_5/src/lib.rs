use proc_macro;


//this is a declarative macro
//the macro_export annotation indicates that the macros should be made available whenever the crate in which the macro is defined is brought into scope
#[macro_export]
macro_rules! vect {
    //one match like arm expression with body, if it matches it will enter body. If no arm matches it will panic
    //the first $ makes it clear this is a macro variable, not a normal variable
    //the $x:expr inside the $() which matches any rust expression and gives the expression the name $x
    //the comma , indicates that a literal comma seperator character could optionally be used after the code match in $()
    //the * specifies that the pattern matches zero or more of whatever precedes the *, in this case $x:expr
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

//this is a procedural macro
//these macro definitions must reside in their own crate with a special crate type
//the function that defines this procedural macro takes and returns a tokenstream
//the tokenstream is defined by the proc_macro crate
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }


pub trait HelloMacro {
    fn hello_macro();
}