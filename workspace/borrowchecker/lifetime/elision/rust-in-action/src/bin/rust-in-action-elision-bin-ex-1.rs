#![allow(dead_code, unused_variables)]

/// rust-in-action-elision-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-elision_bin --bin  rust-in-action-elision-bin-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// ```rust,no_run,compile_fail,ignore
///fn main() {
///let reference_to_nothing = elision();
///}
///
///fn elision() -> &String {
///    let s = String::from("hello");
///
///    &s
///}
/// ```

fn main() {
let reference_to_nothing = elision();
}

fn elision() -> String {
   let s = String::from("hello");

   s
}
