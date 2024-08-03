#![allow(dead_code, unused_variables)]
use std::time::{Duration, Instant};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-while-flowcontrol_bin --bin rust-in-action-while-flowcontrol-main```
///
/// ```cargo doc  --package rust-in-action-while-flowcontrol_bin --message-whilemat short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-while-flowcontrol_bin```
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
/// `15752536 changed based on time`
///
/// ## Example
/// // ```compile_fail,ignore


 fn main() {
    let mut count = 0;
    let time_limit = Duration::new(3,0);
    println!("{:?}", time_limit);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
 }
