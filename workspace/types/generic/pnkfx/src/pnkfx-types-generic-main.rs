#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p pnkfx-types-generic_bin --bin pnkfx-types-generic-main```
///
/// ```cargo doc  --package pnkfx-types-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pnkfx-types-generic_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Announcement! Today is someone's birthday!`
/// `The longest string is abcd`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
// Example 11: enums

// (Can you believe we got this far without "real" Algebraic Data Types?)

enum Request<X> {
    PlayOutsideWith(X),
    GoSwimming,
    LookAtStars,
    EatLunch,
}
#[derive(Debug)]
enum Answer<X> {
    Yes(X),
    No(X),
    Maybe,
}
struct Parent;

fn main() {
    let parent = Parent;
    let requests : [Request<String>; 3] = [Request::GoSwimming,
                                           Request::LookAtStars,
                                           Request::EatLunch];
    for request in requests.iter() {
        match parent.can_i(request) {
            Answer::Yes(_) => {
                println!("Hooray!");
                return;
            }
            Answer::No(why) => {
                println!("I do not believe you when you say {}!", why);
            }
            Answer::Maybe => {
                println!("Well, how about ...");
            }
        }
    }
}
fn print_it( input: impl std::fmt::Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}
impl<X> Answer<X> {
    fn map<Y, F>(self, f: F) -> Answer<Y> where F: Fn(X) -> Y {
        unimplemented!();
    }
}
impl Parent {
    fn can_i<X>(&self, r: &Request<X>) -> Answer<&str> //Answer<String>  
    {
        let answer = match r {
            Request::LookAtStars => Answer::Maybe,
            Request::GoSwimming |
            Request::PlayOutsideWith(_) => Answer::No("we live in on Mars"),
            Request::EatLunch => Answer::Yes("it is close to noon"),
        };

        answer
    }
}

// EXERCISE: code does not compile; fix it somehow.

// EXERCISE: implement `Answer::map`
