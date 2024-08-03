#![allow(dead_code, unused_variables)]



/// rust-doc-std-file-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin  --bin rust-doc-std-file-ex-3```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin ```
///
/// ## What
/// `File I/O`
///
/// ## How
/// The open function can be used to open a file in read-only mode.
/// A File owns a resource, the file descriptor and takes care of closing the file when it is droped.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();
    println!("path is {:?}", &display);
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}