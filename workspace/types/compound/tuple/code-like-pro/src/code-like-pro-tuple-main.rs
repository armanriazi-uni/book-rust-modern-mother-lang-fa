#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-tuple_bin --bin code-like-pro-tuple-main```
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
fn main() {
    let tuple = (1, 2, 3);//or : let (t0,t1,t2) = (1, 2, 3);
    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    match tuple {
        (one, two, three) => println!("{}, {}, {}", one, two, three),
    }
    let (one, two, three) = tuple;
    println!("{}, {}, {}", one, two, three);
}
