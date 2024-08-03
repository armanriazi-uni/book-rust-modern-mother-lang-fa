#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-ownership_bin --bin rust-egg-ownership-main```
///
/// ```cargo doc  --package rust-egg-ownership_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-ownership_bin```
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
fn use_value(_val: Demo) {
     println!("{}", _val.a);
}
#[derive(Debug,Copy, Clone)]
struct Demo {
 a: i32,
}
fn main() {
let demo = Demo { a: 123 }; 
 use_value(demo); 
 println!("{:?}", demo); 
}


//fn main() {
//    let string = "Hello World!".to_string();
//    let reference = &string;
//    println!("this is the original string: {} and this is the reference: {}", string, reference);
//}
