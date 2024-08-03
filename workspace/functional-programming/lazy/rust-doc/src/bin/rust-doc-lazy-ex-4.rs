#![allow(dead_code, unused_variables)]
/// rust-doc-lazy-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-lazy_bin --bin rust-doc-lazy-ex-4```
///
/// ```cargo doc  --package rust-doc-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lazy_bin ```
///
/// ## What
/// `GlobalState`
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
use std::sync::{Arc, Mutex};
use std::thread;

// Define the lazy static Mutex
lazy_static! {
    static ref ITEMS: Mutex<Vec<u64>> = {
        let mut v = Vec::new();
        v.push(9);
        v.push(2);
        v.push(1);
        Mutex::new(v)
    };
}

fn main() {
    unimplemented!();
    // // Create an Arc to share ownership of the Mutex across threads
    // let items = Arc::clone(&ITEMS);

    // // Spawn multiple threads to demonstrate concurrent access
    // for _ in 0..10 {
    //     let items_clone = Arc::clone(&items);
    //     thread::spawn(move || {
    //         // Safely add an item to the vector
    //         let mut num_items = items_clone.lock().unwrap();
    //         num_items.push(42); // Adding a new item
    //         // Print all items in the vector
    //         println!("Items: {:?}", *num_items);
    //     });
    // }

    // // Wait for all threads to finish
    // for _ in 0..10 {
    //     thread::park();
    // }
}
