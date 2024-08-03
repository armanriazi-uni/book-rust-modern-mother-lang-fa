#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-types-generic_bin --bin rust-egg-types-generic-main```
///
/// ```cargo doc  --package rust-egg-types-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-types-generic_bin```
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
/// `None`
///
/// ## Example
/// //```rust,compile_fail,ignore
///

// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
