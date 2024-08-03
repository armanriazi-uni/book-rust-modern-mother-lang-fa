#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// educative-rust-trait-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p educative-trait-bound_bin --bin  educative-rust-trait-ex-5```
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
trait DoubleString {
    fn double_string(&self) -> String;
}

impl DoubleString for i32 {
    fn double_string(&self) -> String {
        format!("{}", self * 2)
    }
}

impl DoubleString for i64 {
    fn double_string(&self) -> String {
        format!("{}", self * 2)
    }
}

fn print_double(x: i32) {
    println!("The doubled value is {}", x.double_string());
}
fn print_double_64(x: i64) {
    println!("The doubled value is {}", x.double_string());
}
fn main() {
    print_double(5_i32);
    print_double_64(20_i64);
}
