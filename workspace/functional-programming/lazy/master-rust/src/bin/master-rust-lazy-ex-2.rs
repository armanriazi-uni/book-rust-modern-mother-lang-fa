#![allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use std::collections::HashMap;
/// master-rust-lazy-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-lazy_bin --bin master-rust-lazy-ex-2```
///
/// ```cargo doc  --package master-rust-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-lazy_bin ```
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


lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
   static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("The entry for `0` is \"{}\".",  HASHMAP.get(&3).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
