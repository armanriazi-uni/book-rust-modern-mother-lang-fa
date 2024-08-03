/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-match-underscore```
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
/// `124`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// **
fn check(x: i32) -> bool {
    print!("{}", x);
    false
}
fn main() {
    match (1, 2) {
        (x, _) | (_, x) if check(x) => {
            print!("3")
        }
        _ => print!("4"),
    }
}
/*
This question covers two behaviors of match arms and guards.

First, whether an if guard on a match-arm containing | applies to all alternatives in the match-arm or just to the one it is adjacent to. In the quiz code, does check(x) execute at all for (x, _) or does it only cover the (_, x) case? We would expect 1 would get printed if and only if the former is the case. In fact 1 does get printed. A match-arm gets to have at most one if guard and that guard applies to all the |-separated alternatives in the arm.

But second, this question also covers a kind of "backtracking" behavior of match-arms. After check(x) returns false on (x, _), does the whole match-arm fail to match at that point or does Rust move on to (_, x) and execute the guard a second time? We would expect 2 to be printed if and only if the latter is the case. In fact 2 does get printed; the guard is being run multiple times, once per |-separated alternative in the match-arm.
*/