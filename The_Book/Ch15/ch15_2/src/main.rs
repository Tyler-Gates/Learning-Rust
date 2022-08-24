use std::ops::Deref;


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    assert_eq!(5, x);
    //the * follows references to the values they point at
    assert_eq!(5, *y);
    //can use Box<T> the same way because it has the dereference trait
    //The box copies the value of x then points to the new location of the copied value
    //unlike above where it points to the location of x
    assert_eq!(5, *z);

    //because we implemented deref it allows us to utilize 'deref coercion' which can turn &MyBox<String> into &String
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

//to override * operator on mutable references, implement the trait DerefMut, it works the same as Deref but with mutable instead of immutable

// Rust does deref coercion when it finds types and trait implementations in three cases:

// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

//The third only works one way, from mutable to immutable, not the other way around