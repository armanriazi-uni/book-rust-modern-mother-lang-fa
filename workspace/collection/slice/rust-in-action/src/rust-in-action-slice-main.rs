#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-slice_bin --bin rust-in-action-slice-main```
///
/// ```cargo doc  --package rust-in-action-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-slice_bin```
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
/// //rust,compile_fail,no_run,ignore
fn main() {
  let one            = [1,2,3]; // <1>
  let two: [u8; 3]   = [1,2,3]; // <2>
  let blank1         = [0; 3]; // <3>
  let blank2: [u8; 3] = [0; 3]; // <4>

  let arrays = [one, two, blank1, blank2]; // <5>
  for a in &arrays { // <6>
    print!("{:?}: ", a);
    for n in a.iter() { // <7>
      print!("\t{} + 10 = {}", n, n+10);
    }
    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i];
    }
    print!("\t(Σ{:?} = {})", a, sum);
    println!("");
  }
}
