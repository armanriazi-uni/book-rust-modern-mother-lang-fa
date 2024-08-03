#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-semantic_bin --bin rust-egg-semantic-main```
///
/// ```cargo doc  --package rust-egg-semantic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-semantic_bin```
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
/// ``
///
/// ## Example
/// move_semantics1.rs
/// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
