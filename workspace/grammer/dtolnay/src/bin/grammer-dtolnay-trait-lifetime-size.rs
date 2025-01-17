/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-trait-lifetime-size```
///
/// ```cargo doc  --package grammer-dtolnay_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package grammer-dtolnay_bin```
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
/// `10`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// 
trait Trait: Sized {
    fn is_reference(self) -> bool;
}
impl<'a, T> Trait for &'a T {
    fn is_reference(self) -> bool {
        true
    }
}
fn main() {
    match 0.is_reference() {
        true => print!("1"),
        false => print!("0"),
    }
    match '?'.is_reference() {
        true => print!("1"),
        false => {
            impl Trait for char {
                fn is_reference(self) -> bool {
                    false
                }
            }
            print!("0")
        }
    }
}

/*
Trait impls anywhere in a program are always in scope, so there is no significance to the impl Trait for char being written inside of a block of code. In particular, that impl is visible throughout the whole program, not just within the block containing the impl.

This question relates to the behavior of trait method auto-ref which is covered in this Stack Overflow answer.

The call to 0.is_reference() observes that there is no implementation of Trait for an integer type that we could call directly. Method resolution inserts an auto-ref, effectively evaluating (&0).is_reference(). This time the call matches impl<'a, T> Trait for &'a T and prints 1.

The call to '?'.is_reference() instead finds impl Trait for char, printing 0.
*/