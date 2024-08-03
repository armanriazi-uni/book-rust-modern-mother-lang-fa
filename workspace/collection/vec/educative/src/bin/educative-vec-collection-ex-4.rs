#![allow(dead_code, unused_variables)]
use std::fmt::Debug;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-vec-collection_bin --bin educative-vec-collection-ex-4```
///
/// ```cargo doc  --package educative-vec-collection_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-vec-collection_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `tag` - This is the [str] to [find] the ['oo']
///
/// //# Example
///
fn is_reversed<T>(v: &[T]) -> bool
where
    T: Eq + Clone + Debug,
{
    let mut reverse = v.to_vec();
    reverse.reverse();
        println!("{:?}",&reverse);
    &reverse == v
}
fn reversed_vec<T>(v: &[T])
where
    T: Eq + Clone + Debug,
{
    let mut reverse = v.into_iter().rev().collect::<Vec<_>>();
        println!("{:?}",&reverse);
}
fn main() {
    println!("{}", is_reversed(&[1, 2, 3]));
    println!("{}", is_reversed(&[1, 2, 1]));
    println!("{}", is_reversed(&[1, 3,8,11,7]));
    //
    println!("{:?}", reversed_vec(&[1,3,8,11,7]));
}
