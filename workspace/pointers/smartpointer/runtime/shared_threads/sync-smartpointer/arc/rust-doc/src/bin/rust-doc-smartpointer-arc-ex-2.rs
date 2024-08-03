#![allow(dead_code, unused_variables)]
use std::sync::{Arc, Weak};

/// rust-doc-smartpointer-arc-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-arc_bin --bin  rust-doc-smartpointer-arc-ex-2```
///
/// ```cargo doc  --package rust-doc-smartpointer-arc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-arc_bin```
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
/// `unimplemented`
///
/// ## Example
/// `TODO`

use std::thread;

fn main() {
    // Create a new Gadget instance.
    let gadget = Gadget::new();

    // Demonstrate sharing the Gadget across threads.
    let mut handles = vec![];

    for _ in 0..10 {
        let gadget_clone = gadget.clone();
        let handle = thread::spawn(move || {
            println!("Thread: {:?}", gadget_clone.me());
        });
        handles.push(handle);
    }
    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Debug)]
struct Gadget {
    me: Weak<Gadget>,
}
impl Gadget {
    /// Construct a reference counted Gadget.
    fn new() -> Arc<Self> {
        Arc::new_cyclic(|me| {
            Gadget { me: me.clone() }
        })
    }
    /// Return a reference counted pointer to Self.
    fn me(&self) -> Arc<Self> {
        self.me.upgrade().unwrap()
    }
}
