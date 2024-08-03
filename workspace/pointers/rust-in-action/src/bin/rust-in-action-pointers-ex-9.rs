#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-9
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-9```
///
/// ## What
/// `Dereference raw pointer`
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
/// 0x2a -> 0x8a
/// ```
/// ## Example
/// //``rust,no_run,compile_fail,ignore
/// You can create pointers safely from any integral value. An i32 is not a Vec<String>, but Rust is quite comfortable ignoring that here.
///
/// ```rust
/// let ptr = 42 as *const Vec<String>;
/// ```
fn main() {
    let ptr = 42 as *const Vec<String>;
    unsafe {
      let new_addr = ptr.offset(4);
     println!("{:p} -> {:p}", ptr, new_addr);
    }
}
