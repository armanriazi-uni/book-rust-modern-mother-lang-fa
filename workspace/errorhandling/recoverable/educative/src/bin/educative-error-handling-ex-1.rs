#![allow(dead_code, unused_variables)]

/// educative-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p educative-error-handling_bin --bin  educative-error-handling-ex-1```
///
/// ## What
/// `Error Handling`
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
/// //```rust,no_run,compile_fail,ignore

/// This is supposed to sum numbers from an array of strings.
/// It works, but a little convoluted.
fn sum_numbers(numbers: [&str; 3]) -> Result<i32, String> {
    match numbers[0].parse::<i32>() {
        Ok(number) => match numbers[1].parse::<i32>() {
            Ok(number2) => match numbers[2].parse::<i32>() {
                Ok(number3) => Ok(number + number2 + number3),
                Err(error) => Err(error.to_string())
            },
            Err(error) => Err(error.to_string())
        },
        Err(error) => Err(error.to_string())
    }
}
fn main() {
    println!("The sum of three items is: {:#?}", sum_numbers(["4", "5", "6"]));
}
