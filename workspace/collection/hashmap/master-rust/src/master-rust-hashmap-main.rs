#![allow(dead_code, unused_variables)]
use std::collections::HashMap;


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-hashmap_bin --bin master-rust-hashmap-main```
///
/// ```cargo doc  --package master-rust-hashmap_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-hashmap_bin```
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
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);
    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }
    fruits.remove("orange");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);
    //println!("\nI now have {} avocados", &fruits.get("avocado").unwrap());
    println!("\nI now have {} avocados", fruits["avocado"]);
}
