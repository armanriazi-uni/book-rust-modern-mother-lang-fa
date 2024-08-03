#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-types-generic_bin --bin rust-doc-types-generic-main```
///
/// ```cargo doc  --package rust-doc-types-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-types-generic_bin```
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
/// `Announcement! Today is someone's birthday!`
/// `The longest string is abcd`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
fn main() {
ng2 = "xyz";
    let result = longest_with_an_announcement(
        strin    let string1 = String::from("abcd");
    let strig1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
