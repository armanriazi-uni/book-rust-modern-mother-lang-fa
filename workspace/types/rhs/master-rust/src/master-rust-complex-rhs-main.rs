#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]
use std::ops::Add;
use std::fmt::{Formatter, Display, Result};
/// Main
///
/// ## Commands
///
/// ```cargo test -q -p master-rust-complex-rhs_bin --bin master-rust-complex-rhs-main```
///
/// ```cargo doc  --package master-rust-complex-rhs_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-complex-rhs_bin```
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
// complex/src/lib.rs



#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    // Real part
    re: T,
    // Complex part
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output=T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Complex<T> {
        Complex { re: value.0, im: value.1 }
    }
}



impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}


#[cfg(test)]
mod tests {
    use crate::Complex;
    #[test]
    fn complex_basics() {
        let first = Complex::new(3,5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        assert!(second.re == second.im);
    }
    #[test]
    fn complex_addition() {
        let a = Complex::new(1,-2);
        let b = Complex::default();
        let res = a + b;
        assert_eq!(res, a);
    }

    #[test]
    fn complex_from() {
        let a = (2345, 456);
        let complex = Complex::from(a);
        assert_eq!(complex.re, 2345);
        assert_eq!(complex.im, 456);
    }

    #[test]
    fn complex_display() {
        let my_imaginary = Complex::new(2345,456);
        println!("{}", my_imaginary);
    }
}


fn main(){
    unimplemented!()
}
