#![allow(dead_code, unused_variables)]

/// educative-error-handling-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p educative-error-handling_bin --bin  educative-error-handling-ex-2```
///
/// ## What
/// `Error Handling`
///
/// ## How
/// We can unwrap the result value, which will return the contained value. However, it’ll panic if it’s an error. We recommend not using unwrap unless you need the code to panic (like in testing).
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
fn convert_to_integer(number: &str) -> i32 {
    number.parse::<i32>().unwrap()
    //number.parse::<i32>().expect("Not a number!")
}

fn main() {
//     println!(
//         "convert a str to a number: {:#?}",
//         convert_to_integer("Hello!")         //Error Exposed in parsing function of convert_to_integer-unwrap
//     );
         println!(
        "convert a str to a number: {:#?}",
        convert_to_integer("10")
    );
 }
