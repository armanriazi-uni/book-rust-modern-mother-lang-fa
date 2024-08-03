#![allow(dead_code, unused_variables)]

/// ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-tuple_bin --bin rust-doc-tuple-ex-2```
///
/// ```cargo doc  --package rust-doc-tuple_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-tuple_bin```
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
