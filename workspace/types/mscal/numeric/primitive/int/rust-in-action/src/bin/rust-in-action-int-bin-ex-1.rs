#![allow(dead_code, unused_variables)]

/// rust-in-action-int-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-int_bin --bin rust-in-action-int-bin-ex-1```
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
/// `255`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// fn main() {
/// let (a, b) = (200, 200);
/// let c: u8 = a + b; // ^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow
/// println!("200 + 200 = {}", c);
/// }
/// ```

fn main() {
    let (a, b) = (200, 55);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}
