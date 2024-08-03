#![allow(dead_code, unused_variables)]
use std::fs::File;
use std::io::{self, Read};

/// rust-doc-error-handling-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-4```
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


fn read_username_from_file() -> Result<String, io::Error> {
    let f: Result<File, Error> = File::open("./hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn main(){
    read_username_from_file();
}
