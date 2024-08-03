#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-slice_bin --bin educative-slice-main```
///
/// ```cargo doc  --package educative-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-slice_bin```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //rust,compile_fail,no_run,ignore

fn main() {
let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let slice = &array[1..2];
println!("Slice: {:?}", slice);
let slice = &array[..3];
println!("Slice: {:?}", slice);
let slice = &array[5..];
println!("Slice: {:?}", slice);
let slice = &array[..0];
println!("Slice: {:?}", slice);
}
