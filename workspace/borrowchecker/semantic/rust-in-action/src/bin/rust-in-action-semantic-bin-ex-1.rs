#![allow(dead_code, unused_variables)]

/// rust-in-action-dangling-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-semantic_bin --bin  rust-in-action-semantic-bin-ex-1```
///
/// ## What
/// `Mome Semantic` primitives do not requiring impl Copy trait
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
/// //```rust,no_run,compile_fail,ignore

 fn use_value(_val: i32) {
 }

 fn main() {
    let a = 123 ;
    use_value(a);

    println!("{}", a);

}
