

//the addition of <'a> means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

//the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. 
//it always returns x, so y doesnt need a lifetime parameter
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}