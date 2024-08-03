#![allow(dead_code, unused_variables)]
use std::collections::HashMap;


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-hashmap_bin --bin rust-in-action-hashmap-main```
///
/// ```cargo doc  --package rust-in-action-hashmap_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-hashmap_bin```
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
    let text = "once upon a time ...";
    let mut word_counts = HashMap::new();
    let pairs = text.split(" ")
                    .map(|x| { (x, 1) });
    for (word, count) in pairs {
        let tmp = word_counts.entry(word)
                             .or_insert(0);
        *tmp += count;
    }
    println!("{:?}", word_counts);
}
