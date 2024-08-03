#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-io-pipe_bin --bin packtpub-io-pipe-main -- --espeak --cat```
///
/// ```cargo doc  --package packtpub-io-pipe_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-types-radix_bin```
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
/// ## Example
/// //```rust,compile_fail,ignore
use std::process::{Command,Stdio};
use std::io::copy;

fn main() {

    let mut c = Command::new("espeak")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Command didn't run");

    let mut d = Command::new("cat")
                    .arg("/mnt/home/book-rust-modern-mother-lang-to-fa/workspace/data/io/io/pipe/packtpub/foldercopy/tosay.md")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Cat didn't run right");


    copy(&mut d.stdout.unwrap(),&mut c.stdin.unwrap());




}
