#![allow(dead_code, unused_variables)]



/// rust-egg-iterator-ex-2
///
/// ## Commands
///
/// ```cargo test -q -p rust-egg-iterator_bin --bin rust-egg-iterator-ex-2```
///
/// ```cargo doc  --package rust-egg-iterator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-iterator_bin```
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
// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*
    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

fn main(){
    unimplemented!();
}
