#![allow(dead_code, unused_variables)]
/// rust-doc-closure-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-closure_bin --bin rust-doc-closure-ex-2```
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
/* 
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}

The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. We saw a solution to this problem earlier. We can use a trait object:
*/

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn main() {
    let increment = returns_closure();
    let result = increment(5);

    println!("Result: {}", result); // Output: Result: 6
}

