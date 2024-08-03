#![allow(dead_code, unused_variables)]
use rand::prelude::*;                      // <1>


/// master-rust-error-handling-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-error-handling_bin --bin  master-rust-error-handling-ex-3```
///
/// ## What
/// `combinators`
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

fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}
fn double(val: usize) -> usize {
    val * val
}
fn main() {
    let items = vec![7, 6, 4, 3, 5, 3, 10, 3, 2, 4];
    println!("{}", items.len());
    let doubled = get_nth(&items, 4).map(double);//.map(|v| v * v);
    println!("{:?}", doubled);
}
