#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-elision_bin --bin rust-doc-elision-main```
///
/// ```cargo doc  --package rust-doc-elision_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-elision_bin```
///
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
/// `The longest string is long string is long
/// The longest string is long string is long`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", string1);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
