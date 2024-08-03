#![allow(dead_code, unused_variables)]

/// rust-in-action-vec-collection-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-vec-collection_bin --bin  rust-in-action-vec-collection-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore

fn main() {
    let input = ["once  upon a time", "there  was an example"];
    let mut input_iter = input.iter();
    let mut words = vec![];
    for _ in 0..input.len() {
        words.push(
            input_iter
                .next()
                .as_mut()
                .unwrap()
                .split(' ')
                .filter(|x| *x != "")
                .collect::<Vec<_>>(),
        );
    }
    println!("{:?}", words);
}
