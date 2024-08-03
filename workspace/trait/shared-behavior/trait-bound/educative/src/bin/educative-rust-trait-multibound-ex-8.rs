#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


///  educative-rust-trait-multibound-ex-8
///
/// ## Commands
///
/// ```cargo run -q -p educative-trait-bound_bin --bin educative-rust-trait-multibound-ex-8```
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
/// `nothing`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore
fn info<T>(x: T)
where
    T: Double + std::fmt::Display + Copy,
{
    println!("Original number: {}", x);
    println!("Doubled number: {}", x.double());
    println!("Quadrupled number: {}", x.double().double());
}
trait Double {
    fn double(self) -> Self;
}

impl Double for i32 {
    fn double(self) -> i32 { self * 2 }
}



fn main() {
    info(5);
    info(10);
}
