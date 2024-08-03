#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-int_bin --bin rust-in-action-types-int-main```
///
/// ```cargo doc  --package rust-in-action-types-int_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-int_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `23`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let x = 21;
    let y = -2;
    let z = add_abs(x, y);
    println!("Add abs:{:?}", z);
}

fn absolute(a: i32) -> i32 {
    if a < 0 {
        return -a;
    }
    a
}

fn add_abs(a: i32, b: i32) -> i32 {
    let c = absolute(a) + absolute(b);
    c
}
