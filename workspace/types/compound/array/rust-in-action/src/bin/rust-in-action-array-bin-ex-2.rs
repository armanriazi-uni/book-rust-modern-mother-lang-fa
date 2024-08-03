#![allow(dead_code, unused_variables)]

/// rust-in-action-array-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-array_bin --bin rust-in-action-array-bin-ex-2```
///
/// ## What
/// `slice_out_of_array`
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
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays = [one, two, blank1, blank2];
    for a in &arrays {
    print!("{:?}: ", a);
    for n in a.iter() {
    print!("\t{} + 10 = {}", n, n+10);
    }
    let mut sum = 0;
    for i in 0..a.len() {
    sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
    }
 }
