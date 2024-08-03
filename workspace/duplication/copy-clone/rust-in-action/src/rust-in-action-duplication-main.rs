#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p rust-in-action-duplication_bin --bin rust-in-action-duplication-main```
///
/// ```cargo doc  --package rust-in-action-duplication_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-duplication_bin ```
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
struct CubeSat {
  id: u64,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

impl Copy for CubeSat { }

impl Copy for StatusMessage { }

impl Clone for CubeSat { // <1>
  fn clone(&self) -> Self {
    CubeSat { id: self.id } //<2>
  }
}

impl Clone for StatusMessage {
  fn clone(&self) -> Self {
    *self // <3>
  }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
  StatusMessage::Ok
}

fn main () {
  let sat_a = CubeSat { id: 0 };
  let sat_b = CubeSat { id: 1 };
  let sat_c = CubeSat { id: 2 };

  let a_status = check_status(sat_a);
  let b_status = check_status(sat_b);
  let c_status = check_status(sat_c);
  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

  // "waiting" ...
  let a_status = check_status(sat_a);
  let b_status = check_status(sat_b);
  let c_status = check_status(sat_c);
  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
