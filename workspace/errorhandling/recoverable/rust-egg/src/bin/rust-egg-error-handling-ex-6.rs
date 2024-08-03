#![allow(dead_code, unused_variables)]
use std::error;
use std::fmt;
use std::io;//::{self,BufRead};

/// rust-egg-error-handling-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-6```
///
/// ```cargo test -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-6```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore

/// This example panics because the second time it calls `pop`, the `vec`
/// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
/// on `None`. Handle this in a more graceful way than calling `unwrap`!
/// Execute `rustlings hint option1` for hints :)
pub fn pop_too_much() -> bool {
    let mut list = vec![3,5];

    let last = list.pop().unwrap();
    println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop().unwrap();
    println!(
        "The second-to-last item in the list is {:?}",
        second_to_last
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
fn main(){
unimplemented!()
}
