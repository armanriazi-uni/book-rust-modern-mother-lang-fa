#![allow(dead_code, unused_variables)]

/// code-like-pro-tuple-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-tuple_bin --bin code-like-pro-tuple-ex-2```
///
/// ```cargo doc  --package code-like-pro-tuple_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-tuple_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your tuple/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
struct Location(i32, i32);

fn main() {
    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}
