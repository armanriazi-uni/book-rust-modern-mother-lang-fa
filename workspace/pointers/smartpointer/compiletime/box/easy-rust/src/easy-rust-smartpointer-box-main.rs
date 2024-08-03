#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p easy-rust-smartpointer-box_bin --bin easy-rust-smartpointer-box-main```
///
/// ```cargo doc  --package easy-rust-smartpointer-box_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package easy-rust-smartpointer-box_bin```
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
/// ````rust,compile_fail,no_run,ignore
/// struct Node {
///     data: u32,
///     next: Option<Node> //  next: Option<Box<Node>>
///                    // ++++    +
/// }
/// fn main() {
///     let a = Node { data: 33, next: None };
/// }
/// ```
/// `Output`:
/// You got the first error!
/// You got the second error!
/// Looks fine to me
///

use std::error::Error;
use std::fmt;
#[derive(Debug)]
struct ErrorOne;
impl Error for ErrorOne {} 
impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!") 
    }
}
#[derive(Debug)] 
struct ErrorTwo;
impl Error for ErrorTwo {}
impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()), 
    }
}
fn main() {
    let vec_of_u8s = vec![0_u8, 1, 80]; 
    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}
