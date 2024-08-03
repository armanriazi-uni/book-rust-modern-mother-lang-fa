#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-semantic_bin --bin educative-semantic-main```
///
/// ```cargo doc  --package educative-semantic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-semantic_bin```
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
///
/// ## Example
/// The below code will return an error because we’re mutating a reference the wrong way.
/// ```rust,no_run,ignore,compile_fail
/// fn main() {
///     let string = "Hello World!".to_string();
///     let mut reference = &string;
///     reference = &"Hola Mundo!".to_string(); //Error exposed
///     println!("this is the original string: {} and this is the reference: {}", string, reference);
/// }
///```
/// `Out put`
/// error[E0716]: temporary value dropped while borrowed

fn change_string(string: &mut String) {
    string.push_str(", Changing the World!");
}
/// We can mutate references, but we’ll be mutating the original value
fn main() {
    let mut string = "Hello World!".to_string();
    change_string(&mut string);
    println!("this is the original string: {}", string);
}
