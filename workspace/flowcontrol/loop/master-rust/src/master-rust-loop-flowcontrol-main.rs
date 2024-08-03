#![allow(dead_code, unused_variables)]
use std::time::{Duration, Instant};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p master-rust-loop-flowcontrol_bin --bin master-rust-loop-flowcontrol-main```
///
/// ```cargo doc  --package master-rust-loop-flowcontrol_bin --message-loopmat short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-loop-flowcontrol_bin```
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
/// `20`
///
/// ## Example
/// // ```rust,compile_fail,ignore
fn main() {
    let mut i = 0;
    let counter = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };
    println!("{}", counter);
}
