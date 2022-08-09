

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v);
        // do stuff with v
    } // <- v goes out of scope and is freed here
    println!("{:?}", v);

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        //v.push(6); this line breaks it. cant have immutable and mutable references in the same scope

        println!("The first element is: {}", first);
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }
    
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
