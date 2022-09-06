fn add_one(x: i32) -> i32 {
    x + 1
}

//function pointer in parameter
//fn type is called function pointer
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

enum Status {
    //enum variants are also initializer functions
    Value(u32),
    Stop,
}

//we can use these as function pointers, they also implement the closure traits, which means we can specify the initializer function
//as arguments for methods that take closures
let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();


//closures can't be returned on their own
//ex doesnt work:  fn returns_closure() -> dyn Fn(i32) -> i32 {
    //|x| x + 1
//}

//this works because we wrapped them in a pointer aka trait object
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}