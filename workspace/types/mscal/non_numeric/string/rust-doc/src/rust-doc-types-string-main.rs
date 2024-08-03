#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-types-string_bin --bin rust-doc-types-string-main```
///
/// ```cargo doc  --package rust-doc-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-types-string_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between stringing-point values.
/// These tolerances are defined as f32::EPSILON and f64::EPSILON. To be more precise,
/// it’s possible to get closer to how Rust is behaving under the hood, as the following small example shows.
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
    let mut s = String::from("foo");

    s.clear();

    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(3, s.capacity());
}
