#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-safe-static-mut_bin --bin rust-doc-safe-static-mut-main```
///
/// ```cargo doc  --package rust-doc-safe-static-mut_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-safe-static-mut_bin```
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
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
