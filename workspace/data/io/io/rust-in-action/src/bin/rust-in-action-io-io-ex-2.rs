
#![allow(dead_code, unused_variables)]
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
/// rust-in-action-io-io-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-io-io_bin --bin rust-in-action-io-io-ex-2```
///
/// ```cargo doc  --package rust-in-action-io-io_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-io-io_bin```
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
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// // ```compile_fail,ignore

///fn main() {
///  let mut f = File::open("../../ria-data-io-io.md").unwrap();
///  let mut reader = BufReader::new(f);
///
///  let mut line = String::new();
///  loop {
///    let len = reader.read_line(&mut line).unwrap();
///
///    if len == 0 {
///      break
///    }
///    println!("{} ({} bytes long)", line, len);
///
///    line.truncate(0);
///  }
///
///}
/// ```

fn main(){
    unimplemented!()
}
