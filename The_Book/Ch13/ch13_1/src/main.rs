use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {

    //contains closure.. parameters used go in || and body goes after
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //its titled expensive because we don't need to be verbose by adding type annotations, this could be done with |num: u32| only
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    //shows how to simplify syntax from functio to closures
    fn  add_one_v1   (x: u32) -> u32 { x + 1 } //function
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; //fully annotated closure
    let add_one_v3 = |x|             { x + 1 }; //remove type annotations from closure definition
    let add_one_v4 = |x|               x + 1  ; //remove the brackets
    let y = add_one_v3(3);//Nothing within the closure body lets compiler deduce the type, so we need to call the closure variables otherwise the compiler complains with:
    let u = add_one_v4(3);//^ consider giving this closure parameter a type

    //compiler will infer ONE concrete type only! After you call it once with a string, it will always be required that x is a string, that is why n is commented out
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5);

    //closures capture values in three ways, take ownership, borrow immutably or mutably.
    //immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
//
    println!("Before calling closure: {:?}", list);
    only_borrows(); //grabs immutable reference
    println!("After calling closure: {:?}", list);

    //mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);//closure definition
//Between the closure definition and the closure call, an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow. 
    borrows_mutably();//closure call to grab mut reference
    println!("After calling closure: {:?}", list);




    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);//takes a FnMut closure Called multiple times, once for each item in the slice
    println!("{:#?}", list);

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

