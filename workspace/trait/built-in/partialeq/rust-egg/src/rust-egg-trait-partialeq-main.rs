#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// rust-egg-trait-partialeq-main
///
/// ## Commands
///
/// ```cargo test -q -p rust-egg-trait-partialeq_bin --bin  rust-egg-trait-partialeq-main```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
struct Book {
    isbn: i32,
    format: BookFormat,
}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){
        let b1 = Book { isbn: 3, format: BookFormat::Paperback };
        let b2 = Book { isbn: 3, format: BookFormat::Ebook };
        let b3 = Book { isbn: 10, format: BookFormat::Paperback };
        assert!(b1 == b2);
        assert!(b1 != b3);
    }
}
