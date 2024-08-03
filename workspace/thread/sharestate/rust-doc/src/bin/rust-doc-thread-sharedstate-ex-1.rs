#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-thread-sharedstate_bin --bin rust-doc-thread-sharedstate-ex-1```
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
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore
use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
    println!("m = {:?}", m.lock().unwrap());
}
