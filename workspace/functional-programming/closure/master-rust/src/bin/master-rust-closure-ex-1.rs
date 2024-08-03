#![allow(dead_code, unused_variables)]
/// master-rust-closure-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-closure_bin --bin master-rust-closure-ex-1```
///
/// ```cargo doc  --package master-rust-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-closure_bin ```
///
/// ## What
/// `fn_closure`
///
/// ## How
/// `The a variable was still accessible even after invoking the closure as the closure used a by reference.`
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
    let a = String::from("Hey!");
    let fn_closure = || {
        println!("Closure says: {}", a);
    };
    fn_closure();
    println!("Main says: {}", a);
}
