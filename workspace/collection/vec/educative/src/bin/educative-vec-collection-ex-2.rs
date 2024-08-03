#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-vec-collection_bin --bin educative-vec-collection-ex-2```
///
/// ```cargo doc  --package educative-vec-collection_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-vec-collection_bin```
///
/// ## What
/// We know that vectors can have only one type, Vec<T>.
/// However, now that we know some of Rust’s generics, we can overcome some of the type-system limitations.
/// Let’s look at the following code:
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
#[derive(Debug)]
enum Cell {
    Int(i64),
    Float(f64),
    Text(String),
}

let row = vec![
    Cell::Text("Hello, world!".into()), Cell::Int(789), Cell::Float(78.9),
];
println!("{:?}", row);
}
