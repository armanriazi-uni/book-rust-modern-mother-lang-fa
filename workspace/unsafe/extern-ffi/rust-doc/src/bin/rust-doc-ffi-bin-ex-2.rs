#![allow(dead_code, unused_variables)]

/// rust-doc-ffi-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-ffi_bin --bin rust-doc-ffi-bin-ex-2```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This usage of extern does not require unsafe.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
