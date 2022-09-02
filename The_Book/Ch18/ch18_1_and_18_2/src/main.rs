fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    //if let, else if, else if let conditionals!
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }



    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    //introducing while let conditional loop!
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    //for loops!
    //here we can destructure a tuple!
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //let PATTERN = EXPRESSION;
    let x = 5;
    let (x, y, _) = (1, 2, 3);

//18.2
    //this line will fail because we are using a refutable pattern where rust requires an irrefutable pattern
    //let Some(x) = some_option_value;

    //we can modify the above to use the right pattern
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    
}
