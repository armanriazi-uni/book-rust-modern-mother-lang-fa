#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-3```
///
/// ## What
/// `Memory Scan 3`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Segmentation fault (core dumped)`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
///
fn main() {
     unsafe { std::ptr::null_mut::<i32>().write(42) };
     //std::ptr::write(uc_addr as *mut usize, 0usize); //Unsafe OverWrite 0 to addr
}
