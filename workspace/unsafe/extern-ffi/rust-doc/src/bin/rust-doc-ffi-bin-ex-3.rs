#![allow(dead_code, unused_variables)]
use std::arch::asm;

/// rust-doc-ffi-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-ffi_bin --bin rust-doc-ffi-bin-ex-3```
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


fn main(){
let i: u64 = 3;
let o: u64;
unsafe {
    asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
    );
}
assert_eq!(o, 8);
}
