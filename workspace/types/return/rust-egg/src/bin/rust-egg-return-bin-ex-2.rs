#![allow(dead_code, unused_variables)]

/// rust-egg-return-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-return_bin --bin rust-egg-return-bin-ex-2```
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
/// `My current favorite color is blue`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from("blue")
}
