#![allow(dead_code, unused_variables)]

use std::slice;


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-slice_bin --bin master-rust-slice-main```
///
/// ```cargo doc  --package master-rust-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-slice_bin```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //rust,compile_fail,no_run,ignore

fn main() {
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }

    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
    }

    println!("Look ma! I can modify through slices: {:?}", numbers);
}
