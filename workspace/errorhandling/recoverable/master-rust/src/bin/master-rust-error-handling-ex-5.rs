#![allow(dead_code, unused_variables)]
use std::string::FromUtf8Error;                    // <1>


/// master-rust-error-handling-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-error-handling_bin --bin  master-rust-error-handling-ex-5```
///
/// ## What
/// `combinators`
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
/// ``rust,no_run,compile_fail,ignore

///The ? operator abstracts this pattern, making it possible to write the bytes_to_str method in a more concise way
fn str_upper_concise(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str).map(|s| s.to_uppercase())?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}
fn main() {
    let valid_str = str_upper_concise(vec![121, 97, 89]);
    println!("{:?}", valid_str);
}
