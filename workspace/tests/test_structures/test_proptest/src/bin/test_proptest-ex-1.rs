#![allow(dead_code, unused_variables)]
use std::ops::Add;
use num_traits::ops::wrapping::WrappingAdd;


/// Main
///
/// ## Commands
///
/// ```cargo test -q -p proptest_test --bin test_proptest-ex-1```
///
/// ```cargo doc  --package proptest_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package proptest_test```
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

pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn wadd<T: WrappingAdd<Output = T>>(a: T, b: T) -> T {
  a.wrapping_add(&b)
}

#[cfg(test)]
mod tests {
use super::*;
  use proptest::prelude::*;
   
    proptest! {
      #[test]
      fn addition_is_commutative(a: u8, b: u8) {
        prop_assert_eq!(a as u16 + b as u16, b as u16 + a as u16);
      }      
      #[test]
      fn test_addition(a in 0..10, b in 0..10) {
        prop_assert!(a + b <= 18);
      }
    
      #[test]
      fn test_string_concat(a in ".*", b in ".*") {
        let cat = format!("{}{}", a, b);
        prop_assert_eq!(a.len() + b.len(), cat.len());
      }
    
 
    #[test]
    fn test_add(a: i64, b: i64) {
        assert_eq!(crate::add(a, b), a + b);
    }
    #[test]
    fn test_wadd(a: i64, b: i64) {
        assert_eq!(crate::wadd(a, b), a.wrapping_add(b));
    }
  }
}

fn main() {

    println!("{}", "Printed");
}

