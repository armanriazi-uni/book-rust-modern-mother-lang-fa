#![allow(dead_code, unused_variables)]



/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-convert-trait-as_bin --bin rust-doc-convert-trait-as-main```
///
///
/// ```cargo doc  --package rust-doc-convert-trait-as_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-convert-as_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`
///
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
fn main() {
   println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
