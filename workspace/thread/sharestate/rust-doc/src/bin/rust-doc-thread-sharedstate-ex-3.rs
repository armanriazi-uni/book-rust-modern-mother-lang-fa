#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-thread-sharedstate_bin --bin rust-doc-thread-sharedstate-ex-3```
///
/// ```cargo doc  --package rust-doc-thread-sharedstate_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-thread-sharedstate_bin```
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
/// `nothig`
///
/// ## Example
///
///
/// ```compile_fail,ignore

use std::sync::{Arc, RwLock};
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn get_value(&self) -> i32 {
        self.value.clone()
    }
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    // Create a shared counter
    let counter = Arc::new(RwLock::new(Counter::new()));
    // Spawn some threads to read and write to the counter
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num_reads = 0;
            // Perform 10 reads
            for _ in 0..10 {
                let read_result = counter_clone.read().unwrap();
                //assert_eq!(read_result.get_value(), 0);
                num_reads += 1;
            }
            println!("Thread finished {} reads", num_reads);
            // Perform 1 write
            let mut write_result = counter_clone.write().unwrap();
            write_result.increment();
            println!("Thread finished 1 write");
        });
        handles.push(handle);
    }
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    // Check the final value of the counter
    let final_counter = counter.read().unwrap();
    println!("Final counter value: {}", final_counter.get_value());
}
