#![allow(dead_code, unused_variables)]

/// rust-egg-dangling-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-semantic_bin --bin  rust-egg-semantic-bin-ex-1```
///
/// ## What
/// `Mome Semantic` primitives do not requiring impl Copy trait
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
// move_semantics2.rs
/// Make me compile without changing line 13!
/// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

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
