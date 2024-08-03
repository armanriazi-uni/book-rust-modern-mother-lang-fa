#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p eduactive-arithmetic_bin --bin eduactive-arithmetic-ex-1```
///
/// ```cargo doc  --package eduactive-arithmetic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package eduactive-arithmetic_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Variable x was repeated without need to be mut`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `15`
///
/// ## Example
///  ` let answer = 6 / 2 + 4 * 3;`
///
/// //```rust,compile_fail,no_run,ignore
fn main() {
    let x = 5 + 3;
    let x = x * 2;
    let x = x - 6;
    let x = x / 2;
    println!("The answer is {}", x);
}
