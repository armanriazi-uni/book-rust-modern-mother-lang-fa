#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-tuple_bin --bin rust-in-action-types-tuple-main```
///
/// ```cargo doc  --package rust-in-action-types-tuple_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-tuple_bin```
///
/// ## What
/// primitive_types_tuple
///
/// ## How
// Use a tuple index to access the second element of `numbers`.
// You can put this right into the `println!` where the ??? is.
// Execute `rustlings hint primitive_types` for hints!
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `The second number is 2`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let numbers = (1, 2, 3);
    println!("The second number is {}", numbers.1);
}
