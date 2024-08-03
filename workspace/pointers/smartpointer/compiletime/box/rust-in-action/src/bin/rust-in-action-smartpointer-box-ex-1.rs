#![allow(dead_code, unused_variables)]


/// rust-in-action-smartpointer-box-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-smartpointer-box_bin --bin  rust-in-action-smartpointer-box-ex-1```
///
/// ## What
/// `Box Heap`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `40 + 60 = 100`
///
/// ## Example
/// ``rust,no_run,compile_fail,ignore
/// let result = a + b;
/// ```
fn main() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);
    println!("{} + {} = {}", a, b, a + *b);
}
