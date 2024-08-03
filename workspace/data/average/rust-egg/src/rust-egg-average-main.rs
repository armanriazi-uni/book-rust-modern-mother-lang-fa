#![allow(dead_code, unused_variables)]



/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-egg-average_bin --bin rust-egg-average-main```
///
///
/// ```cargo doc  --package rust-egg-average_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-average_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `nothing`
///
/// ## Example
/// `nothing`
///
/// Type casting in Rust is done via the usage of the `as` operator.
/// Please note that the `as` operator is not only used when type casting.
/// It also helps with renaming imports.
/// I AM NOT DONE
/// The goal is to make sure that the division does not fail to compile
fn average(values: &[f64]) -> f64 {
    let total = values
        .iter()
        .fold(0.0, |a, b| a + b);
        // (1..=num).into_iter().fold(0,|x,y| x+y)
        // (1..=num).product() //for multiply or factorial
    total// or values.len()
}
fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}
