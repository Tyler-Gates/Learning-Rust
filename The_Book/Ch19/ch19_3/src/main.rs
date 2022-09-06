use std::fmt;
use std::io::Error;

//newtypes are a lightweight way to achieve encapsulation
//newtypes can expose a public api that is different from the api of the private inner type
//ex:   struct Millimeters(u32);


fn main() {
    //example of a type alias
    //not a new type and can be treated the same as the type it is synonymous with
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //example on utility of type alias. Cuts down on repetitve code and communicates intent via alias name
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }
}

//another example on utility of type alias
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

//the ! is the empty type
//so this function will never return (diverging function) because we can't create the ! type
//expression of type ! can be coerced into any other type, which is good for match statements. (continue, panic!, print!, loop(without break), etc.)
//fn bar() -> ! {
    // --snip--
//}

//dynamically sized types (unsized types)
//we can only know the size of these at runtime
//you can't create a variable holding a dynamically sized type (traits, str, etc.)
//DOESNT work: let s1: str = "hello there!";
//DOES   work: let s1: &str = "hello there!";

// The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.


fn generic<T>(t: T) {
    // --snip--
}

//the compiler turns the above, into below by default

fn genericDefault<T: Sized>(t: T) {
    // --snip--
}
//you can avoid the sized requirement by specifying T to tbe ?Sized, which means it may or may not be sized at compile time
//note the t: &T, it must be behind a pointer of somekind, in this case we chose a reference
fn genericModified<T: ?Sized>(t: &T) {
    // --snip--
}