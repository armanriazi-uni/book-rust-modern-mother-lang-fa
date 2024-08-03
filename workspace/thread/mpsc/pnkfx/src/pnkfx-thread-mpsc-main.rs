#![allow(dead_code, unused_variables)]

use std::thread;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p pnkfx-thread-mpsc_bin --bin pnkfx-thread-mpsc-main```
///
/// ```cargo doc  --package pnkfx-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pnkfx-thread-mpsc_bin```
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
/// ```compile_fail,ignore
/// use std::thread;
/// fn main() {
/// let mut data = 100;
/// thread::spawn(|| { data = 500; }); //error[E0499]: cannot borrow `data` as mutable more than once at a time
/// thread::spawn(|| { data = 1000; });
/// println!("{}", data); //^^^^ immutable borrow occurs here
///}
/// ```
///

fn main() {}
