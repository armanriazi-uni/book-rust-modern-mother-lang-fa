#![allow(dead_code, unused_variables)]

/// rust-in-action-vec-collection-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-vec-collection_bin --bin  rust-in-action-vec-collection-ex-5```
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
    let fruit = vec!['🥝', '🍌', '🍇'];
    let buffer_overflow = fruit[4];   // <1>
    assert_ne!(buffer_overflow, '🍉') // <2>
  }