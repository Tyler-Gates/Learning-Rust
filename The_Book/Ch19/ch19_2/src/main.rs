use std::ops::Add;
use std::fmt;

///////////////////////////

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

//main has function calls relevant to this block

///////////////////////////



////////
//example of choosing a type conversion to add.. Add<Meters> as opposed to default which is self
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
/////////

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

//overload the + operator by implementing the add trait on a struct
//you can overload operators by implementing traits associated with operators in std::ops
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//when using generic types, you can set a default with the following syntax
//<PlaceholderType=ConcreteType>
//ex:
//trait Add<Rhs=Self> {
//    type Output;
//
//    fn add(self, rhs: Rhs) -> Self::Output;
//}


pub trait Iterator {
    //associated type: Item
    //it is a stand in for the type of the values the type implementing the iterator trait is iterating over
    type Item;
    //implementers of this trait will specify the concrete type for Item and this function will return that
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    //here the type is implemented as a u32
    type Item = u32;
    //so an Option<u32> will be returned
    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        Some(4)
    }

}

/////////////////////////////////////////////////////////
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
/////////////////////////////////////////////////////////

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
//The trait a trait definition relies on is called a super trait of that trait.
//here Display is the super trait, the syntax is:
//trait 'trait name': 'super trait' {}
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point {}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    let person = Human;
    //calls the pilot fly method for person
    Pilot::fly(&person);

    //calls the wizard fly method for person
    Wizard::fly(&person);

    //calls the human fly method
    person.fly();

    //calls the default implementation
    println!("A baby dog is called a {}", Dog::baby_name());
    //calls the animal trait implementation on dog
    //this way disambiguates to tell compiler to use Animal that was implemented specifically for dog
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    //typical syntax looks like this:
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);


    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}


//newtype pattern: create a wrapper for the type and implement the trait on the wrapper
//useful to get around the orphan rule: can only implement a trait on a type if either are local to our crate
//downside is that it is a new type, so it doesn't include the methods of the values its holding
//in this example, a solution would be implementing the deref trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}