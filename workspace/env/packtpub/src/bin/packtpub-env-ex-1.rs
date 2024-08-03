#![allow(dead_code, unused_variables)]
use rand::prelude::*;                      // <1>


/// packtpub-env-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-env_bin --bin  packtpub-env-ex-1 -- -WRust```
///
/// ## What
/// `Error Handling`
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
/// //```rust,no_run,compile_fail,ignore
use std::env::args;
fn main() {
    for a in args(){
        if let Some(c) = a.chars().next() {
            match c {
                'w'|'W'=> println!("Hello {}", a),
                _=>{}
            }
        }
    }
}
