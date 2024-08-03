#![allow(dead_code, unused_variables)]

use std::error::Error;

/// rip-error-handling-ex-3
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rip-error-handling_bin --bin rip-error-handling-ex-3```
///
/// ```cargo doc  --package rip-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rip-error-handling_bin ```
///
/// ## What
/// `error-conversion`
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
///
fn main(){
// let orig_error = call_returning_error();

// // Use an Option<&Error>. This is the return type of Error.cause().
// let mut err = Some(&orig_error as &dyn Error);

// // Print each error's cause until the cause is None.
// while let Some(e) = err {
//     println!("{}", e);
//     err = e.cause();
// }

unimplemented!();

}
