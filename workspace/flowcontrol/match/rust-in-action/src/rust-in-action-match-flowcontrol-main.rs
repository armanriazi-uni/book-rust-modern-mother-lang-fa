#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-match-flowcontrol_bin --bin rust-in-action-match-flowcontrol-main```
///
/// ```cargo doc  --package rust-in-action-match-flowcontrol_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-match-flowcontrol_bin```
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
/// `42: hit!
//   132: hit!`
///
/// ## Example
/// // ```compile_fail,ignore
/// //
fn main() {
  // let needle = 42; // <1>
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

  for reference in haystack.iter() {
    let item = *reference;

    let result = match item { // <2>
      42 | 132 => "hit!", // <3>
      _ => "miss", // <4>
    };

    if result == "hit!" {
      println!("{}: {}", item, result);
    }
  }
}
