
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //no tuples/structs
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    //tuple strat grouped but no names for elements, only .0 and .1
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tup(rect1)
    );
    //struct strat grouped and named elements, provides ultimate clarity and readability

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    //using debug via :?
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect3 is {:#?}", rect3);

    //dbg!
    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect4);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
