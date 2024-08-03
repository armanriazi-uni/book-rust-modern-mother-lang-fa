#![allow(dead_code, unused_variables)]
// use error_chain::error_chain;
// use serde::Deserialize;
// use std::fmt;


/// rust-doc-error-handling-ex-8
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-8```
///
/// ## What
/// `Propagating errors`
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


use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
fn read_file_content<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() -> io::Result<()> {
    let file_path = "./example.txt";
    let content = read_file_content(file_path)?;
    println!("{}", content);
    Ok(())
}
