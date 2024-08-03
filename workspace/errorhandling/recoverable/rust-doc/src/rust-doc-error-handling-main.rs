#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-error-handling_bin --bin rust-doc-error-handling-main```
///
/// ```cargo doc  --package rust-doc-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-error-handling_bin ```
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
///

use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("./hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("./hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
