#![allow(dead_code, unused_variables)]

/// master-rust-error-panic-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-error-panic_bin --bin  master-rust-error-panic-ex-2```
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
use std::panic;
fn main() {
    panic::catch_unwind(|| {
        panic!("Panicking!");
    }).ok();
println!("Survived that panic.");
}
