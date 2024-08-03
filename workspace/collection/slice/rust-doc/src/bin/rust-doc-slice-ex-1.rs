#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-slice_bin --bin rust-doc-slice-ex-1```
///
/// ```cargo doc  --package rust-doc-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-slice_bin```
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
/// 
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[..2];
    let slice2 = &s[3..];
    let slice3 = &s[..];
    println!("Printed:{:?}",slice3);


}
