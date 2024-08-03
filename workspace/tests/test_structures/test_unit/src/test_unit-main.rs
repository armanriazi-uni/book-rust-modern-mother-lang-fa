#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p unit_test --bin test_unit-main  -- --test-threads=1  --show-output --ignored --test greeting_contains_name```
///
/// ```cargo doc  --package unit_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package unit_test```
///
/// ## What
/// `Unit Test`
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
/// `Shortens a string to the given length`
///
/// ```rust
/// //If you want to test your library as a client, use an integration test(by use keyword).
/// use shared::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_parse_date() {
        assert_eq!(1,1);
    }
    #[test]
    fn greeting_contains_name() {
        let result = "Carol";
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }


}

fn main() {

    unimplemented!();
}