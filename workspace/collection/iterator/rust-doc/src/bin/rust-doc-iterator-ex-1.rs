#![allow(dead_code, unused_variables)]

use std::usize;



/// ex-1
///
/// ## Commands
///
/// ```cargo test -q -p rust-doc-iterator_bin --bin rust-doc-iterator-ex-1```
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

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .inspect(|v| println!("  iteration is seeing {:?}", v))
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}


fn main(){
    let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .inspect(|v| println!("  iteration is seeing mul a*b: {:?}", v))
    .filter(|x| x % 3 == 0)
    .inspect(|v| println!("  iteration is seeing mod 3: {:?}", v))
    .sum();    
    assert_eq!(18, sum);
}
