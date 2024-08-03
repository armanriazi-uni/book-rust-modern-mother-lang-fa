#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p test_dependent --bin test_dependent-main -- --nocapture```
///
/// ```cargo doc  --package test_dependent --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_dependent```
///
/// ## What
/// `Test Dependent`
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
/// `nothing`
///
use std::io;

fn read_last_line(filename: &str) -> Result<String, io::Error> {
    let file_contents = std::fs::read_to_string(filename)?;
    file_contents
        .lines()
        .last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No lines found in input"))
        .map(|s| s.to_string())
}

fn main() -> std::process::ExitCode {
    println!("{}", read_last_line("/home/u2204one/rust/book-rust-modern-mother-lang-to-fa/workspace/tests/test_dependent/test/integration_dep_test.rs").unwrap());
    std::process::ExitCode::from(0)
}
