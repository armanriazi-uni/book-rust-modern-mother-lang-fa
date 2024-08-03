#![allow(dead_code, unused_variables)]
use std::thread;
/// master-rust-error-panic-ex-1
///
/// ## Commands
///
/// ```RUST_BACKTRACE=1 cargo run -q -p master-rust-error-panic_bin --bin  master-rust-error-panic-ex-1```
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
/// //``rust,no_run,compile_fail,ignore


fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        bob();
    })
}
fn bob() {
    malice();
}
fn malice() {
    panic!("malice is panicking!");
}
fn main() {
    let child = alice();
    let _ = child.join();

    bob();
        println!("This is unreachable code");
}
