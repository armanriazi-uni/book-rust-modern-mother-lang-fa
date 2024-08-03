#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-factorial_bin --bin educative-factorial-main```
///
/// ## What
/// `loop int`
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
///```rust

/// fn factorial(n: u32) -> u32 {
///     if n < 2 {
///         1
///     } else {
///         n * factorial(n - 1)
///     }
///}
/// fn factorial(num: u64) -> u64 {
///     match num {
///         0 | 1 => 1,
///         _ => factorial(num - 1) * num,
///     }
/// }
///```
///
fn factorial(x: u32) -> u32 {
    let mut factorial: u32 = 1;
    let mut i: u32;
    for i in 1..=x {
        if &i <= &x {
            factorial *= i;
        }
    }
    factorial
}
fn main() {
    let n = 4;
    println!("{}", factorial(n as u32));
}
