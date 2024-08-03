#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// educative-rust-trait-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p educative-trait-bound_bin --bin  educative-rust-trait-ex-6```
///
/// ## What
/// `TODO`
///
/// ## How
/// `move || a + b` to use other thread
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Double for Fruit {
    fn double(&self) -> Self {
        Fruit {
            apples: self.apples.double(),
            bananas: self.bananas.double(),
        }
    }
}

fn main() {
    let fruit = Fruit {
        apples: 5,
        bananas: 10,
    };

    println!("Apples plus bananas is {} and then doubled, apples plus bananas is {}.", fruit.apples + fruit.bananas, fruit.apples.double() + fruit.bananas.double());
}
