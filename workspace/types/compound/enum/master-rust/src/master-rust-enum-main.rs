#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-enum_bin --bin master-rust-enum-main```
///
/// ```cargo doc  --package master-rust-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-enum_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your enum/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
enum Container {
    Item(u64),
    Empty
}

fn main() {
    let maybe_item = Container::Item(0u64);
    let has_item = if let Container::Item(0) = maybe_item {
        true
    } else {
        false
    };
}
