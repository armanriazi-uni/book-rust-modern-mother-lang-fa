#![allow(dead_code, unused_variables)]

/// rust-doc-generic-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-types-generic_bin --bin rust-doc-generic-bin-ex-3```
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
/// 'nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
