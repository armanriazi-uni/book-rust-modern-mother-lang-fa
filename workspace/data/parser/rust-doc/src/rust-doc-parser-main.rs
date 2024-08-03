#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-parser_bin --bin rust-doc-parser-main```
///
/// ```cargo doc  --package rust-doc-parser_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-parser_bin```
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
/// 'nothing'
/// ## Example
/// //```rust,compile_fail,ignore

fn main(){
  let four: u32 = "4".parse().unwrap();
  assert_eq!(4, four);
  let four = "4".parse::<u32>();
  assert_eq!(Ok(4), four);
  let nope = "j".parse::<u32>();
  assert!(nope.is_err());
}