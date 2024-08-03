#![allow(dead_code, unused_variables)]



/// rust-doc-std-file-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin  --bin rust-doc-std-file-ex-5```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin ```
///
/// ## What
/// `Map-Reduce`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing'
/// 
/// ## Example
///
/// //```compile_fail,ignore

use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

fn main() {
    // Stores the iterator of lines of the file in lines variable.
    let lines = read_lines("./hello.txt".to_string());
    // Iterate over the lines of the file, and in this case print them.
    for line in lines {
        println!("{}", line.unwrap());
    }
}