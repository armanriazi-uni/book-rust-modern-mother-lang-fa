#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-ffi_bin --bin rust-doc-ffi-main```
///
/// ```cargo doc  --package rust-doc-ffi_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-ffi_bin```
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
/// `Absolute value of -3 according to C: 3`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
#[no_mangle]
pub extern "C" {
    fn add_numbers(a: u64, b: u64) -> u64;
    fn multiply_numbers(a: u64, b: u64) -> u64;
}
fn main() {
  //  unimplemented!();
    let x = 5;
    let y = 7;
    let sum: u64;
    let product: u64;

    unsafe {
       sum = add_numbers(x, y);
       product = multiply_numbers(x, y);
    }

    println!("The sum of {} and {} is {}.", x, y, sum);
    println!("The product of {} and {} is {}.", x, y, product);
}
/*
https://cratecode.com/info/rust-ffi
https://rust-lang.github.io/unsafe-code-guidelines
https://doc.rust-lang.org/rust-by-example/unsafe/asm.html
*/
