#![allow(dead_code, unused_variables)]

/// master-rust-semantic-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-semantic_bin --bin  master-rust-semantic-bin-ex-2```
///
/// ## What
/// `/// `primitives do not requiring impl Copy trait``
///
/// ## How
/// `The ref keyword is a keyword that can match items by taking a reference to them instead of capturing them by value`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `I got cake`
/// `Bag { food: Cake }`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore
// ownership_match.rs

#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad
}
#[derive(Debug)]
struct Bag {
    food: Food
}
fn main() {
    let bag = Bag { food: Food::Cake };
    match bag.food {
        Food::Cake => println!("I got cake"),
        ref a => println!("I got {:?}", a)
    }
    println!("{:?}", bag);
}
