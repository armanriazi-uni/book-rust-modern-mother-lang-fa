#![allow(dead_code, unused_variables)]



/// rust-egg-trait-associate-type-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-trait-associate-type-and-trait_bin  --bin rust-egg-trait-associate-type-ex-1```
///
/// ```cargo test -q -p rust-egg-trait-associate-type-and-trait_bin  --bin rust-egg-trait-associate-type-ex-1```
///
/// ## What
/// `TODO`
///
/// ## How
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `te-type-ex-2`
/// `s: FooBar`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore

trait AppendBar {
    fn append_bar(self) -> Self;
}
impl AppendBar for String {
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}
fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }
    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
