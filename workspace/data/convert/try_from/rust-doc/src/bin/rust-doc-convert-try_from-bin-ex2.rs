#![allow(dead_code, unused_variables, unused_imports)]

use std::convert::TryFrom;
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-convert-try_from_bin --bin rust-doc-convert-try_from-bin-ex2```
///
/// ```cargo test -q -p rust-egg-convert-try_from_bin --bin rust-doc-convert-try_from-bin-ex2```
///
/// ```cargo doc  --package rust-egg-convert-try_from_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-convert-try_from_bin```
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
/// 
fn main(){
    let big_number = 1_000_000_000_000i64;
    let smaller_number = big_number as i32;
    assert_eq!(smaller_number, -727379968);
    let try_smaller_number = i32::try_from(big_number);
    assert!(try_smaller_number.is_err());
    let try_successful_smaller_number = i32::try_from(3);
    assert!(try_successful_smaller_number.is_ok());
}