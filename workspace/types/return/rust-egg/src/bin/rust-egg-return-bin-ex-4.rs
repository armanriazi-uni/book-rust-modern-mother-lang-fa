#![allow(dead_code, unused_variables)]

/// rust-egg-return-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-return_bin --bin rust-egg-return-bin-ex-4```
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
/// `the value of optional value is: rustlings`
/// `current value: 9`
/// `current value: 8`
/// `current value: 7`
/// `current value: 6`
/// `current value: 5`
/// `current value: 4`
/// `current value: 3`
/// `current value: 2`
/// `current value: 1`
/// ## Example
/// ```rust,no_run
///   //make this a while let statement - remember that vector.pop also adds another layer of Option<T>
///   // You can stack `Option<T>`'s into while let and if let
///   while let Some(Some(value)) = optional_values_vec.pop() {
///       println!("current value: {}", value);
///   }
///```
fn main() {
    let optional_value = Some(String::from("rustlings"));
    // Make this an if let statement whose value is "Some" type
    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
