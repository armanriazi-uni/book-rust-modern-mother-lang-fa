#![allow(dead_code, unused_variables)]
use rand::random; // <1>
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-data-random_bin --bin rust-in-action-data-random-main```
///
/// ```cargo doc  --package rust-in-action-data-random_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-data-random_bin ```
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


fn main() {
    println!("{} is a random number", rand::random::<u32>());
}
