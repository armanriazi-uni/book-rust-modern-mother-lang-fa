#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo test -q -p rust-doc_operator_bin --bin rust-doc-operator-main```
///
/// ```cargo doc  --package rust-doc_operator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc_operator_bin ```
///
/// ## What
/// `we are using a u8 as Rust does not have a native type to represent bit`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main(){
        let mut point = Point { x: 1, y: 0 };
        point += Point { x: 2, y: 3 };
        assert_eq!(point, Point { x: 3, y: 3 });
    }
}
fn main(){
    unimplemented!();
}
