#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-semantic_bin --bin rust-in-action-semantic-main```
///
/// ```cargo doc  --package rust-in-action-semantic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-semantic_bin```
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
/// ``
///
/// ## Example
/// `non-primitives expose an error for requiring Copy trait`
/// ```rust,no_run,ignore,compile_fail
///fn use_value(_val: Demo) {
///}
///
///struct Demo {
///a: i32,
///}
///
///fn main() {
///let demo = Demo { a: 123 }; // ---- move occurs because `demo` has type `Demo`, which does not implement the `Copy` trait
/// use_value(demo);
/// println!("{}", demo.a);
///}
///```

fn main() {
    let string = "Hello World!".to_string();
    let reference = &string;
    println!("this is the original string: {} and this is the reference: {}", string, reference);
}
