#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin rust-in-action-pointers-main```
///
/// ```cargo doc  --package rust-in-action-pointers-bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-pointers-bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// ```
/// a (an unsigned integer):
/// location: 0x7ffe8f7ddfd0
/// size: 8 bytes
/// value: 42
/// b (a reference to B):
/// location: 0x7ffe8f7ddfd8
/// size: 8 bytes
/// points to: 0x55876090c830
/// c (a "box" for C):
/// location: 0x7ffe8f7ddfe0
/// size: 16 bytes
/// points to: 0x558762130a40
/// ```
///
/// ```
/// B (an array of 10 bytes):
/// location: 0x55876090c830
/// size: 10 bytes
/// value: [99, 97, 114, 114, 121, 116, 111, 119, 101, 108]
/// C (an array of 11 bytes):
/// location: 0x55876090c83a
/// size: 11 bytes
/// value: [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0
/// ```
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `a: 42, b: 0x5563e6ece000, c: 0x5563e6ece00a`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
///
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

/// c is a raw pointer, b is SmartPointer as Box, a int
/// related to (rust-in-action-pointers-ex-4-7)[rust-in-action-pointers-ex-4]
fn main() {
    let a = 42;
    let b = &B;
    let c = &C;


    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}
