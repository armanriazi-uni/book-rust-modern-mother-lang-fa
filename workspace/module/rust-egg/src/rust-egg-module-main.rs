#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-egg-module_bin --bin rust-egg-module-main```
///
/// ```cargo doc  --package rust-egg-module_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-module_bin ```
///
/// ## What
/// `macro-share behavior`
///
/// ## How
/// The macro above takes a list of parameters in $( $x:expr ),*, then it uses them in a loop to sum all the costs of the products provided.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// //```rust


mod delicious_snacks {
    //Relative mods by self
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

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
        //Absloute mods are default
        "favorite snacks: {} and {}",
        crate::delicious_snacks::fruit,
        crate::delicious_snacks::veggie
    );
}
