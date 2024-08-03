#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-dispatchable_bin --bin rust-doc-trait-dispatchable-main```
///
/// ```cargo doc  --package rust-doc-trait-dispatchable_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-dispatchable_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// > two kinds of methods within a trait:
/// > self is just a type alias to Self,which refers to the type on which the trait is being implemented
/// > Sample Associated method: pause, Instance methods: play
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
///


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

}

impl Summary for NewsArticle {


    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}

//use aggregator::{Summary, Tweet};

//**fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { */
/*
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
*/
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    notify(&tweet);
    println!("New article available! {}", article.summarize());
}
