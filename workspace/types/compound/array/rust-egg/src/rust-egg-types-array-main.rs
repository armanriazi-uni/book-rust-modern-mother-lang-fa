#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-types-array_bin --bin rust-egg-types-array-main```
///
/// ```cargo doc  --package rust-egg-types-array_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-types-array_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between arraying-point values.
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
// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = 0..100;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
