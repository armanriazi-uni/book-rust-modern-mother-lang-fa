#![allow(dead_code, unused_variables)]



/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-convert-as_bin --bin rust-doc-convert-as-main```
///
///
/// ```cargo doc  --package rust-doc-convert-as_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-convert-as_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`
///

enum Number {
    Zero,
    One,
    Two,
}
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
