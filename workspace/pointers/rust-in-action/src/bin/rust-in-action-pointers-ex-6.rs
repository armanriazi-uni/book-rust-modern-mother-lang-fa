#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-6```
///
/// ## What
/// `Cast usual pointer to Raw Pointer`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```
/// a: 42 (0x7ffe27e65548)
/// ```
/// ## Example
/// //``rust,no_run,compile_fail,ignore
///
/// related to : [rust-in-action-pointers-ex-7](rust-in-action-pointers-ex-7)
/// Casts a reference to the variable a (&a) to a constant raw pointer i64 (*const i64)
fn main() {
    let a: i64 = 42;

    let a_ptr = &a as *const i64;
    println!("a: {} ({:p})", a, a_ptr);
}
