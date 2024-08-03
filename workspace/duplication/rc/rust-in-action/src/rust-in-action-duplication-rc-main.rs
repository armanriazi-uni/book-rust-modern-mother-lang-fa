#![allow(dead_code, unused_variables)]
use std::rc::Rc;                          // <1>
use std::cell::RefCell;
/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p rust-in-action-duplication-rc_bin --bin rust-in-action-duplication-rc-main```
///
/// ```cargo doc  --package rust-in-action-duplication-rc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-duplication-rc_bin ```
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
///



#[derive(Debug)]
struct GroundStation {
  radio_freq: f64  // Mhz
}

fn main() {
  let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
    GroundStation {
      radio_freq: 87.65
    }
  ));

  println!("base: {:?}", base);

  {                                            // <1>
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
