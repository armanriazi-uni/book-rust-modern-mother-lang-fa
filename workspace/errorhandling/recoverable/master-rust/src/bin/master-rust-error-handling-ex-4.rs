#![allow(dead_code, unused_variables)]
use std::string::FromUtf8Error;                    // <1>


/// master-rust-error-handling-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-error-handling_bin --bin  master-rust-error-handling-ex-4```
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


fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err)
    };
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}
fn main() {
    let invalid_str = str_upper_match(vec![197, 198]);
    println!("{:?}", invalid_str);
}
