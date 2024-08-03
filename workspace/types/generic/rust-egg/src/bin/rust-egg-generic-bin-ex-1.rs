#![allow(dead_code, unused_variables)]
use std::fmt::Debug;
/// rust-egg-generic-bin-ex-1
///
/// ## Commands
///
/// ```cargo test -q -p rust-egg-types-generic_bin --bin rust-egg-generic-bin-ex-1```
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
/// ` 'static value passed in is: 5`
///
/// ## Example
/// //```rust,compile_fail,ignore
///   let i = 5;
///   print_it(i);
///
///   // oops, &i only has the lifetime defined by the scope of
///   // main(), so it's not 'static:
///   print_it(&i);//error : borrowed value does not live long enough
/// ```
///
/// This powerful wrapper provides the ability to store a positive integer value.
/// Rewrite it using generics so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn store_u32_in_wrapper() {    
        assert_eq!(Wrapper::new(42).value, 42);    
    }
    #[test]
    fn store_str_in_wrapper() {
        // TODO: Delete this assert and uncomment the one  below once you have  finished the exercise.
        // assert!(false);
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

fn main(){
    unimplemented!();
}