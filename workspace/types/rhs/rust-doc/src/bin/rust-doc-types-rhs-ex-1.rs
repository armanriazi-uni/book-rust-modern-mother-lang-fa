#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]
use std::ops::Add;


use std::fmt::Display;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-types-rhs_bin --bin rust-doc-types-rhs-ex-1```
///
/// ```cargo doc  --package rust-doc-types-rhs_bin--message-format short --no-deps --open --color always```
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
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     );
// }


//#![allow(unused)]
fn main() {
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
}
