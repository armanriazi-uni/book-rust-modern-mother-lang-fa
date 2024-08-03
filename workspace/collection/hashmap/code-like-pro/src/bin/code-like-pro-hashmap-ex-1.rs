#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-hashmap_bin --bin code-like-pro-hashmap-ex-1```
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
fn main() {
    use metrohash::MetroBuildHasher;
    use std::collections::HashMap;
    let mut map = HashMap::<String, String, MetroBuildHasher>::default();
    map.insert("hello?".into(), "Hello!".into());
    println!("{:?}", map.get("hello?"));
}
