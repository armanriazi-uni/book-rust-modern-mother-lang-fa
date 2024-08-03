#![allow(dead_code, unused_variables)]

use std::rc::Rc;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-rc_bin --bin rust-doc-rc-main```
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
  let base = Rc::new(GroundStation {});   
  println!("{:?}", base);                
  println!("{:?}", base);                 
}
