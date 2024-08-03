#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-egg-macro_bin --bin rust-egg-macro-ex-1```
///
/// ```cargo doc  --package rust-egg-macro_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-macro_bin ```
///
/// ## What
/// `macro-share behavior`
///
/// ## How
/// The macro above takes a list of parameters in $( $x:expr ),*, then it uses them in a loop to sum all the costs of the products provided.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// //```rust
// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
        ($val:expr) => {
            println!("Look at this other macro: {}", $val);
        };
    }

}

fn main() {
    use macros;
    my_macro!();
    my_macro!(7777);
}
