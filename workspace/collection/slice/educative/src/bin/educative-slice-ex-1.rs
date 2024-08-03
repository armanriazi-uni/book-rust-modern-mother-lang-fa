#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-slice_bin --bin educative-slice-ex-1```
///
/// ```cargo doc  --package educative-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-slice_bin```
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
  let magic_choice = [0, 1, 2];
  let magic_answer = ["first", "second", "third"];

  for i in magic_choice {
    println!("{}", magic_answer[i]);
  }
  for i in magic_choice.iter() {
    println!("{}", magic_answer[*i as usize]);
  }
}
