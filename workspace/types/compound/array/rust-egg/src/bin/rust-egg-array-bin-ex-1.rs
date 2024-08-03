#![allow(dead_code, unused_variables)]

/// rust-egg-array-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-types-array_bin --bin rust-egg-array-bin-ex-1```
///
/// ## What
/// `slice_out_of_array`
///
/// ## How
/// Get a slice out of Array a where the ??? is so that the test passes.
/// Execute `rustlings hint primitive_types` for hints!!
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
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
