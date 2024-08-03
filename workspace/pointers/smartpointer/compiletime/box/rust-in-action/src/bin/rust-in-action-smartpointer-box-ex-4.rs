#![allow(dead_code, unused_variables)]

use std::mem::drop;
/// rust-in-action-smartpointer-box-ex-4.rs
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-smartpointer-box_bin --bin  rust-in-action-smartpointer-box-ex-4```
///
/// ## What
/// `TODO`
///
/// ## How
/// std::mem::drop brings the function drop() into local scope. drop() deletes objects before their scope ends.
/// Types that implement Drop have a drop() method, but explicitly calling it is
/// illegal within user code. std::mem::drop is an escape hatch from that rule
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `3 3`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore

 fn main() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);
    let result1 = *a + *b + *c;
    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
 }
