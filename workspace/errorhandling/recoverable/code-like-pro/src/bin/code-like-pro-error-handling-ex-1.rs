#![allow(dead_code, unused_variables)]
use rand::prelude::*;                      // <1>


/// code-like-pro-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-error-handling_bin --bin  code-like-pro-error-handling-ex-1```
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

#[derive(Debug)]
struct Error {
    message: String,
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self {
            message: other.to_string(),
        }
    }
}

/// For line 6 we need to impl From
fn read_file(name: &str) -> Result<String, Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    println!("{}", read_file("workspace/errorhandling/code-like-pro/src/main.rs").unwrap());
   // println!("{}", read_file("src/not-there.rs").unwrap());
}
