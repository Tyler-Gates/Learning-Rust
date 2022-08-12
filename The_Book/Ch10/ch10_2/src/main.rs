use aggregator::{Summary,Tweet,NewsArticle};


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    
}

//This parameter accepts any type that implements the specified trait.
//this implementation is syntax sugar for the below function parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}


//when doing it this way item1 and item2 can be different types, they just must have the associated trait
//ex:pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//ex: pub fn notify(item: &(impl Summary + Display)) {

//When doing this way item1 and item2 must be of the same type
//ex:pub fn notify<T: Summary>(item1: &T, item2: &T) {
//pub fn notify<T: Summary + Display>(item: &T) {

//plus syntax works when specifying multiple trait bound

//can use clearer syntax to dictate trait bounds with WHERE clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
vs. 
fn some_other_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

//can also do this for return values, just has to have the summary trait and only have the option to return one type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

//conditionally implement methods based on if they have certain traits! 
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}