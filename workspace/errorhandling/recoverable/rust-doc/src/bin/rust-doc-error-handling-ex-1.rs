#![allow(dead_code, unused_variables)]

/// rust-doc-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-1```
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
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("./hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
}
fn main() {
    read_username_from_file();
}
