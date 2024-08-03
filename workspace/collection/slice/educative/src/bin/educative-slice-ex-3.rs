#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-slice_bin --bin educative-slice-ex-3```
///
/// ```cargo doc  --package educative-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-slice_bin```
///
/// ## What
/// `Slice pattern matching`
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
fn print_elements(elements: Vec<&str>) {
  match elements.as_slice(){
    [] => println!("No elements"),
    [element] => println!("One element only: {}", element),
    _ => println!("More than one element present"),
  }
}
print_elements(vec![]);
print_elements(vec!["one"]);
print_elements(vec!["one", "two"]);
}
