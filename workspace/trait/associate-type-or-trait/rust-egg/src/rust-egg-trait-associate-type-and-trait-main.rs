#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p rust-egg-trait-associate-type-and-trait_bin --bin rust-egg-trait-associate-type-and-trait-main```
///
/// ````RUST_BACKTRACE=full cargo test -q -p rust-egg-trait-associate-type-and-trait_bin --bin rust-egg-trait-associate-type-and-trait-main```
///
/// ```cargo egg  --package rust-egg-trait-associate-type-and-trait_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package rust-egg-trait-associate-type-and-trait_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
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

trait AppendBar {
    fn append_bar(self) -> Self;
}
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));        
    }
}
