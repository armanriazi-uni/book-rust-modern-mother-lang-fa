#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-shared-behavior-nondispatchable_bin --bin rust-doc-shared-behavior-nondispatchable-main```
///
/// ```cargo doc  --package rust-doc-shared-behavior-nondispatchable_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-shared-behavior-nondispatchable_bin ```
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
/// This trait is object-safe, but these methods cannot be dispatched on a trait object as:
/// ```let obj: Box<dyn NonDispatchable> = Box::new(S);```
fn main() {

trait NonDispatchable {
    // Non-methods cannot be dispatched.
    fn foo() where Self: Sized {}
    // Self type isn't known until runtime.
    fn returns(&self) -> Self where Self: Sized;
    // `other` may be a different concrete type of the receiver.
    fn param(&self, other: Self) where Self: Sized {}
    // Generics are not compatible with vtables.
    fn typed<T>(&self, x: T) where Self: Sized {}
}

struct S;
impl NonDispatchable for S {
    fn returns(&self) -> Self where Self: Sized { S }
}
let obj: Box<dyn NonDispatchable> = Box::new(S);
// obj.returns(); // ERROR: cannot call with Self return
// obj.param(S);  // ERROR: cannot call with Self parameter
// obj.typed(1);  // ERROR: cannot call with generic type
}
