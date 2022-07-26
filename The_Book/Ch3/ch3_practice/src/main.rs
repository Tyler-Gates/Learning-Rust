use std::io;

fn main() {
    loop {
    println!("Choose Option\n1. Celsius to Fahrenheit\n2. Fahrenheit to Celsius\n3. Generate nth Fibonacci number\n4. The Twelve Days of Christmas\n5. Exit");
    let mut choice = String::new();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a number!");

    let temp = if choice == 1 {"Celsius"} else {"Fahrenheit"};

    if choice == 1 || choice == 2{

        println!("Please Input {temp} Temperature");

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
        let input: f32 = input.trim().parse().expect("Please Enter a number!");

        if choice == 1 {
            let input: f32 = celsius_to_fahrenheit(input);
            println!("Converted to Fahrenheit: {input}");
        }
        else {
            let input: f32 = fahrenheit_to_celsius(input);
            println!("Converted to Celsius: {input}");
        }
    }
    if choice == 3 {
        println!("Please Input nth Fibonacci number(non zero)");

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
        let input: u32 = input.trim().parse().expect("Please Enter a number!");
        let ans = generate_fib(input);
        println!("The {input} fib number is {ans}")
    }
    if choice == 4 {
        christmas_function();
    }
    if choice == 5 {
        return
    }
    }
}

fn celsius_to_fahrenheit(x: f32) -> f32{
    x * 1.8 + 32.0
}

fn fahrenheit_to_celsius(x: f32) -> f32{
    (x - 32.0) * (5.0/9.0)
}

fn generate_fib(mut x: u32) -> u32 {
    if x == 1 || x == 2 {
        return 1
    }
    let mut back = 1;
    let mut front = 1;
    let mut ans = 0;
    while x > 2 {
        ans = back + front;
        back = front;
        front = ans;
        x=x-1;
    }
    ans
}

fn christmas_function() {
    let lyrics = ["A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"];

    for number in 1..13 {
        println!("On the {number} day of Christmas, my true love sent to me");
        for line in (0..number).rev() {
            println!("{}", lyrics[line]);
        }
    }
}