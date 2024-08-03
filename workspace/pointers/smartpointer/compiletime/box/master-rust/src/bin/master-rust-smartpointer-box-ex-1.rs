#![allow(dead_code, unused_variables)]


/// master-rust-smartpointer-box-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-smartpointer-box_bin --bin  master-rust-smartpointer-box-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
fn box_ref<T: std::fmt::Debug>(b: T) -> Box<T> {
    let a = b;    
    Box::new(a)
}
#[derive(Debug)]
struct Foo;

fn main() {
    let boxed_one = Box::new(Foo);
    let unboxed_one = *boxed_one;    
    println!("{:?}", box_ref(unboxed_one));
}
