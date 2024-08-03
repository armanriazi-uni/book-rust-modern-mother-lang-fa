#![allow(dead_code, unused_variables)]
use rand::{random};
static mut ERROR: isize = 0;

struct File;

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p rust-in-action-error-handling_bin --bin rust-in-action-error-handling-main```
///
/// ```cargo doc  --package rust-in-action-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-error-handling_bin ```
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
///

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() { 
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
        else{
            panic!("No error!")
        }
    }
}
