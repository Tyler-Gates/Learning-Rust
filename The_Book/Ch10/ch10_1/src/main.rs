fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
//can combine above to make a generic 


fn generic_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//only when both x and y are f32 types
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = generic_largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = generic_largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5.0, y: 10.0 };
    let float = Point { x: 1.0, y: 4 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());


    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}