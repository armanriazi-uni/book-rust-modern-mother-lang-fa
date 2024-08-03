#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```CRITERION_DEBUG=1 cargo bench --package criterion_doc_benches_criterion_lib  -- fibonacci```
///
/// ```cargo doc  --package criterion_doc_benches_criterion_lib --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package criterion_doc_benches_criterion_lib```
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
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[inline]
fn fibonacci_optimize
(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
