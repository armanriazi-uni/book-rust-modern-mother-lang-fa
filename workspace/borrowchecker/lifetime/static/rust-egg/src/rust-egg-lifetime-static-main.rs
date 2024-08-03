#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-lifetime-static_bin --bin rust-egg-lifetime-static-main```
///
/// ```cargo doc  --package rust-egg-lifetime-static_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-lifetime-static_bin```
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
/// `favorite snacks: Apple and Cucumber`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
///
// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {
    pub use self::fruits::APPLE as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    //&str	= x.to_string()
    //'a T = type constructor
    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}

