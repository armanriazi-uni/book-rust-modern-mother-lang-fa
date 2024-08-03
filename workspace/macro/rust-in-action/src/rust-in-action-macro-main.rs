#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-in-action-macro_bin --bin rust-in-action-macro-main```
///
/// ```cargo doc  --package rust-in-action-macro_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-macro_bin ```
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

macro_rules! MyString {
    ($x:expr) => ( // <1>
        String::from(stringify!($x)); // <2>
    )
}

fn main() {
    let s = MyString!(hellothere);
    println!("{}", s);
}
