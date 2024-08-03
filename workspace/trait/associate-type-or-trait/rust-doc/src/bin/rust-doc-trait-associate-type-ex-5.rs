#![allow(dead_code, unused_variables)]

/// rust-doc-trait-associate-type-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-associate-type-and-trait_bin --bin  rust-doc-trait-associate-type-ex-5```
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

trait ConstantId {
    const ID: i32;
}

struct Struct;

impl ConstantId for Struct {
    const ID: i32 = 1;
}

fn main() {
    assert_eq!(1, Struct::ID);
}
