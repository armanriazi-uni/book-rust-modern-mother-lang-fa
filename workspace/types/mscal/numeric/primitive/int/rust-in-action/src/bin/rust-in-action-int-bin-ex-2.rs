#![allow(dead_code, unused_variables)]

/// rust-in-action-int-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-int_bin --bin rust-in-action-int-bin-ex-3```
///
/// ## What
/// `signed int vs unsigned int`
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
/// //```rust,compile_fail,ignore

fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a); // <1>
    println!("b: {:016b} {}", b, b); // <1>
}
