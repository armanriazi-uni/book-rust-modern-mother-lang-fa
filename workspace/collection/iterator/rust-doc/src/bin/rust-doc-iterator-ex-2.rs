#![allow(dead_code, unused_variables)]

use std::usize;



/// ex-2
///
/// ## Commands
///
/// ```cargo test -q -p rust-doc-iterator_bin --bin rust-doc-iterator-ex-2```
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
fn main(){
let a1 = [1, 2, 3];
let a2 = [4, 5, 6];

let mut iter = a1.iter().zip(a2.iter());

assert_eq!(iter.next(), Some((&1, &4)));
assert_eq!(iter.next(), Some((&2, &5)));
assert_eq!(iter.next(), Some((&3, &6)));
assert_eq!(iter.next(), None);
}
