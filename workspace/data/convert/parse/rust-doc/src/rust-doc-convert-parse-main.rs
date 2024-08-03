  #![allow(dead_code, unused_variables, unused_imports)]
 use num::{Float};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-convert-parse_bin --bin rust-doc-convert-parse-main```   
///
/// ```cargo test -q -p rust-doc-convert-parse_bin --bin rust-doc-convert-parse-main``
///
/// ```cargo doc  --package rust-doc-convert-parse_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-convert-parse_bin```
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
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`

use std::convert::From;
#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}