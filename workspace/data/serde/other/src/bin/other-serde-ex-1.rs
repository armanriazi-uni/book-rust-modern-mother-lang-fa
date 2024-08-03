/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-serde_bin --bin other-serde-ex-1```
///
/// ```cargo doc  --package other-serde_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-serde_bin ```
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
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    hobbies: Vec<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "name": "John Doe",
        "age": 30,
        "hobbies": ["reading", "gaming"]
    }"#;
    let person: Person = serde_json::from_str(json)?;
    println!("{:?}", person);
    let person = Person {
        name: "Jane Doe".to_string(),
        age: 25,
        hobbies: vec!["painting".to_string(), "traveling".to_string()],
    };
    let serialized_person = serde_json::to_string(&person)?;
    println!("{}", serialized_person);
    Ok(())
}
