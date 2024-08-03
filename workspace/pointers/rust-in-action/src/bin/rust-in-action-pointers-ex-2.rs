#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-2```
///
/// ## What
/// `Memory Scan 2`
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
    let mut n_nonzero:u8 = 0;

    for i in 1..10 {    // <1>
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
           
        }
 
    }
//println!("non-zero bytes in memory: {:?}", &n_nonzero);
}
