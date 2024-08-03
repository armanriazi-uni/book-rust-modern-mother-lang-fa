#![allow(dead_code, unused_variables)]
use rand::{random};
static mut ERROR: isize = 0;

struct File;

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p master-rust-error-handling_bin --bin master-rust-error-handling-main```
///
/// ```cargo doc  --package master-rust-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-error-handling_bin ```
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
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let incremented_value = match map.get("one") {
        Some(val) => val + 1,
        None => 0
    };
    println!("{}", incremented_value);
}
