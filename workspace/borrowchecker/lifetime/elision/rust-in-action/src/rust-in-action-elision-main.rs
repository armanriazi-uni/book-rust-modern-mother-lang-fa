#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-elision_bin --bin rust-in-action-elision-main```
///
/// ```cargo doc  --package rust-in-action-elision_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-elision_bin```
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
/// `30`
///
/// ## Example
/// ```rust
///fn add_with_lifetimes<'a>(i: &'a i32, j: &'a i32) -> i32 {
///   *i + *j
 ///}```

fn add_with_lifetimes<'a, 'b, 'c>(i: &'a i32, j: &'b i32) -> &'c i32 {
   *i + *j
 }
 fn main() {
   let a = 10;
   let b = 20;
   let & 'c res = add_with_lifetimes(&a, &b);

   println!("{}", res);
 }
