#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-rc_bin --bin rust-doc-rc-ex-3```
///
/// ```cargo doc  --package rust-doc-rc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-rc_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  `TODO`
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
/// //rust,compile_fail,no_run,ignore
///  `TODO`
///
use std::ops::Deref;
/*
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}*/

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
//implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
fn main() {
    let m = MyBox::new(String::from("Rust"));
    //hello(&(*m)[..]);
}

/*

*/
