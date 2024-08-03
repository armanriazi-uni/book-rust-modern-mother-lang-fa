#![allow(dead_code, unused_variables, unused_imports)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p master-rust-shared-behavior-dispatchable_bin --bin master-rust-shared-behavior-dispatchable-ex-1```
///
/// ```cargo doc  --package master-rust-shared-behavior-dispatchable_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-shared-behavior-dispatchable_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// > Do note that we can only create trait objects of types whose sizes we know at compile time.
/// > A dyn Trait is an unsized type and can only be created as a reference. We can also create trait objects by putting them behind other pointer types such as Box, Rc, Arc, and so on.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Hello trait object"
///
/// ## Example
///  `TODO`
///
///

fn show_me(item: &dyn  std::fmt::Display) {
    println!("{}", item);
}

/// by emitting & we will get error doesn't have a size known at compile-time.
/// ```rust
/// show_me("Hello trait object");
/// ```
fn main() {
    show_me(&"Hello trait object");
}
