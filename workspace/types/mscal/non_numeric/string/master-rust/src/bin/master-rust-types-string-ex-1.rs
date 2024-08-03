#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-string_bin --bin master-rust-types-string-ex-1```
///
/// ```cargo egg  --package master-rust-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package master-rust-types-string_bin```
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
/// 'nothing'
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let hello = String::from("Hello");
    for c in hello.chars() {
        println!("{}", c);
    }
}
