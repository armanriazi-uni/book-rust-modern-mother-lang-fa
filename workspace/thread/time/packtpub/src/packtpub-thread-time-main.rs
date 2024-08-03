#![allow(dead_code, unused_variables)]

use std::thread;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-thread-time_bin --bin packtpub-thread-time-main```
///
/// ```cargo doc  --package packtpub-thread-time_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-thread-time_bin```
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

///
/// ```compile_fail,ignore
use std::thread::{spawn,sleep};
use std::time::Duration;
fn main() {
    let h = spawn(|| {
        for i in 0..10 {
            sleep(Duration::from_millis(10));
            println!("{}",i);
        }
        return 5;
    });
    for i in 10..20 {
        sleep(Duration::from_millis(20));
        println!("{}",i);
    }
    let r = h.join().unwrap();
    println!("Done R = {}",r);
}
