#![allow(dead_code, unused_variables)]

/// rust-egg-dangling-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-semantic_bin --bin  rust-egg-semantic-bin-ex-3```
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
/// move_semantics4.rs
/// Refactor this code so that instead of having `vec0` and creating the vector
/// in `fn main`, we instead create it within `fn fill_vec` and transfer the
/// freshly created vector from fill_vec to its caller.
/// Execute `rustlings hint move_semantics4` for hints!

fn main() {
    let mut vec1 = fill_vec();
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
fn fill_vec() -> Vec<i32> {
    let mut vec = vec![];
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.extend([1, 2, 3].iter());// OR iter.copied()
    vec.extend([11, 22, 33].iter());
    vec
}
