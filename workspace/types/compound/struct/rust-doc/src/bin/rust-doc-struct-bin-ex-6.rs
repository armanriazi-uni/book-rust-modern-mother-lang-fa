#![allow(dead_code, unused_variables)]

/// rust-doc-struct-bin-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-struct_bin --bin rust-doc-struct-bin-ex-6```
///
/// ## What
///  `display`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing
///
/// ## Example
/// //```rust,compile_fail,ignore


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3); // eq instance.square()
}
