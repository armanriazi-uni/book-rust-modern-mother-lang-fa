#![allow(dead_code, unused_variables)]
use rand::prelude::*;                      // <1>


/// master-rust-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-error-handling_bin --bin  master-rust-error-handling-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
/// We can use underscores to ask Rust to infer types for us in obvious cases.
fn main() {
    let _my_result: Result<_, ()> = Ok(64);    
    let _my_result = Ok::<_, ()>(64);
    let _my_err = Err::<(), f32>(345.3);
    let _other_err: Result<bool, String> = Err("Wait, what ?".to_string());
}
