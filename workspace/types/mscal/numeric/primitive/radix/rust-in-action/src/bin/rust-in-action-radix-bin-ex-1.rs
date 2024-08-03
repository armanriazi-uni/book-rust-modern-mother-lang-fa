#![allow(dead_code, unused_variables)]

/// rust-in-action-radix-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-radix_bin --bin rust-in-action-radix-bin-ex-1```
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
/// //```rust,compile_fail,ignore

fn main() {
    let needle = 0xCB;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in haystack.iter() {
      if *item == needle {
        println!("{}", item);
      }
    }
  }
