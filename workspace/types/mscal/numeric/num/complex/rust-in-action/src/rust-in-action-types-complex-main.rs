#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

use num::complex::Complex;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-complex_bin --bin rust-in-action-types-complex-main```
///
/// ```cargo doc  --package rust-in-action-types-complex_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-complex_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between complexing-point values.
/// These tolerances are defined as f32::EPSILON and f64::EPSILON. To be more precise,
/// it’s possible to get closer to how Rust is behaving under the hood, as the following small example shows.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
