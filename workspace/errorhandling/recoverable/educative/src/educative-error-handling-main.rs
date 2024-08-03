#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-error-handling_bin --bin educative-error-handling-main```
///
/// ```cargo doc  --package educative-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-error-handling_bin ```
///
/// ## What
/// `error-conversion`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// sum two numbers: Ok(
///    579,
///)
///
/// ## Example
///  `TODO`
///


/// This is the same function we've seen in the lesson, but we are
/// serializing a json object, taking the value and sum it with the argument.
/// Without anyhow this code would have a lot of more lines of code.
use anyhow::{anyhow, Result};
use serde_json::Value;
fn sum_numbers(number: &str) -> Result<i32> {
    let num = number.parse::<i32>()?;
    let num_json: Value = serde_json::from_str("{\"one\": 12}")?;
    let get_number = num_json.get("one").ok_or(anyhow!("Error getting number"))?;// get("j")
    let num2: i32 = serde_json::from_value(get_number.clone())?;
    Ok(num + num2)
}
fn main() {
    println!("sum two numbers: {:#?}", sum_numbers("567")); // sum_numbers("aaa")// kind: InvalidDigit,
}
