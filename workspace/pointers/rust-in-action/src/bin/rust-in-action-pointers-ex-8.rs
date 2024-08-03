#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-8
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-8```
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
/// r1 is: 5
/// r2 is: 5
/// ```
/// ## Example
/// //``rust,no_run,compile_fail,ignore

fn main() {
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

let address = 0x012345usize;
let r = address as *const i32;
}
