#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-slice_bin --bin rust-egg-slice-main```
///
/// ```cargo doc  --package rust-egg-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-slice_bin```
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
   let a = [1,2,3,4];
    let nice_slice = &a[1..4];
    assert_eq!([2,3,4], nice_slice);
}
