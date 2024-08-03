#![allow(dead_code, unused_variables)]

use std::convert::TryInto;

/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-convert-try_into_bin --bin rust-in-action-convert-try-into-main```
///
/// ```cargo doc  --package rust-in-action-convert-try_into_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-convert-try_into_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// The unwrap() method can handle the success value and returns the value of b as an i32 here.
/// If the conversion between u16 and i32 were to fail, then calling unsafe() would crash the program
///
/// # Arguments
///
/// * `Arg1` - This is the integer number to verb the struct-name
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`
///


 fn main() {
   let a: i32 = 10;
   let b: u16 = 100;

   let b_ = b.try_into()
             .unwrap();

   if a < b_ {
     println!("Ten is less than one hundred.");
   }
 }
