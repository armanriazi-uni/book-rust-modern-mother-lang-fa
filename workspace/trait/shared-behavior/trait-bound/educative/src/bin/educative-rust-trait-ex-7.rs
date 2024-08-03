#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// educative-rust-trait-ex-7
///
/// ## Commands
///
/// ```cargo run -q -p educative-trait-bound_bin --bin  educative-rust-trait-ex-7```
///
/// ## What
/// `TODO`
///
/// ## How
/// Unlike interfaces in languages like Java, C# or Scala, new traits can be implemented for existing types (as with double based on following codes).
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `error[E0599]: no method named `double` found for type `T` in the current scope`
///
/// ## Example
/// ```rust,no_run,compile_fail,ignore
/// trait Double {
///     fn double(&self) -> Self;
/// }

/// impl Double for i32 {
///     fn double(&self) -> Self {
///         self * 2
///     }
/// }

/// struct Fruit<T> {
///     apples: T,
///     bananas: T,
/// }

/// impl<T> Double for Fruit<T> {
///     fn double(&self) -> Self {
///         Fruit {
///             apples: self.apples.double(),
///             bananas: self.bananas.double(),
///         }
///     }
/// }

/// fn main() {
///     let fruit = Fruit {
///         apples: 5,
///         bananas: 10,
///     };

///     println!("Fruit total is {} and then doubled, fruit total is {}.", fruit.apples + fruit.bananas, fruit.apples.double() + fruit.bananas.double());
/// }
///```
/// Return error[E0599]: no method named `double` found for type `T` in the current scope
/// This is the same problem we ran into before with the quadruple function.
/// We’ve stated, “Hey, I can implement the Double trait for a Fruit with any type parameter T."
/// But that’s not actually true. We need to know that T itself has an implementation of Double, since we rely on the double method in the implementation. To solve this, we can use a trait bound.


 fn main() {
    unimplemented!();
 }
