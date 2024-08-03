#![allow(dead_code, unused_variables)]

/// rust-in-action-vec-collection-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-vec-collection_bin --bin  rust-in-action-vec-collection-ex-4```
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
/// //```rust,no_run,compile_fail,ignore

fn main() {
    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);    
    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);
}
