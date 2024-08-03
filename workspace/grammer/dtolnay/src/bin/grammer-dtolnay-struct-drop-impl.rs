/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-struct-drop-impl```
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
/// `1234`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// 
struct D(u8);
impl Drop for D {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}
struct S {
    d: D,
    x: u8,
}
fn main() {
    let S { x, .. } = S {
        d: D(1),
        x: 2,
    };
    print!("{}", x);

    let S { ref x, .. } = S {
        d: D(3),
        x: 4,
    };
    print!("{}", x);
}
/*
This question involves drop-placement. Where does D get dropped?

In the first let-binding, we destructure a value of type S into its field x of type u8 as well as .. which represents "the rest of S". The part that is the rest of S is dropped immediately at that point because it no longer has an owner.

In the second let-binding, we borrow a field x from the owner of a value of type S. The whole value of type S remains in scope during the time that its field x is borrowed, and goes out of scope at the close curly brace of main.

The output is 1243.
*/