//! # Ch14_2
//!
//! `Ch14_2` is a test of a tutorial from 'the book'
//! :)

// the above doubleslash and exclamation syntax adds documentation to the item that
//contains the item

// three slash is for documentation comments
// they create html documentation that displays contents for public API items intended for programmers
// interested in knowning how to use your crate as opposed to how your crate is implemented
// they also support markdown notation!

//if you make examples in your documentation, they get counted as tests! if you run cargo test you can see the below from the documentation

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ch14_2::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}