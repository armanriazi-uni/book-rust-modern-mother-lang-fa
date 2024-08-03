#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```CRITERION_DEBUG=1 cargo bench --package master_rust_benches_criterion_lib```
///
/// ```cargo doc  --package master_rust_benches_criterion_lib --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master_rust_benches_criterion_lib```
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
/// //```rust,compile_fail,no_run,ignore
///
pub fn slow_fibonacci(nth: usize) -> u64 {
    if nth <= 1 {
        return nth as u64;
    } else {
        return slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2);
    }
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 1..nth {
        c = a + b;
        a = b;
        b = c;
    }
    c
}
