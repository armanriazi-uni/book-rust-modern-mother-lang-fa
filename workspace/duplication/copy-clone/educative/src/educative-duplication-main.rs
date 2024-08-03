#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-duplication_bin --bin educative-duplication-main```
///
/// ```cargo doc  --package educative-duplication_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-duplication_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
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
/// Two way attr or impl of clone #[derive(Clone)]
struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn clone(&self) -> Fruit {
        Fruit {
            apples: self.apples.clone(),
            bananas: self.bananas.clone(),
        }
    }
}

// Should use a reference, but I'm proving a point
fn print_fruit(fruit: Fruit) {
    println!("Apples: {}, bananas: {}", fruit.apples, fruit.bananas);
}

fn main() {
    let mut fruit = Fruit { apples: 5, bananas: 10 };
    print_fruit(fruit.clone()); // moved here
    fruit.apples *= 2; // without impl clone this will fail
    print_fruit(fruit.clone());
}
