/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-struct-drop-2```
///
/// ```cargo doc  --package grammer-dtolnay_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package grammer-dtolnay_bin```
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
/// `3121`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// 
struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        print!("1");
    }
}
fn main() {
    let _guard = Guard;
    print!("3");
    let _ = Guard;
    print!("2");
}
/*
That is, the Drop impl for let _guard = Guard runs at the end of main but the Drop impl for let _ = Guard runs right away.

In general, a value is dropped when it no longer has an owner. The variable _guard owns the first value of type Guard and remains in scope until the end of main. The _ is not a variable but a wildcard pattern that binds nothing; since no variables are bound on this line, there is no variable to be the owner of the second value of type Guard and that value is dropped on the same line.

This distinction between the underscore pattern vs variables with a leading underscore is incredibly important to remember when working with lock guards in unsafe code.

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref MUTEX: Mutex<()> = Mutex::new(());
}

/// MUTEX must be held when accessing this value.
static mut VALUE: usize = 0;

fn main() {
    let _guard = MUTEX.lock().unwrap();
    unsafe {
        VALUE += 1;
    }
}
If this code were to use let _ = MUTEX.lock().unwrap() then the mutex guard would be dropped immediately, releasing the mutex and failing to guard the access of VALUE.
*/