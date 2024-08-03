#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-convert-as_ref_bin --bin rust-in-action-convert-as_ref-main```
///
/// ## What
// `One (&str) is allocated on the stack, the other (String) allocates memory on the heap. That means that types cannot be trivially cast between one another. It’s possible, however, to work around this with Rust’s generics.`
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

fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}
fn main(){
    let pw = String::from("justok");
    println!("{}", is_strong(pw))
}
