#[cfg(test)]
mod tests {
    use super::*;
    //can use Result so tests don't panic when they fail
    //enables you to use the question mark operator in the body of tests which is a connvenient way to write tests that hosuld fail if any operation within them returns an Err variant
    // To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    //can add custom outputs by using next parameter as format! and can even use variables
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    //adding #[should_panic] allows us to test cases where we know the code should fail so we can handle error conditions
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

}
//for structs and enums we define ourselves we need to add #[derive(PartialEq, Debug)] these traits

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}