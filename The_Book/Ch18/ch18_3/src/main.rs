struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageNested {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum MessageBinding {
    Hello { id: i32 },
}

fn main() {
    let x = Some(5);
    let y = 10;

    match x { //a new scope is created here
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // y is instantiated in a new scope, so it doesnt adhere to y = 10 from earlier, it shadows the outer scope.
                                                 // it will take the value inside of Some(), which is exactly what x is. so it's value is 5, the pattern matches, and it prints
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    //an example of the or operator | being used to match multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //an example of the ..= syntax. Allows for you to match against a range of values.
    //this is only acceptable with numeric or char values
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    //destructuring structs
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    //Rust has a shorthand for patterns that match struct fields: 
    let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //pattern matching by destructing struct
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    //destructing enums
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    //example of destructing nested enums
    let msg = MessageNested::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        MessageNested::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        MessageNested::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    //can mix, match, and nest destructuring patterns:
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    //3 will be ignored in this function
    foo(3, 4);
    

    // here we use nested _ because we don't need to know the values, only if they are both present, or aren't
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);


    //can also use _ like this:
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    //if a variable has a _ infront of it, the compiler will not complained about unused variable(good if not using yet)
    // these variables still bind and can possibly take ownership of a value
    let _x = 5;
    //where as the below won't bind, because _ by itself doesn't bind to values or take ownership
    let _ = 4;


    //you can ignore remaining parts of a value with ..
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    //can also sandwich irrelevant data by putting .. between two points
    //no reverse sandwiching ".., second, .."
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    //match guards allow for additional conditionals
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }


    //pattern matching here allows for us to solve our pattern solving problem
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);


    //can also use the | operator with match guards
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }


    //here we use the @ binding, it allows us to assign a variable to what we are pattern match testing
    let msg = MessageBinding::Hello { id: 5 };
    match msg {
        MessageBinding::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        MessageBinding::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageBinding::Hello { id } => println!("Found some other id: {}", id),
    }

}

//here we use _ to ignore the x parameter.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}