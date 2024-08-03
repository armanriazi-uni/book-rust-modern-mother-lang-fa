#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-while_bin --bin educative-while-main```
///
/// ```cargo doc --package educative-while_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package educative-while_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `while true{...}` Doing this generates a warning
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your while/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
fn main() {
    let mut iter = 1..10;
    while true {
        match iter.next() {
            None => break,
            Some(x) => println!("{}", x),
        }
    }
}
