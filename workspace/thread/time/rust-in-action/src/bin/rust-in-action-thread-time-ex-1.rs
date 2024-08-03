#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-time_bin --bin rust-in-action-thread-time-ex-1```
///
/// ```cargo doc  --package rust-in-action-thread-time_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-time_bin```
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

// #![feature(thread_id)]

use std::thread;
use std::time::Duration;
fn print_hello_with_delay(thread_id: u64) {
    let delay_ms = if thread_id == 0 { 100 } else { 90 + (10 * thread_id) };
    let delay = Duration::from_millis(delay_ms);
    thread::sleep(delay);
    println!("Hello from thread {}", thread_id);
}
fn main() {
    for i in 0..5 {
        let thread_id = i; // Directly use the loop variable
        thread::spawn(move || print_hello_with_delay(thread_id));
    }
}
