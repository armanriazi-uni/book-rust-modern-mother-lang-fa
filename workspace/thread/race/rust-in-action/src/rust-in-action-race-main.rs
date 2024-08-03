#![allow(dead_code, unused_variables)]

use std::thread;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-race_bin --bin rust-in-action-race-main```
///
/// ```cargo doc  --package rust-in-action-race_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-race_bin```
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

fn main() {
    unimplemented!();
}
