#![allow(dead_code, unused_variables)]

/// educative-macro-handling-ex-3
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-macro-handling_bin --bin educative-macro-handling-ex-3```
///
/// ```cargo doc  --package educative-macro-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-macro-handling_bin ```
///
/// ## What
/// `macro-conversion`
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



use std::num::ParseIntError;
// This is the same function as above, but easier to read.
fn sum_numbers(numbers: [&str; 3]) -> Result<i32, ParseIntError> {
    let number = numbers[0].parse::<i32>()?;
    let number2 = numbers[1].parse::<i32>()?;
    let number3 = numbers[2].parse::<i32>()?;
    Ok(number + number2 + number3)
}

fn main() {
    println!("The sum of three items is: {:#?}", sum_numbers(["4", "5", "6"]));
}
