#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]
use std::time::Duration;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-generic_bin --bin rust-in-action-types-generic-main```
///
/// ```cargo doc  --package rust-in-action-types-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-generic_bin```
///
/// ## What
// `TODO`
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
/// ```rust,compile_fail,ignore
/// //error[E0369]: cannot add `T` to `T`
/// pub fn add<T>(a: T, b: T) -> T {
///    a + b
/// }

///```
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     use proptest::prelude::*;
///     proptest! {
///         #[test]
///         fn test_add(a: i64, b: i64) {
///             assert_eq!(add(a, b), a+b);
///         }
///     }
/// }
///```
///
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
   let floats = add(1.2, 3.4);
   let ints = add(10, 20);
   let durations = add(
     Duration::new(5, 0),
     Duration::new(10, 0)
   );

   println!("{}", floats);
   println!("{}", ints);
   println!("{:?}", durations);

 }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
