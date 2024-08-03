#![allow(dead_code, unused_variables)]

use std::usize;



/// Main
///
/// ## Commands
///
/// ```cargo test -q -p rust-doc-iterator_bin --bin rust-doc-iterator-main```
///
/// ```cargo doc  --package rust-doc-iterator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-iterator_bin```
///
/// ## What
/// `custom_iterator`
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
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}


fn main(){
    unimplemented!();
}
