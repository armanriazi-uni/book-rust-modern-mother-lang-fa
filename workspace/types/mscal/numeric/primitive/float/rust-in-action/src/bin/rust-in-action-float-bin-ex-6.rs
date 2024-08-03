#![allow(dead_code, unused_variables)]

/// rust-in-action-float-bin-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-float_bin --bin rust-in-action-float-bin-ex-6```
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
///
/// ## Example
/// //```rust,compile_fail,ignore
/// Rust Egg
fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let error = 0.0001f64;
    if (x - y).abs() < error {
        println!("Success!");
    }
}
