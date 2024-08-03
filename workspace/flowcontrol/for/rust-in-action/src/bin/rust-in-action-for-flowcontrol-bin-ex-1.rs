#![allow(dead_code, unused_variables)]
///
/// rust-in-action-for-flowcontrol-bin-ex-1
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-for-flowcontrol_bin --bin rust-in-action-for-flowcontrol-bin-ex-1```
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
///
/// //```rust,compile_fail,no_run

fn main() {
  let start_at = 3;
  let stop_at = 10;
  let mut items = vec![];
  for (i,x) in (start_at..stop_at).enumerate() {
    let y = i * x;
    items.push(y);
  }
  println!("{:?}", items);
  let multiples_of_10: Vec<_> = items.into_iter().filter(|y| y % 10 == 0).collect();
  println!("{:?}", multiples_of_10);

}
