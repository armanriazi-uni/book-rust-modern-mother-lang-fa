#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p packtpub-command_bin --bin packtpub-command-main```
///
/// ```cargo doc  --package packtpub-command_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-command_bin```
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
/// // ```compile_fail,ignore
use std::process::Command;

fn main() {
    let mut c = Command::new("ls");
    c.arg("-l");
    let c = c.output()
                .expect("LS Not useable");

    for (n,ln) in String::from_utf8(c.stdout).expect("NOT UTF8able").split("\n").enumerate() {
        println!("Line {}: {}",n,ln);
    }



}
