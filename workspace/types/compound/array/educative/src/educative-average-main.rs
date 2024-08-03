#![allow(dead_code, unused_variables)]



/// Main
///
/// # Commands
///
/// ```cargo run -q -p educative-average_bin --bin educative-average-main```
///
///
/// ```cargo doc  --package educative-average_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-average_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]`
///
/// ## Example
/// `let array = [0i8; 33];`
/// > Result:
/// `error[E0277]: arrays only have std trait implementations for lengths 0..=32`
fn main(){
let array = [0i8; 32];
println!("{:?}", array);
}
