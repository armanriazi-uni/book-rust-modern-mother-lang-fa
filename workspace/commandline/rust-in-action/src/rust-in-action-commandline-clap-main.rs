#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
/// USAGE:
/// ```grep-lite <pattern>```
///
/// ```RUST_BACKTRACE=1 cargo run -q -p rust-in-action-commandline-clap_bin --bin rust-in-action-commandline-clap-main -- pattern -picture```
///
/// ```cargo run --picture```
/// 
/// ```grep-lite --help```
///
/// ```cargo doc  --package rust-in-action-commandline-clap_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-commandline-clap_bin ```
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
///  `TODO`
///
///

use regex::Regex;
use clap::{App,Arg};

fn main() {
let args = App::new("grep-lite")
                    .version("0.1")
                    .about("searches for patterns")
                    .arg(Arg::with_name("pattern")
                    .help("The pattern to search for")
                    .takes_value(true)
                    .required(false))
                    .get_matches();

 let pattern = args.value_of("pattern").unwrap();
 let re = Regex::new(pattern).unwrap();

 let quote = "Every face, every shop, bedroom window, public-house, and
 dark square is a picture feverishly turned--in search of what?
 It is the same with books. What do we seek through millions of pages?";

 for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
}
}
