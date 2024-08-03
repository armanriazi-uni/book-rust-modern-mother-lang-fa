#![allow(dead_code, unused_variables)]



/// rust-doc-std-filesystem-arg-ex-11
/// cli with clap
/// https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
/// 
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin  --bin rust-doc-std-filesystem-arg-ex-11 -- increase 42```
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
/// $ ./match_args Rust
/// This is not the answer.
/// $ ./match_args 42
/// This is the answer!
/// $ ./match_args do something
/// error: second argument not an integer
/// usage:
/// match_args <string>
///     Check whether given string is the answer.
/// match_args {increase|decrease} <integer>
///     Increase or decrease given integer by one.
/// $ ./match_args do 42
/// error: invalid command
/// usage:
/// match_args <string>
///     Check whether given string is the answer.
/// match_args {increase|decrease} <integer>
///     Increase or decrease given integer by one.
/// $ ./match_args increase 42
/// 43
/// //```compile_fail,ignore

use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}
fn decrease(number: i32) {
    println!("{}", number - 1);
}
fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        _ => {
            help();
        }
    }
}