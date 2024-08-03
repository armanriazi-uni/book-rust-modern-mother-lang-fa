#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-convert-as_ref_bin --bin rust-egg-convert-as_ref-main```
///
/// ```cargo test -q -p rust-egg-convert-as_ref_bin --bin rust-egg-convert-as_ref-main```
///
/// ## What
// `TODO`
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
/// ```rust,compile_fail,ignore
/// AsRef and AsMut allow for cheap reference-to-reference conversions.
/// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
/// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

/// I AM NOT DONE
/// Obtain the number of bytes (not characters) in the given argument
/// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
    {

    arg.as_ref().as_bytes().len()
}
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }
    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }
}
fn main() {
    let s = "Café au lait";
    println!("{}", char_counter(s));
    println!("{}", byte_counter(s));
}