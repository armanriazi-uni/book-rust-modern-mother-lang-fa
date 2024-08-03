/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-struct-function```
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
/// `1`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// 
struct S {
    f: fn(),
}
impl S {
    fn f(&self) {
        print!("1");
    }
}
fn main() {
    let print2 = || print!("2");
    S { f: print2 }.f();
}
/*
1
A call that looks like .f() always resolves to a method, in this case the inherent method S::f. If there were no method f in scope, a call like this would fail to compile even if a field f exists and contains a function pointer.

To call the function pointer stored in field f, we would need to write parentheses around the field access:

fn main() {
    let print2 = || print!("2");
    (S { f: print2 }.f)();
}
*/