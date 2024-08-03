#![allow(dead_code, unused_variables)]
use std::fmt::Display;


/// master-rust-trait-bound-trait-both-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-trait-bound_bin --bin  master-rust-trait-bound-trait-both-ex-3```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This provides advantages in cases where we want to return a complex or unrepresentable type, such as a closure from a function. Without this syntax, you had to return it by putting it behind a pointer using the Box smart pointer type, which involves heap allocation`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// ``rust
/// fn show_me(val: impl Display) {
///     println!("{}", val);
/// }
/// fn main() {
///     show_me("Trait bounds are awesome");
/// }
/// ```


fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{}}}", val)
}

fn main() {
    println!("{}", surround_with_braces("Hello"));
}
