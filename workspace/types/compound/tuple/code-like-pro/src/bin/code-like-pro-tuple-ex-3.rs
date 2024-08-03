#![allow(dead_code, unused_variables)]

/// code-like-pro-tuple-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-tuple_bin --bin code-like-pro-tuple-ex-3```
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
fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn main() {
    let a = 1f64;
    let b = 2i32;

    println!("{:?}", swap(a, b));
}
