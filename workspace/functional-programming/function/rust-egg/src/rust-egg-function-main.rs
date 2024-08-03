#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-function_bin --bin rust-egg-function-main```
///
/// ```cargo doc  --package rust-egg-function_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-function_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// Make me compile! Execute `rustlings hint functions4` for hints :)
/// This store is having a sale where if the price is an even number, you get
/// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
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


fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
