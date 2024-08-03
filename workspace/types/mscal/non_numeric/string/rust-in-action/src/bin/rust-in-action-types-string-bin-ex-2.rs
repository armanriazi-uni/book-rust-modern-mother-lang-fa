#![allow(dead_code, unused_variables)]

/// rust-in-action-string-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-string_bin --bin rust-in-action-string-bin-ex-2```
///
/// ## What
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore


fn main(){
    let mut s = String::from("hello");

    s.truncate(2);

    assert_eq!("he", s);
}
