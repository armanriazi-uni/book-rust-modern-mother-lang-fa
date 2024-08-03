#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// master-rust-trait-closure-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-trait-bound_bin --bin  master-rust-trait-closure-ex-4```
///
/// ## What
/// `TODO`
///
/// ## How
/// `move || a + b` to use other thread
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

fn lazy_adder(a:u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

fn main() {
    let add_later = lazy_adder(1024, 2048);
    println!("{:?}", add_later());
}
