#![allow(dead_code, unused_variables)]



/// rust-doc-trait-bound-operator-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-bound-operator_bin --bin  rust-doc-trait-bound-operator-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore

use std::ops::Add;

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
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);
trait Add<Rhs=Self> {
    type Output;
 
    fn add(self, rhs: Rhs) -> Self::Output;
}
impl Add<Meters> for Millimeters {
     type Output = Millimeters;
  
     fn add(self, other: Meters) -> Millimeters {
         Millimeters(self.0 + (other.0 * 1000))
     }
 }



