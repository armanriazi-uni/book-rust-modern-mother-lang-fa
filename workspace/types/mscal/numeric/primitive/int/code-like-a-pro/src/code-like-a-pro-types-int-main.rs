#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-a-pro-types-int_bin --bin code-like-a-pro-types-int-main```
///
/// ```cargo doc  --package code-like-a-pro-types-int_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-a-pro-types-int_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between inting-point values.
/// These tolerances are defined as f32::EPSILON and f64::EPSILON. To be more precise,
/// it’s possible to get closer to how Rust is behaving under the hood, as the following small example shows.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `value=0, length=1
/// value=1, length=2
/// value=2, length=4
/// value=3, length=8
/// value=4, length=16
/// Binary (base 2)         0b1111_1111=255
/// Octal (base 8)          0o1111_1111=2396745
/// Decimal (base 10)         1111_1111=11111111
/// Hexadecimal (base 16)   0x1111_1111=286331153
/// Byte literal            b'A'=65`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let value = 0u8;


    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0b1u16;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0o2u32;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0x3u64;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 4u128;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));

    println!("Binary (base 2)         0b1111_1111={}", 0b1111_1111);
    println!("Octal (base 8)          0o1111_1111={}", 0o1111_1111);
    println!("Decimal (base 10)         1111_1111={}", 1111_1111);
    println!("Hexadecimal (base 16)   0x1111_1111={}", 0x1111_1111);
    println!("Byte literal            b'A'={}", b'A');
}
