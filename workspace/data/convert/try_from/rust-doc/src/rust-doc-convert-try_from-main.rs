  #![allow(dead_code, unused_variables, unused_imports)]
 use num::{Float};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-convert-try_from_bin --bin rust-doc-convert-try_from-main```
///
/// ```cargo test -q -p rust-doc-convert-try_from_bin --bin rust-doc-convert-try_from-main```
///
/// ```cargo doc  --package rust-doc-convert-try_from_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-convert-try_from_bin```
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

use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}