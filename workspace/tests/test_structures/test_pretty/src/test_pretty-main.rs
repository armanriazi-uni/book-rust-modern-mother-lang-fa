#![allow(dead_code, unused_variables)]

use pretty_assertions::assert_eq;


/// Main
///
/// ## Commands
///
/// ```cargo test -q -p pretty_test --bin test_pretty-main```
///
/// ```cargo doc  --package pretty_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pretty_test```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
#[cfg(test)]
mod tests {

use super::*;
use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn diff(){
        let x= "Hello world";
        let y = "Hello world";
        
        assert_eq!(x, y);
    }
}



fn main() {
    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey ho!".to_string())});
    let y = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey ho!".to_string())});

    assert_eq!(x, y);
}

