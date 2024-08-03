#![allow(dead_code, unused_variables)]
use std::io;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-io-process_bin --bin rust-egg-io-process-main```
///
/// ```cargo doc  --package rust-egg-io-process_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-io-process_bin ```
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
/// // ```rust,compile_fail,ignore




fn main (){
    let mut name= String::new();
    let _ = io::stdin().read_line(&mut name);
    println!("Hello {}", name);
}
