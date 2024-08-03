#![allow(dead_code, unused_variables)]
///
/// rust-in-action-for-flowcontrol-bin-ex-2
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-for-flowcontrol_bin --bin rust-in-action-for-flowcontrol-bin-ex-2```
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
///
/// //```rust,compile_fail,no_run


fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}