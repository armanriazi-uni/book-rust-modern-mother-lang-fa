#![allow(dead_code, unused_variables)]

/// rust-doc-non-objectsafe-ex-1
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-non-objectsafe_bin --bin rust-doc-non-objectsafe-ex-1```
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
// use std::rc::Rc;
// // Examples of non-object safe traits.
// trait NotObjectSafe {
//     const CONST: i32 = 1;  // ERROR: cannot have associated const

//     fn foo() {}  // ERROR: associated function without Sized
//     fn returns(&self) -> Self; // ERROR: Self in return type
//     fn typed<T>(&self, x: T) {} // ERROR: has generic type parameters
//     fn nested(self: Rc<Box<Self>>) {} // ERROR: nested receiver not yet supported
}

// struct S;
// impl NotObjectSafe for S {
//     fn returns(&self) -> Self { S }
// }
// let obj: Box<dyn NotObjectSafe> = Box::new(S); // ERROR
// }
