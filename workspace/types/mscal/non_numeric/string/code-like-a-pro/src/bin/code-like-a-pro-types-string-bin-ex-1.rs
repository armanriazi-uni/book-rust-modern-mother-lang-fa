#![allow(dead_code, unused_variables)]

/// code-like-a-pro-string-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p code-like-a-pro-types-string_bin --bin code-like-a-pro-types-string-bin-ex-1```
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
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore

struct StringWrapper(String);

impl From<&str> for StringWrapper {
    fn from(other: &str) -> Self {
        Self(other.into())
    }
}

fn main() {
    println!("{}", StringWrapper::from("Hello, world!").0);
}
