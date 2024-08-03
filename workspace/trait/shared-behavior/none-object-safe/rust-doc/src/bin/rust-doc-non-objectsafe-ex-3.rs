#![allow(dead_code, unused_variables)]

/// rust-doc-non-objectsafe-ex-3
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-non-objectsafe_bin --bin rust-doc-non-objectsafe-ex-3```
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
// // Not object safe if `Self` is a type argument.
// trait Super<A> {}
// trait WithSelf: Super<Self> where Self: Sized {}

// struct S;
// impl<A> Super<A> for S {}
// impl WithSelf for S {}
// let obj: Box<dyn WithSelf> = Box::new(S); // ERROR: cannot use `Self` type parameter
}
