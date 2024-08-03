#![allow(dead_code, unused_variables)]

/// rust-egg-string-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-types-string_bin --bin rust-egg-string-bin-ex-1```
///
/// ## What
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)
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


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
