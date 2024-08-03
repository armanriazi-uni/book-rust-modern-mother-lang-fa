#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-slice_bin --bin educative-slice-ex-1```
///
/// ```cargo doc  --package educative-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-slice_bin```
///
/// ## What
/// `Slice pattern matching`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //rust,compile_fail,no_run,ignore

fn main() {
let point = [1.22, 3.3];
let [x, y] = point;
println!("X: {}", x);
println!("Y: {}", y);
}
