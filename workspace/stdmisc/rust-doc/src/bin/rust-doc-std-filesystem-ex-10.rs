#![allow(dead_code, unused_variables)]



/// rust-doc-std-filesystem-ex-10
/// cli with clap
/// https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
/// 
/// ## Commands
///
/// ```v```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin ```
///
/// ## What
/// `Arg command line`
///
/// ## How
/// The std::fs module contains several functions that deal with the filesystem.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
///My path is target/debug/playground.
///I got 0 arguments: [].
///$ ./args 1 2 3
///
///My path is ./args.
///I got 3 arguments: ["1", "2", "3"].
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}