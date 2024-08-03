#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-io-process_bin --bin other-io-process-main```
///
/// ```cargo doc  --package other-io-process_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-io-process_bin ```
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


use std::error::Error;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn go_wc() -> Result<(), Box<dyn Error>> {
    // Spawn the 'echo' command and capture its output
    let mut echo = Command::new("echo")
       .arg("one two three")
       .stdout(Stdio::piped())
       .spawn()?;

    // Capture the stdout of 'echo' command
    let mut echo_output = echo.stdout.take().expect("Failed to take stdout");

    // Spawn the 'wc' command, piping its stdin from the 'echo' command's stdout
    let mut wc = Command::new("wc")
       .arg("-w") // Count words
       .stdin(Stdio::piped())
       .stdout(Stdio::inherit()) // Inherit stdout so we can see the output
       .spawn()?;

    // Write the output of 'echo' to the stdin of 'wc'
    io::copy(&mut echo_output, &mut wc.stdin.take().expect("Failed to take stdin"))?;

    Ok(())
}

fn main() {
    match go_wc() {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
