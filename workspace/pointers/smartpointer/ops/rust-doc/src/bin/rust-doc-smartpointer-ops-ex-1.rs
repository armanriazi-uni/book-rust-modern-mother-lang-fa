#![allow(dead_code, unused_variables)]
use std::ops::Deref;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-ops_bin --bin rust-doc-smartpointer-ops-ex-1```
///
/// ```cargo doc  --package rust-doc-smartpointer-ops_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-ops_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This works as you would expect, and the only added cost is that you have to write a bit more. The additional runtime cost is zero, though, and the references to the mutable things remain immutable.`
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
/// //```rust,compile_fail,no_run,ignore
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}
fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5,*(y.deref()));
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}
