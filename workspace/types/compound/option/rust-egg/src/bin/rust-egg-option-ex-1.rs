#![allow(dead_code, unused_variables)]

/// rust-egg-option-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-option_bin --bin rust-egg-option-ex-1```
///
/// ```cargo doc  --package rust-egg-option_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-option_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your option/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  // ```rust,compile_fail,ignore

/// Make me compile! Execute `rustlings hint option2` for hints

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
