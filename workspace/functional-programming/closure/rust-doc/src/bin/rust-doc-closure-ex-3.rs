#![allow(dead_code, unused_variables)]
/// rust-doc-closure-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-closure_bin --bin rust-doc-closure-ex-3```
///
/// ```cargo doc  --package rust-doc-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-closure_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Closures are represented by traits, which means you can’t return closures directly. In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function. But you can’t do that with closures because they don’t have a concrete type that is returnable; you’re not allowed to use the function pointer fn as a return type`
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

fn main() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
}
// fn main() {
//     let list_of_numbers = vec![1, 2, 3];
//     let list_of_strings: Vec<String> =
//         list_of_numbers.iter().map(ToString::to_string).collect();
// }
