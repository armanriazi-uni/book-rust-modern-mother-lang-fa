#![allow(unused)]
#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p proptest_runner_test --bin test_proptest-runner-main```
///
/// ```cargo doc  --package proptest_runner_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package proptest_runner_test```
///
/// ## What
/// `Test Compound Strategies`
/// `Testing functions that take single arguments of primitive types is nice and all, but is kind of underwhelming. Back when we were writing the whole stack by hand, extending the technique to, say, two integers was clear, if verbose. But TestRunner only takes a single Strategy; how can we test a function that needs inputs from more than one?`
///
/// ## How
/// `Other compound strategies include fixed-sizes arrays of strategies and Vecs of strategies (which produce arrays or Vecs of values parallel to the strategy collection), as well as the various strategies provided in the collection module.`
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


fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
use super::*;
use proptest::test_runner::TestRunner;

#[test]
pub fn test_add() {
    let mut runner = TestRunner::default();
    // Combine our two inputs into a strategy for one tuple. Our test
    // function then destructures the generated tuples back into separate
    // `a` and `b` variables to be passed in to `add()`.
    runner.run(&(0..1000i32, 0..1000i32), |(a, b)| {
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
        Ok(())
    }).unwrap();
    }
  }




fn main() { unimplemented!() }