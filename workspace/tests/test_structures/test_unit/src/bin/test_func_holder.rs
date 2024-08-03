#![allow(dead_code, unused_variables)]

/// Bin
///
/// ## Commands
///
/// ```cargo test -q -p unit_test --bin test_func_holder -- --show-output --ignored```
///
/// ```cargo doc  --package unit_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package unit_test```
///
/// ## What
/// `Unit Test`
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
/// `TODO`


#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}

fn main() {
    unimplemented!();
}