#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-union_bin --bin master-rust-union-main```
///
/// ```cargo doc  --package master-rust-union_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-union_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `At the time of writing this book, union types only allow Copy types as their fields. They share the same memory space with all of their fields, exactly like C unions.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your union/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore

#[repr(C)]
union Metric {
    rounded: u32,
    precise: f32,
}
fn main() {
    let mut a = Metric { rounded: 323 };
    unsafe {
        println!("{}", a.rounded);
    }
    unsafe {
        println!("{}", a.precise);
    }
    a.precise = 33.3;
    unsafe {
        println!("{}", a.precise);
    }
}
