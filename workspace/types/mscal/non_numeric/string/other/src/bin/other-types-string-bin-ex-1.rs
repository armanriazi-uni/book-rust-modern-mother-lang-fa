#![allow(dead_code, unused_variables)]

/// other-types-string-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p other-types-string_bin --bin other-types-string-bin-ex-1```
///
/// ## What
///  make use of the is_nan() and is_finite() methods.
/// Inducing a crash, rather than silently proceeding with a mathematical error, allows you to debug close to what has caused the problem.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore
/// Determine if a word or phrase is an isogram.



pub fn determine_isogram(_word: &str) -> i32 {
    let mut sorted = _word
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    sorted.sort();
    let mut unique = sorted.clone();
    unique.dedup();
  if unique.len() == sorted.len(){
  return 1;
  }
  else {
    return 0;
  }
}
fn main() {
  println!("{:?}",determine_isogram("armanriazi-uni"));
  println!("{:?}",determine_isogram("1,2,3,4,5"));
  println!("{:?}",determine_isogram("1,2,3,4,1"));
  println!("{:?}",determine_isogram("abc"));
  println!("{:?}",determine_isogram("abca"));
}
