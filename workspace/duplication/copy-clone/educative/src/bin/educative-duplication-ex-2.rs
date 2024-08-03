#![allow(dead_code, unused_variables)]

use std::clone;

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-duplication_bin --bin educative-duplication-ex-2```
///
/// ```cargo doc  --package educative-duplication_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-duplication_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// using only `#[derive(Copy)]` make error compiler because Copy is a subtrait of Clone
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
#[derive(Clone)]
struct Fruit<T> {
    apples: T,
    bananas: T,
}

impl<T: Copy> Copy for Fruit<T> {}
// Should use a reference, but I'm proving a point
fn print_fruit(fruit: Fruit<i32>) {
    println!("Apples: {}, bananas: {}", fruit.apples, fruit.bananas);
}

fn main() {
    let mut fruit = Fruit { apples: 5, bananas: 10 };
    print_fruit(fruit);
    fruit.apples *= 2;
    fruit.bananas *= 3;
    print_fruit(fruit);
}
