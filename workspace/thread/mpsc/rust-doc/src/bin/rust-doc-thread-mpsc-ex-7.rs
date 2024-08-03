#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-thread-mpsc_bin --bin rust-doc-thread-mpsc-ex-7```
///
/// ```cargo doc  --package rust-doc-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-thread-mpsc_bin```
///
/// ## What
/// `thread-channel-multi-message`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothig`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore

use std::thread;
use std::time::Duration;

fn main() {
    // Create the first thread
    let handle = thread::spawn(|| {
        println!("Thread 1 starting");

        // Simulate some work by sleeping
        thread::sleep(Duration::from_millis(100));

        // Yield control to other threads
        thread::yield_now();

        println!("Thread 1 after yielding");
    });

    // Create the second thread
    let _handle2 = thread::spawn(|| {
        println!("Thread 2 starting");

        // Wait for the first thread to yield
        thread::park();

        println!("Thread 2 after waiting");
    });

    // Wait for both threads to finish
    handle.join().unwrap();
}
