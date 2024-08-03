#![allow(dead_code, unused_variables)]

/// rust-in-action-tuple-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-tuple_bin --bin rust-in-action-tuple-bin-ex-1```
///
/// ## What
/// `Destructure`
///
/// ## How
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types` for hints!
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Furry McFurson is 3.5 years old.`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
