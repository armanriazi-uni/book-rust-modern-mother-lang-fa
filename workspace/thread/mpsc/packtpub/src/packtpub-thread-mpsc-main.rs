#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-thread-mpsc_bin --bin packtpub-thread-mpsc-main```
///
/// ```cargo doc  --package packtpub-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-thread-mpsc_bin```
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
/// `100`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore


use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel using mpsc::channel
    let (tx, rx) = mpsc::channel::<i32>();

    // Spawn a new thread that receives values from the channel
    let receiver_handle = thread::spawn(move || {
        for value in rx {
            println!("Received value: {}", value);
        }
    });

    // Send values to the channel
    for i in 0..10 {
        tx.send(i).expect("Failed to send value");
    }

    // Close the sender side of the channel to allow the receiver to exit
    drop(tx);

    // Wait for the receiver thread to finish
    receiver_handle.join().expect("Receiver thread panicked");
}
