#![allow(dead_code, unused_variables)]
use std::fmt::Debug;
/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p master-rust-shared-behavior-dispatchable_bin --bin master-rust-shared-behavior-dispatchable-main```
///
/// ```cargo doc  --package master-rust-shared-behavior-dispatchable_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-shared-behavior-dispatchable_bin ```
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
/// `Square(3.0)
/// `Rectangle(4.0, 2.0)`
///
/// ## Example
///  `TODO`
///
///



#[derive(Debug)]
struct Square(f32);
#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

/// A dyn Trait is an unsized type and can only be created as a reference.
fn main() {
    let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
    for s in shapes {
        println!("{:?}", s);
    }
}
