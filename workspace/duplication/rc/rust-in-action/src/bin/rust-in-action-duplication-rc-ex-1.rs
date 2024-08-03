#![allow(dead_code, unused_variables)]

use std::rc::Rc;

/// rust-in-action-duplication-rc-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-duplication-rc_bin --bin  rust-in-action-duplication-rc-ex-1```
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

#[derive(Debug)]
struct GroundStation {}

fn main() {
  let base = Rc::new(GroundStation {});   // <2>

  println!("{:?}", base);                 // <3>
  println!("{:?}", base);                 // <3>
}
