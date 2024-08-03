#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-safe-abstraction_bin --bin rust-doc-safe-abstraction-main```
///
/// ```cargo doc  --package rust-doc-safe-abstraction_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-safe-abstraction_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
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
///
use std::slice;
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    let address = 0x01234usize;
    let r = address as *mut i32;
    //let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(ptr, 6) };
    println!("Printed:{:?}",values);
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),

        )
    }
}
fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
