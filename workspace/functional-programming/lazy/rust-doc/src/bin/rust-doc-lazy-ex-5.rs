#![allow(dead_code, unused_variables)]
/// rust-doc-lazy-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-lazy_bin --bin rust-doc-lazy-ex-5```
///
/// ```cargo doc  --package rust-doc-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lazy_bin ```
///
/// ## What
/// `GlobalState`
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
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}
fn main() {
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("The entry for `2` is \"{}\".", HASHMAP.get(&2).unwrap());
}