#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-arithmetic_bin --bin master-rust-arithmetic-main```
///
/// ```cargo doc  --package master-rust-arithmetic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-arithmetic_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// ```assert_eq!(25u8.wrapping_mul(12), 44);```
/// 25*12=400-256= 44
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
fn main() {
    assert_eq!(25u8.wrapping_mul(12), 44);
    assert_eq!(0xffu8.wrapping_add(1), 0);
    assert_eq!(0xffffffffu32.wrapping_add(1), 0);
    assert_eq!(0u32.wrapping_sub(1), 0xffffffff);
    assert_eq!(0x80000000u32.wrapping_mul(2), 0);
}
