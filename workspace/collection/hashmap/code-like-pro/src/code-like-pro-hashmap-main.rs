#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-hashmap_bin --bin code-like-pro-hashmap-main```
///
/// ```cargo doc  --package code-like-pro-hashmap_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-hashmap_bin ```
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
/// // rust,compile_fail,no_run,ignore

use std::collections::HashMap;
#[derive(Hash, Eq, PartialEq, Debug)]
struct CompoundKey {
    name: String,
    value: i32,
}
fn main() {
    let mut map = HashMap::<CompoundKey, String>::new();
    let key = CompoundKey {
        name: "key 1".into(),
        value: 1,
    };
    map.insert(key, "some value".into());
    println!("{:#?}", map);
}
