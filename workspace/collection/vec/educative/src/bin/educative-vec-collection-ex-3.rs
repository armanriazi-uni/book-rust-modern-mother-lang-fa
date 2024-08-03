#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-vec-collection_bin --bin educative-vec-collection-ex-3```
///
/// ```cargo doc  --package educative-vec-collection_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-vec-collection_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `tag` - This is the [str] to [find] the ['oo']
///
/// //# Example
///
fn main(){
let students = vec!["Robert", "Jason", "Marie"];
let scores = vec![7, 6, 8];
let pos = students.iter().position(|&x| x == "Jason").unwrap();
println!("Jason's score {}", scores[pos]);
}
