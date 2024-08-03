#![allow(dead_code, unused_variables)]

/// rust-doc-non-objectsafe-ex-2
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-non-objectsafe_bin --bin rust-doc-non-objectsafe-ex-2```
///
/// ```cargo doc  --package rust-doc-non-objectsafe_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-non-objectsafe_bin ```
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
/// `TODO`
///
/// ## Example
///  `TODO`
///


fn main() {
unimplemented!();
// // Self: Sized traits are not object-safe.
// trait TraitWithSize where Self: Sized {}

// struct S;
// impl TraitWithSize for S {}
// let obj: Box<dyn TraitWithSize> = Box::new(S); // ERROR
}
