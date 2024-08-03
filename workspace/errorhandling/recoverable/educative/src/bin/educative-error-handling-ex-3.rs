#![allow(dead_code, unused_variables)]


/// educative-error-handling-ex-3
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-error-handling_bin --bin educative-error-handling-ex-3```
///
/// ```cargo doc  --package educative-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-error-handling_bin```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
///

// This is the same function as above, but easier to read.
use std::num::ParseIntError;
fn sum_numbers(numbers: [&str; 3]) -> Result<i32, ParseIntError> {
    let number = numbers[0].parse::<i32>()?;
    let number2 = numbers[1].parse::<i32>()?;
    let number3 = numbers[2].parse::<i32>()?;
    Ok(number + number2 + number3)
}
fn main() {
    println!("The sum of three items is: {:#?}", sum_numbers(["4", "5", "6"]));
}
