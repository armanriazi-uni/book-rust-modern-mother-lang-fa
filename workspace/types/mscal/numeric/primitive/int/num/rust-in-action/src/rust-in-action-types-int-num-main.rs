#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-int-num_bin --bin rust-in-action-types-int-num-main```
///
/// ```cargo doc  --package rust-in-action-types-int-num_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-int-num_bin```
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
/// `unimplement`
///
/// ## Example
/// //```rust,compile_fail,ignore
///extern crate num;
///
///use num::{Integer, Unsigned};
///
///fn fibonacci<T: Integer + Unsigned>(n: T) -> T {
///  match n {
///    0 => 0,
///    1 => 1,
///    _ => n + fibonacci(n-1),
///  }
///}
///
///fn main() {
///    let n = 10;
///    println!("{}", fibonacci(n as u16));
///    println!("{}", fibonacci(n as u64));
///}
/// ```

fn main() {
    unimplemented!()
}
