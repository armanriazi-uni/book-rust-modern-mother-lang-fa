#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-generic_bin --bin master-rust-types-generic-main```
///
/// ```cargo doc  --package master-rust-types-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-types-generic_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Announcement! Today is someone's birthday!`
/// `The longest string is abcd`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
fn main() {
    // providing a type
    let v1: Vec<u8> = Vec::new();

    // or calling method
    let mut v2 = Vec::new();
    v2.push(2);    // v2 is now Vec<i32>

    // or using turbofish
    let v3 = Vec::<u8>::new();    // not so readable
}