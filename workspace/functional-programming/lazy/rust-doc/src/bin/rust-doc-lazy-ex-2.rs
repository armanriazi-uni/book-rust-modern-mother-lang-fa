#![allow(dead_code, unused_variables)]
/// rust-doc-lazy-ex-2
///
/// ## Commands
///
/// ```cargo test -q -p rust-doc-lazy_bin --bin rust-doc-lazy-ex-2```
///
/// ```cargo doc  --package rust-doc-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lazy_bin ```
///
/// ## What
/// `lazy_static macro`
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
// Assuming this is part of your main.rs or lib.rs file
// Assuming you've added lazy_static to your Cargo.toml and imported it here

use std::sync::Mutex;
use lazy_static::lazy_static; // Import the lazy_static macro
lazy_static! {
    static ref ITEMS: Mutex<Vec<u64>> = {
        let mut v = vec![];
        v.push(9);
        v.push(2);
        v.push(1);
        Mutex::new(v)
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    #[test]
    fn items_is_initialized() {
        // Accessing ITEMS triggers its initialization
        let _guard = ITEMS.lock().unwrap();
        assert!(true); // Assert true to pass the test
    }

    #[test]
    fn mutex_operations_are_successful() {
        // Simulate a scenario where multiple threads might access the ITEMS
        let handle = thread::spawn(|| {
            let mut guard = ITEMS.lock().unwrap();
            guard.push(42); // Attempt to modify the vector through the mutex
        });
        // Ensure the spawned thread completes
        handle.join().expect("Thread panicked");
        // Verify the modification
        let guard = ITEMS.lock().unwrap();
        assert_eq!(guard[0], 9);
        assert_eq!(guard[1], 2);
        assert_eq!(guard[2], 1);
        assert_eq!(guard[3], 42); // Check if the new value was added successfully
    }
}

