#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-data-regex_bin --bin rust-in-action-data-regex-ex-2```
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
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u8,
}
fn main() {
    // Example JSON string
    let json_str = r#"
        [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]
    "#;
    let people: Vec<Person> = serde_json::from_str(json_str).unwrap();
    let re = Regex::new(r"^A").unwrap();
    let filtered_people: Vec<&Person> = people.iter().filter(|person| re.is_match(&person.name)).collect();
    println!("People whose names start with 'A':");
    for person in filtered_people {
        println!("{:?}", person);
    }
    let re = Regex::new(r"\b\d+\b").unwrap();
    let modified_json_str = re.replace_all(json_str, "REPLACED");
    println!("Modified JSON:");
    println!("{}", modified_json_str);
}
