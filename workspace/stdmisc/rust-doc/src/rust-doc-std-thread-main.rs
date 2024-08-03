#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin --bin rust-doc-std-thread-main```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin```
///
/// ## What
/// `Thread`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```
/// this is thread number 1
/// tthis is thread number 9
/// tthis is thread number 0
/// tthis is thread number 4
/// tthis is thread number 7
/// tthis is thread number 2
/// tthis is thread number 5
/// tthis is thread number 3
/// tthis is thread number 8
/// tthis is thread number 6
/// ```
/// ## Example
///

use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
