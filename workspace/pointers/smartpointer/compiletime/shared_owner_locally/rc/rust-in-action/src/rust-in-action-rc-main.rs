#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-rc_bin --bin rust-in-action-rc-main```
///
/// ```cargo doc  --package rust-in-action-rc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-rc_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  `TODO`
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
/// //rust,compile_fail,no_run,ignore
///  `TODO`
///
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
struct GroundStation {
  radio_freq: f64 // Mhz
}
fn main() {
  let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
    GroundStation {
      radio_freq: 87.65
    }
  ));
  println!("base: {:?}", base);
  { 
    let mut base_2 = base.borrow_mut();
    base_2.radio_freq -= 12.34;
    println!("base_2: {:?}", base_2);
  }
  println!("base: {:?}", base);
  let mut base_3 = base.borrow_mut();
  base_3.radio_freq += 43.21;
  println!("base: {:?}", base);
  println!("base_3: {:?}", base_3);
}
