/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-map-mod```
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
/// `112031`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// **
fn main() {
    let input = vec![1, 2, 3];
    let parity = input
        .iter()
        .map(|x| {
            print!("{}", x);
            x % 2
        });

    for p in parity {
        print!("{}", p);
    }
}

/*
As described in the documentation of the Iterator::map method, the map operation is performed lazily. The closure provided as an argument to map is only invoked as values are consumed from the resulting iterator. The closure is not applied eagerly to the entire input stream up front.

In this code, the for loop is what drives the iteration. For each element consumed from the parity iterator, our closure needs to be evaluated one time. Thus the output will alternate between numbers printed by the closure and numbers printed by the loop body.
*/