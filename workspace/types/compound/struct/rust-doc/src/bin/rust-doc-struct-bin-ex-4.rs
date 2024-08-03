#![allow(dead_code, unused_variables)]

/// rust-doc-struct-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-struct_bin --bin rust-doc-struct-bin-ex-4```
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
/// `&rect1 = Rectangle {
///    width: 60,
///    height: 50,
///}`
///
/// ## Example
/// //```rust,compile_fail,ignore
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
