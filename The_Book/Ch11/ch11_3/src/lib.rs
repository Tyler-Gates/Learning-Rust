

//unit tests are used to test each unit of code in isolation from the rest of the code to pinpint where code is and isn't working
//they always go in the src directory in each file with the code that they're testing and the convention is to put them in a module
//titled tests and to annotate the module with cfg(test) like below
//the cfg(test) tells compiler to only compile when running cargo test -- cfg stands for configuation the (x) tells it to only run when x is included: cargo x
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

//and can test public functions
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

//can test private functions
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}