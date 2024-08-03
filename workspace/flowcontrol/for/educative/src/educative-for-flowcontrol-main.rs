#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p educative-for-flowcontrol_bin --bin educative-for-flowcontrol-main```
///
/// ```cargo doc  --package educative-for-flowcontrol_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-for-flowcontrol_bin```
/// ## What
/// `TODO`
///
/// ## How
/// prime < &10 is possible in for
/// ```
/// if prime < &10 {
///             println!("{}", prime);
///      }
/// ```
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// // ```compile_fail,ignore

fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for prime in &primes {
        if *prime < 10 {
            println!("{}", prime);
        }
    }
}
