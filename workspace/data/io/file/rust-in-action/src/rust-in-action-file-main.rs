#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-file_bin --bin rust-in-action-file-main```
///
/// ```cargo doc  --package rust-in-action-file_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-file_bin ```
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
/// ```rust,compile_fail,ignore
/// use std::fs::File;
/// use std::io::BufReader;
/// use std::io::prelude::*;
///fn main() {
///  let f = File::open("../ria-data-file.md").unwrap();
///  let reader = BufReader::new(f);
///
///  for line_ in reader.lines() { // <1>
///    let line = line_.unwrap(); // <2>
///    println!("{} ({} bytes long)", line, line.len());
///  }
///}
/// ```




fn main() {
  unimplemented!()
}
