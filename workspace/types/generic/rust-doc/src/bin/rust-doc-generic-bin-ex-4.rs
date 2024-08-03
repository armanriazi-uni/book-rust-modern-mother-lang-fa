#![allow(dead_code, unused_variables)]
use std::fmt::Debug;
/// rust-doc-generic-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-types-generic_bin --bin rust-doc-generic-bin-ex-4```
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
/// 'nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore

// struct Point<T>
// where T:PartialOrd {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         if (&self.x == 5){
//         0
//     }
//         &self.x
//     }
// }

fn main() {
    //let p = Point { x: 5, y: 10 };

    //println!("p.x = {}", p.x());
    unimplemented!();
}
