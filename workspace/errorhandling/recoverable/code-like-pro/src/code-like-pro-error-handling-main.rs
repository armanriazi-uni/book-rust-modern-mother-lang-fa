#![allow(dead_code, unused_variables)]

use std::io::prelude::*;
use std::{fs::File, io::Read};
use rand::{random};
static mut ERROR: isize = 0;



/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p code-like-pro-error-handling_bin --bin code-like-pro-error-handling-main```
///
/// ```cargo doc  --package code-like-pro-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-error-handling_bin ```
///
/// ## What
/// `error-conversion`
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


#[derive(Debug)]
struct Error(String);

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self(other.to_string())
    }
}

fn read_file(name: &str) -> Result<String, Error> {
    let mut f = File::open(name)?;
    let mut output = String::new();

    f.read_to_string(&mut output)?;

    Ok(output)
}

fn main() {
    println!("{}", read_file("/mnt/home/book-rust-modern-mother-lang-to-fa/workspace/errorhandling/code-like-pro/src/main.rs").unwrap()); // this line succeeds
   // println!("{}", read_file("src/failure.rs").unwrap()); // this line fails
}
