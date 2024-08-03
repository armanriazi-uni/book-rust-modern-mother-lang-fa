#![allow(dead_code, unused_variables)]
use std::io;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-io-buffer_bin --bin rust-in-action-io-buffer-main```
///
/// ```cargo doc  --package rust-in-action-io-buffer_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-io-buffer_bin```
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
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("/mnt/home/book-rust-modern-mother-lang-to-fa/workspace/data/io/io/buffer/rust-in-action/ria-data-io-buffer.md").unwrap();
  let reader = BufReader::new(f);

  for line_ in reader.lines() { // <1>
    let line = line_.unwrap(); // <2>
    println!("{} ({} bytes long)", line, line.len());
  }
}
