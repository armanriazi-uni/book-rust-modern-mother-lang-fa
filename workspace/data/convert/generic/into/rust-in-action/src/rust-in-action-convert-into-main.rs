#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-convert-into_bin --bin rust-in-action-convert-into-main```
///
/// ## What
/// > performs that conversion within the function and applies any required business logic to that newly created String. This can circumvent the issue of &str being an immutable value.
/// > This implicit conversion strategy does have significant risks, though. If a stringified version of the password variable needs to be created multiple times in the pipeline, it would be much more efficient to require an explicit conversion within the calling application. That way the String would be created once and reused.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// ```rust,compile_fail,ignore

/// fn is_strong(password: String) -> bool {
///    password.len() > 5
///}
/// ```

fn is_strong<T: Into<String>>(password: T) -> bool {
password.into().len() > 5
}

fn main(){
    let pw = "justok";
    //let pw = String::from("justok");
   println!("{}",is_strong(pw));
}
