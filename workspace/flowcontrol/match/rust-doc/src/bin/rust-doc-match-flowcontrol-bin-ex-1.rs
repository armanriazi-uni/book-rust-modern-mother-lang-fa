#![allow(dead_code, unused_variables)]
///
/// rust-doc-match-flowcontrol-bin-ex-1
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-match-flowcontrol_bin --bin rust-doc-match-flowcontrol-bin-ex-1```
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
///
/// //```rust,compile_fail,no_run

fn main() {
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {        
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}
