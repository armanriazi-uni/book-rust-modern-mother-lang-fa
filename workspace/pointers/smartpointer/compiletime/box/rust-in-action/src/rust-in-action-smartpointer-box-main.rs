#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-smartpointer-box_bin --bin rust-in-action-smartpointer-box-main```
///
/// ```cargo doc  --package rust-in-action-smartpointer-box_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-smartpointer-box_bin ```
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
///rust,compile_fail,no_run,ignore
///fn main() {
///  let a = Box::new(10);
///  let b = a.clone();
///  let c = a;
///  println!("Printed:{} {} {}", a,b,c); //Expose Error: a^ value borrowed here after move
///}

fn main() {
  let a = Box::new(10);
  let b = a.clone();
  let c = &a;
  println!("Printed:{} {} {}", a,b,c);
}
