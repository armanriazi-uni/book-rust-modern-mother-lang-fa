#![allow(dead_code, unused_variables)]

/// rust-in-action-types-operator-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-operator_bin --bin rust-in-action-types-operator-ex-1```
///
/// ```cargo doc  --package rust-in-action-types-operator_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-radix_bin```
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
/// `1 + 2 = 3
/// 1 - 2 = -1
/// true AND false is false
/// true OR false is true
/// NOT true is false
/// 0011 AND 0101 is 00000001
/// 0011 OR 0101 is 0111
/// 0011 XOR 0101 is 0110
/// 1 << 5 is 32
/// 0x80 >> 2 is 0x20
/// One million is written as 1000000`
///
/// ## Example
/// //```rust,compile_fail,ignore
use std::ops::{Add};


fn main() {
  let (a, b) = (1.2, 3.4);
  let (x, y) = (10, 20);

  let c = add(a,b);
  let z = add(x,y);

  println!("{} + {} = {}", a, b, c);
  println!("{} + {} = {}", x, y, z);
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
  i + j
}
