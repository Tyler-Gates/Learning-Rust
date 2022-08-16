
//$ cargo test -- --test-threads=1
//allows you to test each function one at a time, needed when tests use share state


fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);    //returns the println! from called function as well as error message when failing
    //     assert_eq!(10, value);/
    // }


    //to run this ignored test, include -- --ignored 
    //'cargo test -- --ignored' will run only all ignored tests
    // or 'cargo test -- --include-ignored' to include them with other tests
    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);//does not include println! because test passed
        assert_eq!(5, value);
    }
    //if we want to see printed values of even passed tests do command: 
    //$ cargo test -- --show-output


    // to run subsets you can include keywords
    //ex: 'cargo test run' will run add_two_and_two and add_three_and_two
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
