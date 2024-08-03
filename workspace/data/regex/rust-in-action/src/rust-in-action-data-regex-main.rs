#![allow(dead_code, unused_variables)]
use regex::Regex; // <1>
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-data-regex_bin --bin rust-in-action-data-regex-main```
///
/// ```cargo doc  --package rust-in-action-data-regex_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-data-regex_bin ```
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


fn main() {
  let re = Regex::new("picture").unwrap(); // <2>
  let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
  for line in quote.lines() {
    let contains_substring = re.find(line);
    match a { // <3>
        Some(_) => println!("{}", line), // <4>
        None => (), // <5>
    }
  }
}
