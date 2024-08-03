#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-7
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-7```
///
/// ## What
/// `Transmut Raw Pointer to usize`
///
/// ## How
/// Interprets *const i64 as usize.
/// Using transmute() is highly unsafe
/// but is used here to postpone
/// introducing more syntax.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```
/// a: 42 (0x7ffe1b271790...0x7ffe1b271797)
/// ```
/// ## Example
/// ```rust,no_run
/// //Casts a reference to the variable a (&a) to a constant raw pointer i64 (*const i64)
///  let a_ptr = &a as *const i64;
/// ```rust,no_run,compile_fail,ignore
/// println!("a: {} ({:p}...0x{:x})", a, a_ptr+7, a_addr);
/// ```
fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe {
    std::mem::transmute(a_ptr)
    //std::mem::swap(&mut a, &mut b);
    };
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr +7);
    println!("{:032b}", a_addr+7);
}
