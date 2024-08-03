#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-float_bin --bin rust-in-action-types-float-main```
///
/// ```cargo doc  --package rust-in-action-types-float_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-float_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between floating-point values.
/// These tolerances are defined as f32::EPSILON and f64::EPSILON. To be more precise,
/// it’s possible to get closer to how Rust is behaving under the hood, as the following small example shows.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Alphabetical!'
/// `Numerical!`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    // Characters (`char`)

    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '1';
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
