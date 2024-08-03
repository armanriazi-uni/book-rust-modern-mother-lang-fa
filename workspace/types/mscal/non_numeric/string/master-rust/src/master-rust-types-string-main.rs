#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-string_bin --bin master-rust-types-string-main```
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
    let mut empty_string = String::new();
    let empty_string_with_capacity = String::with_capacity(50);
    let string_from_bytestring: String = String::from_utf8(vec![82, 85, 83,
    84]).expect("Creating String from bytestring failed");

    println!("Length of the empty string is {}", empty_string.len());
    println!("Length of the empty string with capacity is {}",
    empty_string_with_capacity.len());
    println!("Length of the string from a bytestring is {}",
    string_from_bytestring.len());

    println!("Bytestring says {}", string_from_bytestring);

    empty_string.push('1');
    println!("1) Empty string now contains {}", empty_string);
    empty_string.push_str("2345");
    println!("2) Empty string now contains {}", empty_string);
    println!("Length of the previously empty string is now {}",
    empty_string.len());
}
