#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-string_bin --bin master-rust-types-string-ex-2```
///
/// ```cargo egg  --package master-rust-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package master-rust-types-string_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Internally, &String automatically coerces to &str, due to the type coercion trait Deref implemented for &String to &str. This is because String implements Deref for the str type.`
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
fn say_hello(to_whom: &str) {
    println!("Hey {}!", to_whom)
}

fn main() {
    let string_slice: &'static str = "you";
    let string: String = string_slice.into();
    say_hello(string_slice);
    say_hello(&string);
}
