#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-closure_bin --bin master-rust-closure-main```
///
/// ```cargo doc  --package master-rust-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-closure_bin ```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
///

fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    println!("{} doubled is {}", value, twice);

    let big_closure = |b, c| {
        let z = b + c;
        z * twice
    };
    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);
}
