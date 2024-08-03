#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p eduactive-arithmetic_bin --bin eduactive-arithmetic-main```
///
/// ```cargo doc  --package eduactive-arithmetic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package eduactive-arithmetic_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// ```assert_eq!(25u8.wrapping_mul(12), 44);```
/// 25*12=400-256= 44
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
    let answer = 6 / 2 + 4 * 3;
    println!("The answer is: {}", answer);
}
