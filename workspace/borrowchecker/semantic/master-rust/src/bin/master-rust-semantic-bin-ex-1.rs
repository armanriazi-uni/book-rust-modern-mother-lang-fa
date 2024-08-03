#![allow(dead_code, unused_variables)]

/// master-rust-semantic-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-semantic_bin --bin  master-rust-semantic-bin-ex-1```
///
/// ## What
/// `/// `primitives do not requiring impl Copy trait``
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `n changed to 7'
/// 's changed to Borrowing`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore

fn take_the_n(n: &mut u8) {
    *n += 2;
}
fn take_the_s(s: &mut String) {
    s.push_str("ing");
}
fn main() {
    let mut n = 5;
    let mut s = String::from("Borrow");
    take_the_n(&mut n);
    take_the_s(&mut s);
    println!("n changed to {}", n);
    println!("s changed to {}", s);
}
