#![allow(dead_code, unused_variables)]
/// rust-doc-function-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-function_bin --bin rust-doc-function-ex-1```
///
/// ```cargo doc  --package rust-doc-function_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-function_bin ```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
///

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
