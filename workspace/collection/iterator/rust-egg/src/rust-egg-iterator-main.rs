#![allow(dead_code, unused_variables)]

use std::usize;



/// Main
///
/// ## Commands
///
/// ```cargo test -q -p rust-egg-iterator_bin --bin rust-egg-iterator-main```
///
/// ```cargo doc  --package rust-egg-iterator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-iterator_bin```
///
/// ## What
/// `custom_iterator`
///
/// ## How
/// In this module, you'll learn some of unique advantages that iterators can offer
/// Step 1. Complete the `capitalize_first` function to pass the first two cases
/// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it returns a vector of strings as well
/// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
/// As always, there are hints if you execute `rustlings hint iterators2`!
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


pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    // h
    match c.next() {
        None => String::new(),
        Some(first) => first.to_string().to_uppercase() + c.as_str(),
    };
    //e
    match c.next() {
        None => String::new(),
        Some(first) => first.to_string().to_uppercase() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        //assert_ne!(capitalize_first("hello"), "HEllo");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words.iter().map(|w| capitalize_first(w)).collect();// or: .collect::<Vec<String>>();
        assert_ne!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = words
            .iter()
            .map(|w| capitalize_first(w))
            .collect::<String>();
        assert_ne!(capitalized_words, "Hello World");
    }
}


fn main(){
    unimplemented!();
}
