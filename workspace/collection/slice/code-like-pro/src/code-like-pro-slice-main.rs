#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-slice_bin --bin code-like-pro-slice-main```
///
/// ```cargo doc  --package code-like-pro-slice_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-slice_bin```
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
/// //rust,compile_fail,no_run,ignore
fn main() {
    let array = [0u8; 64];
    let slice: &[u8] = &array;
    let (first_half, second_half) = slice.split_at(32);
    println!(
        "first_half.len()={} second_half.len()={}",
        first_half.len(),
        second_half.len()
    );
    let wordlist = "one,two,three,four";
    for word in wordlist.split(',') {
        println!("word={}", word);
    }
}
