#![allow(dead_code, unused_variables)]


/// master-rust-error-handling-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-error-handling_bin --bin  master-rust-error-handling-ex-6```
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
/// `Error: "I got only 3 fruits"`
///
/// ## Example
/// ``rust,no_run,compile_fail,ignore

// we do not have println but we can see error message on termial
fn main() -> Result<(), &'static str> {
    let s = vec!["apple", "mango", "banana"];
    let fourth = s.get(4).ok_or("I got only 3 fruits")?;
    Ok(())
}
