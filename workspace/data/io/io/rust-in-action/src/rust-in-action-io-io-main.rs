#![allow(dead_code, unused_variables)]
use std::io;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-io-io_bin --bin rust-in-action-io-io-main```
///
/// ```cargo doc  --package rust-in-action-io-io_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-io-io_bin ```
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

    io::stdin().read_line(&mut name);

    println!("Hello {}", name);

}
