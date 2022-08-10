use std::collections::HashMap;
use std::io;

fn main() { //      1, 2, 3, 4, 5, 6, 9, 9, 10
    let vec = vec!( 1, 2, 9, 6, 4, 3, 5, 9, 10);
    let med_mod = median_mode(vec);
    println!("{:#?}", med_mod);
    let string = "Greetings from the internets, My name is Tyler and I am here to learn rust and chew bubble gum";
    let oink = convert_to_pig_latin(string.to_string());
    println!("{}", oink);
    company_employees_cli() 
}

//get median and mode from a vector
fn median_mode(mut list: Vec<u32>) -> (u32,u32) {
    //get median
    list.sort();
    let median = list[list.len()/2];

    //get mode
    let mut map = HashMap::new();
    for i in &list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut current = 0;
    let mut mode = 0;
    for (k,v) in &map {
        if v > &current {
            current = *v;
            mode = **k;
        }
    }
    //return tuple of median and mode
    (median, mode)
}


//must be english!!!!
fn convert_to_pig_latin(string: String) -> String{
    let vowels = vec!("a", "e", "i", "o", "u");
    let string = string.to_lowercase();
    let mut newstring = String::new();
    for word in string.split_whitespace() {
        if vowels.contains(&&word[0..1]) {
            newstring = format!("{} {}-hay", newstring, &&word[1..]);
        }
        else {
            newstring = format!("{} {}-{}ay", newstring, &&word[1..], &&word[0..1]);
        }
    }
    newstring
}

//encapsulates everything inside the function for exercise purposes
//creates a text interface to add users to departments and retrieve lists based on departments
fn company_employees_cli() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        let mut department = String::new();
        let mut employee = String::new();
        println!("please insert command! (1. Get a Department / 2. Add employee to Department / 3. Get all Departments / 4. exit)");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("{:#?}", input);
        let choice: u32 = input.trim().parse().expect("Please enter a number!");
        match choice {
            1 => {
                println!("Please Enter Department name!");
                input.clear();
                io::stdin().read_line(&mut input);
                println!("{:#?}", input);
                if map.contains_key(&input[0..input.len() - 2].to_string()) {
                    let department = match map.get(&input[0..input.len() - 2].to_string()) {
                        Some(x) => x,
                        none => continue,
                    };
                    println!("Here is a list of eemployees of {} department", input);
                    println!("{:#?}", department );
                }
                else {
                    println!("That department doesn't exist!!");
                }
            },
            2 => {
                println!("Please enter Department name!");
                io::stdin().read_line(&mut department);
                println!("Please enter employee name!");
                io::stdin().read_line(&mut employee);
                if !map.contains_key(&department[0..department.len() - 2].to_string()){ 
                    let employee = map.insert(department[0..department.len() - 2].to_string(), vec![employee[0..employee.len() - 2].to_string()]);
                }
                else {
                    let employees = map.entry(department[0..department.len() - 2].to_string()).and_modify(|e| e.push(employee[0..employee.len() - 2].to_string()));
                }
            },
            3 => {
                for (k,v) in &map {
                    println!("Department: {:#?}", k);
                    println!("Employees: {:#?}", v);
                }
            },
            4 => {
                return;
            },
            _ => ()
        }
    }

}