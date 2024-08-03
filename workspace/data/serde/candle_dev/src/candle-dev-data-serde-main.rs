#![allow(dead_code, unused_variables)]
use serde::{Deserialize, Serialize};



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p candle-dev-data-serde_bin --bin candle-dev-data-serde-main```
///
/// ```cargo doc  --package candle-dev-data-serde_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package candle-dev-data-serde_bin ```
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



#[derive(Serialize, Deserialize, Debug)]
struct Author {
    first: String,
    last: String,
}
fn main() {
    let mark_twain = Author {
        first: "Samuel".to_owned(),
        last: "Clemens".to_owned(),
    };
    let serialized_json = serde_json::to_string(&mark_twain).unwrap();
    println!("Serialized as JSON: {}", serialized_json);
    let serialized_mp = rmp_serde::to_vec(&mark_twain).unwrap();
    println!("Serialized as MessagePack: {:?}", serialized_mp);

    let deserialized_json: Author = serde_json::from_str(&serialized_json).unwrap();
    println!("Deserialized from JSON: {:?}", deserialized_json);
    let deserialized_mp: Author = rmp_serde::from_read_ref(&serialized_mp).unwrap();
    println!("Deserialized from MessagePack: {:?}", deserialized_mp);
}
