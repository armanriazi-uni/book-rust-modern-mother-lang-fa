#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-vec-collection_bin --bin educative-vec-collection-main```
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
/// # Example
/// `1: Every face, every shop,`
///
/// `2: bedroom window, public-house, and`

/// `3: dark square is a picture`
///
/// `4: feverishly turned--in search of what?`
///
/// `3: dark square is a picture`
///
/// `4: feverishly turned--in search of what?`
///
/// `5: It is the same with books.`
///
/// `6: What do we seek`
///
/// `7: through millions of pages?`
///
///
/// ## Result
///
/// ## Trace
/// [educative-vec-collection-doc-main](../rust_in_action_vec_collection_trace1/index.html)


fn print_elements(elements: Vec<&str>) {
  match elements.as_slice(){
    [] => println!("No elements"),
    [element] => println!("One element only: {}", element),
    [element1, element2] => println!("Two elements\n1: {}\n2: {}", element1, element2),
    _ => println!("More than one element present"),
  }
}
fn main() {
  let empty = vec![];
  let one = vec!["one"];
  let two = vec!["one", "two"];
  let three = vec!["one", "two", "three"];
  print_elements(empty);
  print_elements(one);
  print_elements(three);
  print_elements(two);
}
