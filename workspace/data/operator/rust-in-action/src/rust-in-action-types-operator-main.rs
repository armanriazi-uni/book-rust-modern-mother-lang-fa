#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-operator_bin --bin rust-in-action-types-operator-main```
///
/// ```cargo doc  --package rust-in-action-types-operator_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-radix_bin```
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
/// `1 + 2 = 3
/// 1 - 2 = -1
/// true AND false is false
/// true OR false is true
/// NOT true is false
/// 0011 AND 0101 is 00000001
/// 0011 OR 0101 is 0111
/// 0011 XOR 0101 is 0110
/// 1 << 5 is 32
/// 0x80 >> 2 is 0x20
/// One million is written as 1000000`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:08b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000);
}
