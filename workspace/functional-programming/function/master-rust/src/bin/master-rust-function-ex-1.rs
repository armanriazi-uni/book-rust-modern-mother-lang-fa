#![allow(dead_code, unused_variables)]
/// master-rust-function-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-function_bin --bin master-rust-function-ex-1```
///
/// ```cargo doc  --package master-rust-function_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-function_bin ```
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
///


const fn salt(a: u32) -> u32 {
    0xDEADBEEF ^ a
}

const CHECKSUM: u32 = salt(23);

fn main() {
    println!("{}", CHECKSUM);
}
