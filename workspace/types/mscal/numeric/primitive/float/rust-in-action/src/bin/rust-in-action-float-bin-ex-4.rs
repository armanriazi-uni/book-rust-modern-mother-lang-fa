#![allow(dead_code, unused_variables)]

/// rust-in-action-float-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-float_bin --bin rust-in-action-float-bin-ex-4```
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
/// `Printed:-0.00009999999999998899`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let error = 0.00001f64;
    println!("Printed:{}", x - y);

    if (x - y).abs() < error {
        println!("success!");
    }
}
